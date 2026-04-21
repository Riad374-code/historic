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
