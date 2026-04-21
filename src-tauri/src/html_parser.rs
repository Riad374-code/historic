use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle, LinkStyle};

/// Extract the content between `<body` and `</body>` tags.
/// Falls back to the full HTML if no body tags are found.
fn extract_body(html: &str) -> &str {
    let lower = html.to_lowercase();
    if let Some(body_tag_start) = lower.find("<body") {
        // Skip past the closing `>` of the opening tag (handles attributes)
        if let Some(rel_close) = lower[body_tag_start..].find('>') {
            let content_start = body_tag_start + rel_close + 1;
            let end = lower.find("</body>").unwrap_or(html.len());
            return &html[content_start..end];
        }
    }
    html
}

pub fn html2markdown(html: String) -> Result<String, String> {
    let body = extract_body(&html);

    let options = ConversionOptions::builder()
        .heading_style(HeadingStyle::Atx)
        .link_style(LinkStyle::Reference)
        .wrap(false)
        .wrap_width(100)
        .build();
    let result = convert(body, Some(options)).map_err(|e| e.to_string())?;

    match result.content {
        Some(content) if !content.trim().is_empty() => Ok(content),
        _ => Err("Could not get markdown from html".to_string()),
    }
}
