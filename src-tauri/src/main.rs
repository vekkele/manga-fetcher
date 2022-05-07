#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Deserialize, serde::Serialize)]
struct TitleView {
    name: String,
    url: String,
}

#[tauri::command]
fn search(query: &str) -> Vec<TitleView> {
    println!("Search query123 is {query}");

    return vec![
        TitleView {
            name: format!("{} 1", query),
            url: "some url".to_string(),
        },
        TitleView {
            name: format!("{} 2", query),
            url: "another url".to_string(),
        },
    ];
}
