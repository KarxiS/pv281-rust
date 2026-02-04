use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use kmf_driver::driver::{DeviceReader, DriverEvent, DriverWriter, MouseMove};
use kmf_driver::event::MouseButton;
use kmf_middleware::command::{send_action, GenericAction};
use kmf_protocol::config::ServerMessage;

use crate::status::MasterStatus;

const FAILSAFE_KEY_CTRL: u16 = 29; // KEY_LEFTCTRL
const FAILSAFE_KEY_ALT: u16 = 56; // KEY_LEFTALT
const FAILSAFE_KEY_Q: u16 = 16; // KEY_Q

pub struct DriverLoopContext {
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub remote_mode: bool,
    pub calibration_mode: bool,
    pub master_width: i32,
    pub master_height: i32,
    pub inputs_grabbed: bool,
    pub pressed_keys: HashSet<u16>,
    pub writer: DriverWriter,
    pub tx: broadcast::Sender<ServerMessage>,
    pub status_mutex: Arc<Mutex<MasterStatus>>,
    pub running_flag: Arc<AtomicBool>,
}

impl Default for DriverLoopContext {
    fn default() -> Self {
        panic!("DriverLoopContext cannot be default-constructed without arguments");
    }
}

impl DriverLoopContext {
    pub fn new(
        writer: DriverWriter,
        tx: broadcast::Sender<ServerMessage>,
        status_mutex: Arc<Mutex<MasterStatus>>,
        running_flag: Arc<AtomicBool>,
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            remote_mode: false,
            calibration_mode: true,
            master_width: width,
            master_height: height,
            inputs_grabbed: false,
            pressed_keys: HashSet::new(),
            writer,
            tx,
            status_mutex,
            running_flag,
        }
    }

    /// Processes a single event. Returns `false` if the loop should terminate.
    pub fn handle_event(&mut self, event: DriverEvent, reader: &mut DeviceReader) -> bool {
        // Track pressed keys for Failsafe
        if let DriverEvent::KeyboardPress(kp) = &event {
            if kp.pressed {
                self.pressed_keys.insert(kp.key);
            } else {
                self.pressed_keys.remove(&kp.key);
            }

            // Failsafe: Ctrl+Alt+Q
            if kp.pressed
                && self.pressed_keys.contains(&FAILSAFE_KEY_CTRL)
                && self.pressed_keys.contains(&FAILSAFE_KEY_ALT)
                && kp.key == FAILSAFE_KEY_Q
            {
                println!("[FAILSAFE] Triggered. Stopping master.");
                self.running_flag.store(false, Ordering::SeqCst);
                return false;
            }
        }

        match event {
            DriverEvent::MouseMove(mm) => self.handle_mouse_move(mm, reader),
            DriverEvent::MouseClick(mc) => self.handle_mouse_click(mc),
            DriverEvent::KeyboardPress(kp) => self.handle_key_press(kp),
        }
        true
    }

    fn handle_mouse_move(&mut self, mm: MouseMove, reader: &mut DeviceReader) {
        if self.calibration_mode {
            self.cursor_x = (self.cursor_x + mm.x).max(0);
            self.cursor_y = (self.cursor_y + mm.y).max(0);
            self.update_status();
            return;
        }

        let total_width = self.master_width * 2;
        self.cursor_x = (self.cursor_x + mm.x).clamp(0, total_width - 1);
        self.cursor_y = (self.cursor_y + mm.y).clamp(0, self.master_height - 1);

        // Determine if we crossed the boundary to the slave
        let new_remote_mode = self.cursor_x >= self.master_width;

        if new_remote_mode != self.remote_mode {
            self.switch_mode(new_remote_mode, reader);
        }

        self.update_status();

        if self.remote_mode {
            let action = GenericAction::MouseMove {
                x: mm.x,
                y: mm.y,
                wheel: mm.wheel,
            };
            self.send_network_action(action);
        } else if self.inputs_grabbed {
            // Replay event locally only if we have grabbed inputs
            let _ = self.writer.simulate_event(DriverEvent::MouseMove(mm));
        }
    }

    fn handle_mouse_click(&mut self, mc: kmf_driver::event::MouseClick) {
        let action = GenericAction::MouseClick {
            button: match mc.button {
                MouseButton::Left => "left".to_string(),
                MouseButton::Right => "right".to_string(),
                MouseButton::Middle => "middle".to_string(),
            },
            pressed: mc.pressed,
        };

        if self.remote_mode {
            self.send_network_action(action);
        } else if self.inputs_grabbed {
            let _ = self.writer.simulate_event(DriverEvent::MouseClick(mc));
        }
    }

    fn handle_key_press(&mut self, kp: kmf_driver::event::KeyboardPress) {
        // Calibration confirmation
        if self.calibration_mode && kp.key == 46 && kp.pressed {
            // 'c' key
            self.master_width = (self.cursor_x + 1).max(1);
            self.master_height = (self.cursor_y + 1).max(1);
            self.calibration_mode = false;
            self.remote_mode = false;
            println!(
                "[CAL] Calibrated: {}x{}",
                self.master_width, self.master_height
            );
            self.update_status();
            return;
        }

        let action = GenericAction::KeyPress {
            key: kp.key.to_string(),
            pressed: kp.pressed,
        };

        if self.remote_mode {
            self.send_network_action(action);
        } else if self.inputs_grabbed {
            let _ = self.writer.simulate_event(DriverEvent::KeyboardPress(kp));
        }
    }

    fn switch_mode(&mut self, new_remote: bool, reader: &mut DeviceReader) {
        self.remote_mode = new_remote;
        if self.remote_mode {
            // Entering remote mode: grab inputs to prevent local OS from receiving them
            if !self.inputs_grabbed {
                if let Err(e) = reader.grab_inputs() {
                    eprintln!("Failed to grab inputs: {}", e);
                } else {
                    self.inputs_grabbed = true;
                }
            }
        } else {
            // Entering local mode: ungrab inputs
            if self.inputs_grabbed {
                if let Err(e) = reader.ungrab_inputs() {
                    eprintln!("Failed to ungrab inputs: {}", e);
                } else {
                    self.inputs_grabbed = false;
                }
            }
        }
        println!(
            "[MODE] Switched to {}",
            if self.remote_mode { "REMOTE" } else { "LOCAL" }
        );
    }

    fn send_network_action(&self, action: GenericAction) {
        if let Some(ServerMessage::Action(value)) = send_action(&action) {
            let _ = self.tx.send(ServerMessage::Action(value));
        }
    }

    fn update_status(&self) {
        let mut status = self.status_mutex.lock().unwrap();
        status.running = true;
        status.calibration_mode = self.calibration_mode;
        status.cursor_x = self.cursor_x;
        status.cursor_y = self.cursor_y;
        status.master_width = self.master_width;
        status.master_height = self.master_height;
        status.remote_mode = self.remote_mode;
    }
}
