use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MasterStatusSnapshot {
    pub running: bool,
    pub calibration_mode: bool,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub master_width: i32,
    pub master_height: i32,
    pub remote_mode: bool,
}

#[derive(Debug, Clone)]
pub struct MasterStatus {
    pub running: bool,
    pub calibration_mode: bool,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub master_width: i32,
    pub master_height: i32,
    pub remote_mode: bool,
}

impl MasterStatus {
    pub fn reset(&mut self) {
        self.running = true;
        self.calibration_mode = true;
        self.remote_mode = false;
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.master_width = 1920;
        self.master_height = 1080;
    }
}

impl Default for MasterStatus {
    fn default() -> Self {
        Self {
            running: false,
            calibration_mode: true,
            cursor_x: 0,
            cursor_y: 0,
            master_width: 1920,
            master_height: 1080,
            remote_mode: false,
        }
    }
}

impl From<&MasterStatus> for MasterStatusSnapshot {
    fn from(status: &MasterStatus) -> Self {
        Self {
            running: status.running,
            calibration_mode: status.calibration_mode,
            cursor_x: status.cursor_x,
            cursor_y: status.cursor_y,
            master_width: status.master_width,
            master_height: status.master_height,
            remote_mode: status.remote_mode,
        }
    }
}
