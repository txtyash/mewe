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

/// The result of a successful search. The search may return suggestions for words not present in the Merriam Webster dictionary.
#[derive(Debug, PartialEq)]
pub enum Search {
    /// Definition returned on a successful search for an existing word in the Merriam Webster dictionary.
    Definition(String),
    /// Word suggestions from Merriam Webster dictionary on for a misspelled word.
    Suggestions(String),
}

// TODO: Add doc examples for using search()
/// Takes the word query as input("String").
/// Returns a definition if the word was found in the Merriam Webster dictionary.
/// Returns suggestions if the word was misspelled.
/// Returns an error if the word was utterly misspelled.
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
        return Ok(Search::Definition(definition.trim().to_string()));
    }
    if let Ok(suggestions) = suggestions(document).await {
        if !suggestions.is_empty() {
            return Ok(Search::Suggestions(suggestions.trim().to_string()));
        }
    }
    Err(Box::new(SearchFailed))
}

/// Searches for the "meta" tag with the attribute: name="description"
/// Extracts the text inside the "content" attribute in the same meta tag
/// If the tag is not found then returns an error else returns the definition
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

/// Searches for the "p" tag with class name "spelling-suggestions"
/// Extracts the text inside the "a" tags inside the "p" tags
/// If the tag is not found or the text is empty then returns an error
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

#[cfg(test)]
mod tests {
    use super::{search, Search};
    #[tokio::test]
    async fn query_dictionary() {
        // Searching an existing word "dictionary"
        let definition = search("dictionary".to_string()).await;
        let saved_definition = Search::Definition("The meaning of DICTIONARY is a reference source in print or electronic form containing words usually alphabetically arranged along with information about their forms, pronunciations, functions, etymologies, meanings, and syntactic and idiomatic uses.".to_string());
        assert!(definition.is_ok_and(|definition| definition == saved_definition));
    }
    #[tokio::test]
    async fn misspelling() {
        // Searching a misspelled word "dictionar"
        let suggestions = search("dictionar".to_string()).await;
        let saved_suggestions = Search::Suggestions("dictionary,dictional,diction,dictions,dictionaries,fictional,dictionally,factionary,diactinal,frictional,dictyonine,lectionary,diactin,sectionary,diactine,duction,indicational,miction,discretionary,fiction,".to_string());
        assert!(suggestions.is_ok_and(|suggestions| suggestions == saved_suggestions));
    }
    #[tokio::test]
    async fn search_garbage_word() {
        // Searching a garbage word "zqrxg" with possibly no suggestions
        assert!(search("zqrxg".to_string()).await.is_err());
    }
}
