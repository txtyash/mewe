use reqwest::get;
use scraper::{Html, Selector};
use std::{error::Error, fmt};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct SearchFailed;

impl Error for SearchFailed {}

impl fmt::Display for SearchFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sorry, the word you’re looking for can’t be found in the dictionary."
        )
    }
}

#[derive(Debug)]
pub enum Search {
    Definition(String),
    Suggestions(String),
}

pub async fn search(query: String) -> Result<Search> {
    let html = get(format!(
        "https://www.merriam-webster.com/dictionary/{}",
        query
    ))
    .await?
    .text()
    .await?;
    let document = Html::parse_document(&html);
    if let Ok(mut definition) = definition(document.clone()).await {
        // Definition ends with unnecessary text: "How to use..."
        // Remove that by finding the last match of the pattern.
        let index = definition.rfind("How to use").unwrap_or(definition.len());
        definition.truncate(index);
        return Ok(Search::Definition(definition));
    }
    if let Ok(suggestions) = suggestions(document).await {
        if !suggestions.is_empty() {
            return Ok(Search::Suggestions(suggestions));
        }
    }
    Err(Box::new(SearchFailed))
}

async fn definition(document: Html) -> Result<String> {
    // Selector for definition: <meta name="description" content="The meaning of STUFF is materials, supplies, or equipment used in various activities. How to use stuff in a sentence.">
    let def_sel = Selector::parse(r#"meta[name="description"]"#)?;
    match document
        .select(&def_sel)
        .next()
        .and_then(|node| node.value().attr("content"))
    {
        Some(definition) => Ok(definition.into()),
        _ => Err(Box::new(SearchFailed)),
    }
}

async fn suggestions(document: Html) -> Result<String> {
    // Selector for: <p class="spelling-suggestions"><a href="/medical/sluff">sluff</a></p>
    let mut suggestions = String::new();
    let ss_sel = Selector::parse("p.spelling-suggestions")?;
    for node in document.select(&ss_sel) {
        suggestions.push_str(&node.text().collect::<String>());
        suggestions.push(',');
    }
    Ok(suggestions)
}
