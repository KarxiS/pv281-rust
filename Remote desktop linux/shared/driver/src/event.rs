use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct MouseMove {
    pub x: i32,
    pub y: i32,
    pub wheel: i32,
}

impl std::ops::Add for MouseMove {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            wheel: self.wheel + rhs.wheel,
        }
    }
}

impl std::iter::Sum for MouseMove {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::default();
        for mm in iter {
            res += mm;
        }

        res
    }
}

impl std::ops::AddAssign for MouseMove {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.wheel += rhs.wheel;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct MouseClick {
    pub button: MouseButton,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct KeyboardPress {
    pub key: u16,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum DriverEvent {
    MouseMove(MouseMove),
    MouseClick(MouseClick),
    KeyboardPress(KeyboardPress),
}

impl DriverEvent {
    pub const fn mouse_move(x: i32, y: i32, wheel: i32) -> Self {
        Self::MouseMove(MouseMove { x, y, wheel })
    }

    pub const fn mouse_click(button: MouseButton, pressed: bool) -> Self {
        Self::MouseClick(MouseClick { button, pressed })
    }

    pub const fn keyboard_press(key: u16, pressed: bool) -> Self {
        Self::KeyboardPress(KeyboardPress { key, pressed })
    }
}
