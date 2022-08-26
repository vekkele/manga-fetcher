#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_log::{LogTarget, LoggerBuilder};

use app::commands;

fn main() {
    tauri::Builder::default()
        .plugin(
            LoggerBuilder::new()
                .targets([
                    // write to the OS logs folder
                    LogTarget::LogDir,
                    // write to stdout
                    LogTarget::Stdout,
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::get_manga,
            commands::get_chapters,
            commands::download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
