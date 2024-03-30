mod theme;

use dotenv::dotenv;
use newsapi::{Country, Endpoint, NewsApiResponse};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
    let data = newsapi::NewsApi::new(api_key.as_str())
        .country(Country::Us)
        .endpoint(Endpoint::TopHeadlines)
        .fetch()?;

    print_articles(data);

    Ok(())
}

fn print_articles(data: NewsApiResponse) -> () {
    let theme = theme::default();
    theme.print_text("Top headlines\n\n");
    for article in data.get_articles() {
        theme.print_text(&format!("`{}`", article.title()));
        theme.print_text(&format!("> *{}*", article.url()));
        theme.print_text("---")
    }
}

fn get_api_key() -> Result<String, dotenv::Error> {
    dotenv();

    let api_key = std::env::var("NEWS_API_KEY").unwrap_or("".to_string());

    Ok(api_key)
}
