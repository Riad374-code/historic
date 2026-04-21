use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle, LinkStyle};

pub fn html2markdown(body: String) -> Result<String, String> {
    let options = ConversionOptions::builder()
        .heading_style(HeadingStyle::Atx)
        .link_style(LinkStyle::Reference)
        .wrap(false)
        .wrap_width(100)
        .build();
    let result = convert(&body, Some(options)).map_err(|e| e.to_string())?;

    match result.content {
        Some(content) if !content.trim().is_empty() => Ok(content),
        _ => Err("Could not get markdown from html".to_string()),
    }
}
