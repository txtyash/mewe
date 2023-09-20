use regex::Regex;
use reqwest::{blocking::get, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
struct Word {
    text: String,
    definitions: Vec<DefiningText>,
    suggestions: Vec<Suggestion>,
}

/// Defining Text
#[derive(Debug)]
struct DefiningText {
    text: String,      // TODO: prepend subject label
    div_sense: String, // Divided Sense
    illustrations: Vec<Illustration>,
}

// Similar words
#[derive(Debug)]
struct Suggestion {
    text: String,
}

// Verbal Illustration
#[derive(Debug)]
struct Illustration {
    text: String,
}

/// Whether to look for definitions or suggestions?
#[derive(Debug)]
enum Action {
    FindDefinitions,
    FindSuggestions,
}

/// print suggestions/definitions/errors
fn main() {
    let query = "drums";
    let result: Result<Word, Box<dyn Error>> = match get_request(query) {
        Ok((action, document)) => match action {
            Action::FindDefinitions => get_definitions(query, document),
            Action::FindSuggestions => get_suggestions(query, document),
        },
        Err(e) => Err(e),
    };
    println!("{result:#?}")
}

/// Makes a get request & returns a scraper::html::Html struct
fn get_request(query: &'static str) -> Result<(Action, String), Box<dyn Error>> {
    // TODO: Set a timeout for requests.
    let response = get(format!(
        "https://www.merriam-webster.com/dictionary/{query}"
    ))?; // TODO: Handle Error

    let action = match &response.status() {
        &StatusCode::OK => Action::FindDefinitions,
        _ => Action::FindSuggestions, // HACK: is this always be a 404?
    };

    let document = response.text()?; // TODO: Handle Error

    Ok((action, document))
}

/// grab suggestions
fn get_suggestions(query: &'static str, document: String) -> Result<Word, Box<dyn Error>> {
    Html::parse_document(&document);
    Ok(Word {
        text: query.to_string(),
        definitions: Vec::new(),
        suggestions: Vec::new(),
    })
}

/// grab word data
fn get_definitions(query: &'static str, document: String) -> Result<Word, Box<dyn Error>> {
    let mut result = Word {
        text: query.to_string(),
        definitions: Vec::new(),
        suggestions: Vec::new(),
    };
    let document = Html::parse_document(&document);
    let vg_sel = Selector::parse("div.vg").unwrap();
    let sc_sel = Selector::parse("div.sense-content").unwrap();
    let def_sel = Selector::parse("span.dtText").unwrap();
    let sdsense_sel = Selector::parse("div.sdsense").unwrap();
    let ex_sel = Selector::parse("span.ex-sent").unwrap();
    let sl_sel = Selector::parse("span.sl").unwrap();

    // Select first dictionary
    let vg1 = document.select(&vg_sel).next().unwrap();

    for sc in vg1.select(&sc_sel) {
        let sub_label = match sc.select(&sl_sel).next() {
            Some(subject_label) => neat_text(subject_label.text().collect()),
            None => String::new(),
        };
        let mut definition: DefiningText = DefiningText {
            text: [
                sub_label,
                neat_text(sc.select(&def_sel).next().unwrap().text().collect()),
            ]
            .concat(),
            div_sense: match sc.select(&sdsense_sel).next() {
                Some(sdsense) => neat_text(sdsense.text().collect()),
                None => String::new(),
            },
            illustrations: Vec::new(),
        };
        for example in sc.select(&ex_sel) {
            let example: Illustration = Illustration {
                text: neat_text(example.text().collect()),
            };
            definition.illustrations.push(example);
        }
        // add definition to word
        result.definitions.push(definition);
    }
    Ok(result)
}

fn neat_text(text: String) -> String {
    let re = Regex::new(r"(\\n|[\s\r]|^\s*:\s*)+").unwrap(); // TODO: HANDLE ERROR
    re.replace_all(&text, " ").trim().to_string()
}
