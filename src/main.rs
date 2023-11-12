use clap::Parser;
use reqwest::get;
use scraper::{Html, Selector};
use std::{error::Error, fmt};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct DefinitionMissing;

impl Error for DefinitionMissing {}

impl fmt::Display for DefinitionMissing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find the definition.")
    }
}

#[derive(Debug, Clone)]
struct NoSuggestions;

impl Error for NoSuggestions {}

impl fmt::Display for NoSuggestions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect spelling.")
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Word to search
    #[arg(num_args(1))]
    query: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let query = args.query;
    // https://www.merriam-webster.com/dictionary/stuff
    let html = get(format!(
        "https://www.merriam-webster.com/dictionary/{}",
        query
    ))
    .await?
    .text()
    .await?;
    let document = Html::parse_document(&html);
    println!("Query: {}", query);
    if let Ok(mut definition) = definition(document.clone()).await {
        // Remove this suffix from every definition: "How to use stuff in a sentence."
        definition.truncate(definition.len() - 32);
        println!("Definition: {}", definition);
    } else if let Ok(suggestions) = suggestions(document).await {
        if !suggestions.is_empty() {
            println!("You probably misspelled it.");
            println!("Suggestions: {}", suggestions);
        } else {
            println!("Word not found.");
        }
    }
    Ok(())
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
        _ => Err(Box::new(DefinitionMissing)),
    }
}

async fn suggestions(document: Html) -> Result<String> {
    // Selector for: <p class="spelling-suggestions"><a href="/medical/sluff">sluff</a></p>
    let mut suggestions = String::new();
    let ss_sel = Selector::parse("p.spelling-suggestions")?;
    for node in document.select(&ss_sel) {
        suggestions.push_str(&node.text().collect::<String>());
        suggestions.push_str(",");
    }
    Ok(suggestions)
}
