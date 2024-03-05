use clap::Parser;
use mewe::{search, Search};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Word to search
    #[arg(num_args(1))]
    query: String,
}

/// Grabs user query from the cli and searches it using the search() function
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let query = args.query;
    match search(query).await {
        Ok(search) => match search {
            Search::Definition(definition) => println!("{}", definition),
            Search::Suggestions(suggestions) => println!(
                "The word you've entered isn't in the dictionary. Here are some suggestions:\n{}",
                suggestions
            ),
        },
        Err(error) => println!("{}", error),
    }
}
