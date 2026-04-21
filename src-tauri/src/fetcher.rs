use reqwest;

pub async fn fetch_html(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;

    let html_response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    let status = html_response.status();
    if !status.is_success() {
        return Err(format!("Could not get the needed data {status}"));
    }

    let body = html_response.text().await.map_err(|e| e.to_string())?;

    Ok(body)
}

//r.jina.ai is used for markdown but fallback is applied for any case
pub async fn fetch_markdown_from_reader(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;

    let clean = url.trim();
    let reader_url = if let Some(rest) = clean.strip_prefix("https://") {
        format!("https://r.jina.ai/http://{rest}")
    } else if let Some(rest) = clean.strip_prefix("http://") {
        format!("https://r.jina.ai/http://{rest}")
    } else {
        format!("https://r.jina.ai/http://{clean}")
    };

    let markdown_response = client
        .get(reader_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = markdown_response.status();
    if !status.is_success() {
        return Err(format!("Could not get rendered markdown {status}"));
    }

    let markdown = markdown_response.text().await.map_err(|e| e.to_string())?;
    if markdown.trim().is_empty() {
        return Err("Rendered markdown response is empty".to_string());
    }

    Ok(markdown)
}
