mod fetcher;
mod file_design;
mod html_parser;

use fetcher::{fetch_html, fetch_markdown_from_reader};
use file_design::markdown2pdf;
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

/// Downloads all remote images in `markdown` to temp files and replaces their
/// URLs with absolute local paths so the PDF renderer can read them reliably.
/// Returns the rewritten markdown and the temp paths to clean up afterwards.
async fn localise_images(markdown: &str) -> (String, Vec<std::path::PathBuf>) {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(_) => return (markdown.to_string(), vec![]),
    };

    let mut output = String::with_capacity(markdown.len());
    let mut temp_files: Vec<std::path::PathBuf> = Vec::new();
    let mut pos = 0;

    while pos < markdown.len() {
        if markdown[pos..].starts_with("![") {
            let after_bang = pos + 2;
            if let Some(rel_bracket) = markdown[after_bang..].find("](") {
                let alt_end = after_bang + rel_bracket;
                let url_start = alt_end + 2;
                if let Some(rel_paren) = markdown[url_start..].find(')') {
                    let url = &markdown[url_start..url_start + rel_paren];
                    let end = url_start + rel_paren + 1;

                    if url.starts_with("http://") || url.starts_with("https://") {
                        if let Ok(resp) = client.get(url).send().await {
                            if let Ok(bytes) = resp.bytes().await {
                                let ext = url
                                    .split('?')
                                    .next()
                                    .unwrap_or(url)
                                    .rsplit('.')
                                    .next()
                                    .filter(|e| {
                                        e.len() <= 5 && e.chars().all(|c| c.is_alphanumeric())
                                    })
                                    .unwrap_or("png");
                                let tmp = std::env::temp_dir()
                                    .join(format!("historic_img_{}.{}", temp_files.len(), ext));
                                if std::fs::write(&tmp, &bytes).is_ok() {
                                    let path_str = tmp.to_string_lossy().replace('\\', "/");
                                    let alt = &markdown[after_bang..alt_end];
                                    output.push_str(&format!("![{}]({})", alt, path_str));
                                    temp_files.push(tmp);
                                    pos = end;
                                    continue;
                                }
                            }
                        }
                    }

                    output.push_str(&markdown[pos..end]);
                    pos = end;
                    continue;
                }
            }
        }

        let ch_len = markdown[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        output.push_str(&markdown[pos..pos + ch_len]);
        pos += ch_len;
    }

    (output, temp_files)
}

#[tauri::command]
async fn create_pdf(markdown: String, output_path: String) -> Result<String, String> {
    let (localised, temp_files) = localise_images(&markdown).await;

    let selected_path = std::path::PathBuf::from(output_path);
    if let Some(parent) = selected_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to prepare output directory: {e}"))?;
    }

    let result = match markdown2pdf(localised.clone(), selected_path.clone()) {
        Ok(written_path) => Ok(written_path.to_string_lossy().to_string()),
        Err(primary_error) => {
            let fallback_path = std::env::var("USERPROFILE")
                .ok()
                .map(std::path::PathBuf::from)
                .and_then(|base| {
                    selected_path
                        .file_name()
                        .map(|name| base.join("Downloads").join(name))
                });

            if let Some(path) = fallback_path {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to prepare fallback directory: {e}"))?;
                }

                let fallback_written =
                    markdown2pdf(localised, path).map_err(|fallback_error| {
                        format!(
                        "Failed to save in selected folder ({primary_error}) and fallback folder ({fallback_error})"
                    )
                    })?;

                Ok(fallback_written.to_string_lossy().to_string())
            } else {
                Err(primary_error)
            }
        }
    };

    for p in &temp_files {
        let _ = std::fs::remove_file(p);
    }

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![fetch_markdown, create_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
