use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Data {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    description: String,
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let url = get_url();
    let data: Data = get_articles(&url)?;

    print_articles(data);

    Ok(())
}

fn print_articles(data: Data) -> () {
    for article in &data.articles {
        println!("\n\n\n");
        colour::dark_yellow!("> {}", article.title);
        print!("\n");
        colour::white!("  {}", article.description);
        print!("\n\n");
        colour::dark_green!("  {}", article.url);
    }
}

fn get_articles(url: &String) -> Result<Data, Box<dyn Error>> {
    let resp = ureq::get(url).call();

    if resp.is_ok() {
        let response = resp.unwrap().into_string();
        if response.is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to convert response to string",
            )));
        }

        let articles = serde_json::from_str(&response.unwrap());

        if articles.is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to parse response",
            )));
        }

        return Ok(articles.unwrap());
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch data",
        )));
    }
}

fn get_api_key() -> String {
    String::from("1bbbd9dd69564803a61305b23057397d")
}

fn get_url() -> String {
    format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={}",
        get_api_key()
    )
}
