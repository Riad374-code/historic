use glyphweaveforge::{BuiltInTheme, Forge, LayoutMode, RenderBackendSelection};
use std::path::PathBuf;

fn unwrap_fenced_markdown(markdown: &str) -> String {
    let normalized = markdown.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.lines().collect();
    if lines.len() >= 3
        && lines[0].trim_start().starts_with("```")
        && lines[lines.len() - 1].trim() == "```"
    {
        return lines[1..lines.len() - 1].join("\n");
    }
    normalized
}

#[derive(PartialEq)]
enum LineKind {
    Blank,
    Heading,
    OrderedList,
    BulletList,
    Other,
}

fn classify(line: &str) -> LineKind {
    let t = line.trim();
    if t.is_empty() {
        return LineKind::Blank;
    }
    // Heading: 1–6 hashes followed by a space
    let hashes = t.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&hashes) && t[hashes..].starts_with(' ') {
        return LineKind::Heading;
    }
    // Bullet: `* text` or `- text` (after normalise_line, always single space)
    if (t.starts_with("* ") || t.starts_with("- ")) && t.len() > 2 {
        return LineKind::BulletList;
    }
    // Ordered list: digits followed by `. text`
    let digit_end = t
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(i, _)| i + 1);
    if let Some(end) = digit_end {
        if end > 0 && t.as_bytes().get(end) == Some(&b'.') && !t[end + 1..].trim_start().is_empty()
        {
            return LineKind::OrderedList;
        }
    }
    LineKind::Other
}

fn normalise_line(line: &str) -> String {
    let expanded = line.replace('\t', "    ");
    let t = expanded.trim_start();
    let indent = &expanded[..expanded.len() - t.len()];

    // Headings take priority — never reparse as list
    let hashes = t.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&hashes) {
        let after = &t[hashes..];
        // Fix `##Foo` → `## Foo`; already-valid headings pass through
        if !after.is_empty() && !after.starts_with(' ') {
            return format!("{}{} {}", indent, "#".repeat(hashes), after);
        }
        return expanded;
    }

    // Normalise bullet lists: collapse any whitespace after marker to one space
    // Handles `*   text`, `-  text`, `* text`, `*\ttext`, etc.
    // In normalise_line, change bullet output from `*` to `-`:
    // Guard: next byte must be whitespace so `**bold**` is not treated as a bullet
    let is_bullet_marker = (t.starts_with('*') || t.starts_with('-') || t.starts_with('+'))
        && t.as_bytes().get(1).map_or(false, |b| b.is_ascii_whitespace());
    if is_bullet_marker {
        let after = t[1..].trim_start();
        if !after.is_empty() {
            return format!("{}- {}", indent, after);
        }
    }

    // Normalise ordered lists: `1.   text` → `1. text`
    let digit_end = t
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(i, _)| i + 1);
    if let Some(end) = digit_end {
        if end > 0 && t.as_bytes().get(end) == Some(&b'.') {
            let after = t[end + 1..].trim_start();
            if !after.is_empty() {
                return format!("{}{}. {}", indent, &t[..end], after);
            }
        }
    }

    expanded
}

fn normalize_markdown_for_pdf(markdown: &str) -> String {
    let source = unwrap_fenced_markdown(markdown);
    let mut out = String::new();
    let mut prev = LineKind::Blank;

    for raw in source.lines() {
        let line = normalise_line(raw);
        let kind = classify(&line);

        match kind {
            LineKind::Blank => {
                // Suppress blank lines between list items so pulldown-cmark
                // keeps the list "tight" (items are not wrapped in paragraphs).
                if prev != LineKind::Blank
                    && prev != LineKind::BulletList
                    && prev != LineKind::OrderedList
                {
                    out.push('\n');
                }
            }
            LineKind::Heading => {
                if prev != LineKind::Blank {
                    out.push('\n');
                }
                out.push_str(&line);
                out.push('\n');
                out.push('\n');
            }
            LineKind::OrderedList | LineKind::BulletList => {
                // Insert blank line when transitioning into a list from prose
                if prev != LineKind::Blank
                    && prev != LineKind::OrderedList
                    && prev != LineKind::BulletList
                {
                    out.push('\n');
                }
                out.push_str(&line);
                out.push('\n');
            }
            LineKind::Other => {
                out.push_str(&line);
                out.push('\n');
            }
        }

        prev = kind;
    }

    out
}

pub fn markdown2pdf(markdown: String, output_path: PathBuf) -> Result<PathBuf, String> {
    let normalized = normalize_markdown_for_pdf(&markdown);

    let pdf = Forge::new()
        .from_text(&normalized)
        .with_backend(RenderBackendSelection::Typst)
        .with_layout_mode(LayoutMode::Paged)
        .with_theme(BuiltInTheme::Informational)
        .to_file(output_path.as_path())
        .convert()
        .map_err(|e| e.to_string())?;

    pdf.written_path
        .ok_or_else(|| "PDF was generated but output path was not returned".to_string())
}
