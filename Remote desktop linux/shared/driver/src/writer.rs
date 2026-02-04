use std::io::Result;

use super::event::*;

use evdev::{
    uinput::VirtualDevice, AttributeSet, KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent,
};

pub struct DriverWriter {
    device: VirtualDevice,
}

impl DriverWriter {
    pub fn new(keys: Vec<KeyCode>, axes: Vec<RelativeAxisCode>) -> Result<Self> {
        // name format "fake-kmr-<timestamp:mili>"
        let keys = keys.iter().collect::<AttributeSet<KeyCode>>();
        let axes = axes.iter().collect::<AttributeSet<RelativeAxisCode>>();

        let device = VirtualDevice::builder()?
            .name("fake-kmdr-")
            .with_keys(&keys)?
            .with_relative_axes(&axes)?
            .build()?;

        Ok(Self { device })
    }

    fn simulate_mouse_move(&mut self, mouse_move: &MouseMove) -> Result<()> {
        let x = *RelativeAxisEvent::new(RelativeAxisCode::REL_X, mouse_move.x);
        let y = *RelativeAxisEvent::new(RelativeAxisCode::REL_Y, mouse_move.y);
        let wheel = *RelativeAxisEvent::new(RelativeAxisCode::REL_WHEEL, mouse_move.wheel);
        self.device.emit(&[x, y, wheel])?;

        Ok(())
    }

    fn simulate_mouse_click(&mut self, click: &MouseClick) -> Result<()> {
        let button = match click.button {
            MouseButton::Left => KeyCode::BTN_LEFT,
            MouseButton::Right => KeyCode::BTN_RIGHT,
            MouseButton::Middle => KeyCode::BTN_MIDDLE,
        };
        let click = *KeyEvent::new(button, i32::from(click.pressed));

        self.device.emit(&[click])?;

        Ok(())
    }

    fn simulate_key_press(&mut self, press: &KeyboardPress) -> Result<()> {
        let press = *KeyEvent::new(KeyCode(press.key), i32::from(press.pressed));

        self.device.emit(&[press])?;

        Ok(())
    }

    pub fn simulate_event(&mut self, event: DriverEvent) -> Result<()> {
        match event {
            DriverEvent::MouseMove(mouse_move) => self.simulate_mouse_move(&mouse_move),
            DriverEvent::MouseClick(click) => self.simulate_mouse_click(&click),
            DriverEvent::KeyboardPress(press) => self.simulate_key_press(&press),
        }
    }

    pub fn block_inputs(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn unblock_inputs(&self) -> Result<()> {
        unimplemented!()
    }
}
