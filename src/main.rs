mod theme;

use dotenv::dotenv;
use newsapi::{get_articles, Data};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let url = get_url();
    let data: Data = get_articles(&url)?;

    print_articles(data);

    Ok(())
}

fn print_articles(data: Data) -> () {
    let theme = theme::default();
    theme.print_text("Top headlines\n\n");
    for article in &data.articles {
        theme.print_text(&format!("`{}`", article.title));
        theme.print_text(&format!("> *{}*", article.url));
        theme.print_text("---")
    }
}

fn get_api_key() -> Result<String, dotenv::Error> {
    dotenv();

    let api_key = std::env::var("NEWS_API_KEY").unwrap_or("".to_string());

    Ok(api_key)
}

fn get_url() -> String {
    let api_key = get_api_key().unwrap();
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    format!("{}{}", url, api_key)
}
