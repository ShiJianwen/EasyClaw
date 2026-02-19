pub mod commands;
pub mod utils;

use commands::gateway;
use commands::init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            init::check_initialized,
            init::initialize_zeroclaw,
            gateway::gateway_status,
            gateway::start_gateway,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
