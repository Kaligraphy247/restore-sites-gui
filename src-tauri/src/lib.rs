// Module declarations
mod commands;
mod database;
mod models;
mod services;
mod utils;

use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing
    let default_level = if cfg!(debug_assertions) {
        "restore_sites_gui_lib=debug,info"
    } else {
        "warn"
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_level.into()),
        )
        .init();

    info!("Starting Restore Sites application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::save_collection,
            commands::load_collections,
            commands::restore_collection,
            commands::get_collection,
            commands::update_collection,
            commands::delete_collection,
            // Browser Profile Management
            commands::create_browser_profile,
            commands::get_browser_profiles,
            commands::get_browser_profile,
            commands::update_browser_profile,
            commands::delete_browser_profile,
            // Browser Detection
            commands::check_browser_detection,
            // Default Browser Mode Management
            commands::get_default_browser_mode,
            commands::set_default_browser_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
