// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Load environment variables from .env file (e.g. WEBKIT_DISABLE_COMPOSITING_MODE)
    dotenvy::dotenv().ok();

    app_lib::run();
}
