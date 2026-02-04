pub mod driver_loop;
pub mod master_service;
pub mod slave_service;
pub mod status;
pub mod ui_server;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    ui_server::run();
}
