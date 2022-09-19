use serde::{Serialize, Deserialize};
 use colored::*;


#[derive(Serialize, Deserialize, Debug)]
struct QuoteResponse {
    _id: String,
    content: String,
    tags: Vec<String>,
    length: i32,
    author: String,
}

use clap::{Parser, ValueEnum};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_enum)]
    quote: Quote,
}

#[derive(ValueEnum, Clone)]
enum Quote {
    Honor,
    Humor,
    Proverb,
    Philosophy,
    History,
    Science,
    Random,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let quote = match args.quote {
        q => get_quote(q).await,
    };
    if let Ok(q) = quote {
        println!("\n {} - {} \n", q.content.green().bold(), q.author.magenta());
    } else {
        println!("\n {} \n", "Something went wrong!".red());
    }
}

async fn get_quote(
    quote_type: Quote,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let url = match quote_type {
        Quote::Honor => "https://api.quotable.io/random?tags=honor|war|virtue",
        Quote::Humor => "https://api.quotable.io/random?tags=humor|humorous",
        Quote::Proverb => "https://api.quotable.io/random?tags=proverb|wisdom|inspirational|leadership|politics",
        Quote::Philosophy => "https://api.quotable.io/random?tags=philosophy|life|pain|spirituality",
        Quote::History => "https://api.quotable.io/random?tags=history|literature",
        Quote::Science => "https://api.quotable.io/random?tags=science|technology",
        Quote::Random => "https://api.quotable.io/random",
    };
    let resp: QuoteResponse = reqwest::get(url)
        .await?
        .json::<QuoteResponse>()
        .await?;
    Ok(resp)
}
