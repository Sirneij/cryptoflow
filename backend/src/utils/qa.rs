use itertools::Itertools;

pub async fn slugify(title: &str) -> String {
    let regex = regex::Regex::new(r#"(?m)[\p{P}\p{S}]"#).unwrap();
    let result = regex.replace_all(title, "");
    result
        .to_ascii_lowercase()
        .split_ascii_whitespace()
        .join("-")
}

#[tracing::instrument(name = "Convert markdown to HTML", skip(text))]
pub async fn convert_markdown_to_html(text: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(text, options);
    let mut html_output: String = String::with_capacity(text.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
