use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle, LinkStyle};

pub fn html2markdown(body: String) -> Result<String, String> {
    let options = ConversionOptions::builder()
        .heading_style(HeadingStyle::Atx)
        .link_style(LinkStyle::Reference)
        .wrap(true)
        .wrap_width(100)
        .build();
    let result = convert(&body.as_str(), Some(options)).map_err(|e| e.to_string())?;

    if !result.content.is_some() || !result.warnings.is_empty() {
        return Err(format!("Could not get markdown from html"));
    }

    Ok(result.content.unwrap())
}
