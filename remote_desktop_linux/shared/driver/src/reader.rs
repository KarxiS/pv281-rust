use evdev::uinput::VirtualDevice;
use std::io::{ErrorKind, Result};
use std::path::PathBuf;

use crate::device_type::DeviceType;
use crate::event::{DriverEvent, MouseButton, MouseMove};
use evdev::{Device, EventSummary, KeyCode, RelativeAxisCode};

use super::stream::ToStream;

pub struct DriverReader<S>
where
    S: ToStream + std::marker::Send,
{
    devices: Vec<S>,
}

pub type DeviceReader = DriverReader<Device>;
pub type VirtualDevicerReader = DriverReader<VirtualDevice>;

impl DriverReader<Device> {
    pub fn grab_inputs(&mut self) -> Result<()> {
        for d in &mut self.devices {
            d.grab()?;
        }
        Ok(())
    }

    pub fn ungrab_inputs(&mut self) -> Result<()> {
        for d in &mut self.devices {
            d.ungrab()?;
        }
        Ok(())
    }

    #[must_use]
    pub fn available_axes(&self) -> Option<Vec<RelativeAxisCode>> {
        let mut result = Vec::new();
        for d in &self.devices {
            result.extend(d.supported_relative_axes()?.iter());
        }
        Some(result)
    }

    #[must_use]
    pub fn available_keys(&self) -> Option<Vec<KeyCode>> {
        let mut result = Vec::new();
        for d in &self.devices {
            result.extend(d.supported_keys()?.iter());
        }

        Some(result)
    }

    pub fn list_devices() -> Vec<(PathBuf, String, DeviceType)> {
        let mut devices = Vec::new();

        for (path, dev) in evdev::enumerate() {
            let mut name = String::new();

            if let Some(n) = dev.name() {
                name = n.to_string();
            }

            let mut t = DeviceType::Mouse;

            let events = dev.supported_events();
            if !events.contains(evdev::EventType::RELATIVE) {
                t = DeviceType::Keyboard;
            }
            if !events.contains(evdev::EventType::KEY) {
                t = DeviceType::Other;
            }

            devices.push((path, name, t));
        }

        devices
    }
}

impl<S> DriverReader<S>
where
    S: ToStream + std::marker::Send + std::marker::Sync,
{
    pub fn open_path(device: PathBuf, grab: bool, non_block: bool) -> Result<Device> {
        let mut device = Device::open(device)?;

        if grab {
            device.grab()?;
        }

        if non_block {
            device.set_nonblocking(non_block)?;
        }

        Ok(device)
    }

    pub const fn new(devices: Vec<S>) -> Result<Self> {
        Ok(Self { devices })
    }

    fn match_mouse_move(code: RelativeAxisCode, val: i32) -> MouseMove {
        //println!("DEBUG: code: {:?} s value: {}", code, val); debug for future
        match code {
            RelativeAxisCode::REL_X => MouseMove {
                x: val,
                y: 0,
                wheel: 0,
            },
            RelativeAxisCode::REL_Y => MouseMove {
                x: 0,
                y: val,
                wheel: 0,
            },
            RelativeAxisCode::REL_WHEEL => MouseMove {
                wheel: val,
                ..Default::default()
            },
            _ => MouseMove::default(), //doesnt crash, we just ignore and create blank message
        }
    }

    const fn match_button(code: KeyCode) -> Option<MouseButton> {
        match code {
            KeyCode::BTN_LEFT => Some(MouseButton::Left),
            KeyCode::BTN_RIGHT => Some(MouseButton::Right),
            KeyCode::BTN_MIDDLE => Some(MouseButton::Middle),
            _ => None,
        }
    }

    const fn match_key(code: KeyCode, val: i32) -> DriverEvent {
        let pressed = val != 0;
        if let Some(button) = Self::match_button(code) {
            return DriverEvent::mouse_click(button, pressed);
        }
        DriverEvent::keyboard_press(code.0, pressed)
    }

    fn fetch_events(device: &mut S) -> Result<Vec<DriverEvent>> {
        let events = match device.get_events() {
            Ok(events) => events,
            Err(e) if e.kind() == ErrorKind::WouldBlock => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        Ok(events
            .iter()
            .filter_map(|event| match event.destructure() {
                EventSummary::RelativeAxis(_, code, val) => {
                    Some(DriverEvent::MouseMove(Self::match_mouse_move(code, val)))
                }
                EventSummary::Key(_, code, val) => Some(Self::match_key(code, val)),
                EventSummary::Repeat(_, _code, _val) => {
                    // todo?
                    None
                }
                _ => None,
            })
            .collect())
    }

    pub fn read_events(&mut self) -> Result<Vec<DriverEvent>> {
        let mut result = Vec::new();
        for d in &mut self.devices {
            result.extend(Self::fetch_events(d)?);
        }

        Ok(result)
    }
}

impl<S> Drop for DriverReader<S>
where
    S: ToStream + std::marker::Send,
{
    ///Added trait Drop, if the DriverReader gets closed (panic, close program) rust will call drop() - it will ungrab the program and will not lock you out
    fn drop(&mut self) {
        let _ = self.devices.iter_mut().map(S::ungrab_device);
        println!("DriverReader destroyed, ungrabbing inputs .. ");
    }
}
