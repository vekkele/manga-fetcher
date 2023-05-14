#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Menu;
use tauri_plugin_log::{Builder, LogTarget};

use app::commands;

fn main() {
    let menu = Menu::os_default("Manga Fetcher");

    tauri::Builder::default()
        .menu(menu)
        .plugin(
            Builder::new()
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
            commands::aggregate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
