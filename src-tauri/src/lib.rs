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
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "restore_sites_gui_lib=debug,info".into()),
        )
        .init();

    info!("Starting Restore Sites application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::save_collection,
            commands::load_collections,
            commands::restore_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
