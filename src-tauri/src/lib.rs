mod fetcher;
mod html_parser;

use fetcher::{fetch_html, fetch_markdown_from_reader};
use html_parser::html2markdown;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkdownResult {
    markdown: String,
    source: String,
    used_fallback: bool,
}

fn looks_good_enough(markdown: &str) -> bool {
    let body = markdown.trim();
    if body.len() < 250 {
        return false;
    }

    let alpha_count = body.chars().filter(|c| c.is_alphabetic()).count();
    let has_structure = body.contains('\n');
    alpha_count > 80 && has_structure
}

fn polish_markdown(mut markdown: String, source_url: &str) -> String {
    if !markdown.contains("http://") && !markdown.contains("https://") {
        markdown.push_str(&format!("\n\nSource: {source_url}"));
    }

    let has_table = markdown
        .lines()
        .any(|line| line.trim_start().starts_with('|'));
    if !has_table {
        markdown.push_str("\n\n| Field | Value |\n| --- | --- |\n");
        markdown.push_str(&format!("| Source URL | {source_url} |\n"));
        markdown.push_str("| Method | local-html-parser |\n");
    }

    if !markdown.contains("![") {
        let favicon_url =
            format!("https://www.google.com/s2/favicons?sz=128&domain_url={source_url}");
        markdown.push_str(&format!("\n\n![Source icon]({favicon_url})"));
    }

    markdown
}
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
async fn fetch_markdown(url: String) -> Result<MarkdownResult, String> {
    let clean_url = url.trim().to_string();

    let reader_attempt = fetch_markdown_from_reader(clean_url.clone())
        .await
        .ok()
        .filter(|md| !md.trim().is_empty());

    if let Some(markdown) = reader_attempt.as_ref() {
        if looks_good_enough(markdown) {
            return Ok(MarkdownResult {
                markdown: markdown.clone(),
                source: "r.jina.ai".to_string(),
                used_fallback: false,
            });
        }
    }

    match fetch_html(clean_url.clone()).await {
        Ok(html) => match html2markdown(html) {
            Ok(local_markdown) => Ok(MarkdownResult {
                markdown: polish_markdown(local_markdown, &clean_url),
                source: "local-html-parser".to_string(),
                used_fallback: true,
            }),
            Err(local_error) => {
                if let Some(markdown) = reader_attempt {
                    Ok(MarkdownResult {
                        markdown,
                        source: "r.jina.ai (low-confidence)".to_string(),
                        used_fallback: false,
                    })
                } else {
                    Err(format!("Reader and local parser failed: {local_error}"))
                }
            }
        },
        Err(fetch_error) => {
            if let Some(markdown) = reader_attempt {
                Ok(MarkdownResult {
                    markdown,
                    source: "r.jina.ai (low-confidence)".to_string(),
                    used_fallback: false,
                })
            } else {
                Err(format!("Reader and HTML fetch failed: {fetch_error}"))
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            fetcher,
            html_to_markdown,
            fetch_markdown
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
