use colour::{green_ln,magenta_ln};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}
#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn main() {
    let apikey: String = "c314df27e7884185a4720d347f50e1d4".to_string();

    let mut url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={}",
        apikey
    );

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        url = format!("{}&q={}", url, args[1])
    }

    match get_articles(&url) {
        Ok(d) => print_articles(d),
        Err(e) => {
            println!("Failed to get articles: {}", e)
        }
    }
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn print_articles(articles: Articles) {
    for article in articles.articles {
        magenta_ln!("{}", article.title);
        green_ln!(">>> {}", article.url);
        println!()
    }
}
