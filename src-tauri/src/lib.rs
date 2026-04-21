mod fetcher;
mod html_parser;

use fetcher::fetch_html;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn fetcher(url: String) -> Result<String, String> {
    let body = fetch_html(url).await?;
    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetcher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
