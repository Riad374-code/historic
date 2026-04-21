mod fetcher;
mod html_parser;

use fetcher::{fetch_html, fetch_markdown_from_reader};
use html_parser::html2markdown;
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

#[tauri::command]
async fn html_to_markdown(body: String) -> Result<String, String> {
    let markdown = html2markdown(body)?;

    Ok(markdown)
}

#[tauri::command]
async fn fetch_markdown(url: String) -> Result<String, String> {
    fetch_markdown_from_reader(url).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetcher, html_to_markdown, fetch_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
