mod list;

use crate::list::get_sources;
use chrono::{Duration, Local};
use colour::{blue_ln, red_ln, yellow_ln};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
struct NewsApiUrl {
    endpoint: String,
    api_key: String,
    country: Option<String>,
    language: Option<String>,
    query: Option<String>,
    category: Option<String>,
    sources: Option<String>,
}

impl NewsApiUrl {
    fn new(endpoint: String, api_key: String) -> NewsApiUrl {
        NewsApiUrl {
            endpoint,
            api_key,
            country: Some(String::from("us")),
            language: None,
            query: None,
            category: None,
            sources: None,
        }
    }

    fn to_str(&self) -> String {
        let mut base = format!(
            "https://newsapi.org/v2/{}?apiKey={}",
            self.endpoint, self.api_key,
        );

        if self.endpoint == "everything" {
            let date = (Local::now() - Duration::days(14))
                .format("%Y-%m-%d")
                .to_string();

            match &self.language {
                Some(d) => base = format!("{}&language={}", base, d),
                _ => {}
            }

            base = format!("{base}&from={date}&pageSize=20")
        } else if self.endpoint == "top-headlines" {
            match &self.sources {
                Some(_) => {}
                None => match &self.country {
                    Some(d) => base = format!("{}&country={}", base, d),
                    _ => {}
                },
            }

            match &self.category {
                Some(d) => base = format!("{}&category={}", base, d),
                _ => {}
            }
        }

        match &self.sources {
            Some(d) => base = format!("{}&sources={}", base, d),
            _ => {}
        }

        match &self.query {
            Some(d) => base = format!("{}&q={}", base, d),
            _ => {}
        }

        base
    }
}

fn main() {
    let mut url: NewsApiUrl = NewsApiUrl::new(
        String::from("top-headlines"),
        String::from("c314df27e7884185a4720d347f50e1d4"),
    );

    let args: Vec<String> = std::env::args().collect();
    let arg_count = args.len();
    let mut return_articles = true;

    if arg_count > 1 {
        match args[1].as_str() {
            "q" => {
                if arg_count > 2 {
                    url.query = Some(args[2].clone());
                    url.endpoint = String::from("everything");
                } else {
                    red_ln!("Please input a query")
                }
            }
            "source" => {
                if arg_count > 2 {
                    if arg_count > 3 {
                        red_ln!("Please separate sources using only commas and no spaces")
                    } else {
                        url.sources = Some(args[2].clone())
                    }
                } else {
                    red_ln!("Please enter sources");
                    red_ln!("Run `simple-cli-news list sources' to get a list of sources")
                }
            }
            "list" => {
                return_articles = false;
                if arg_count > 2 {
                    match args[2].as_str() {
                        "sources" => {
                            if arg_count > 3 {
                                if arg_count < 5 {
                                    match get_sources(Some(args[3].clone())) {
                                        Ok(d) => {
                                            d.display();
                                        }
                                        Err(e) => red_ln!("Failed to get sources: {}", e),
                                    }
                                } else {
                                    red_ln!("Please input one country at a time")
                                }
                            } else {
                                match get_sources(None) {
                                    Ok(d) => {
                                        d.display();
                                    }
                                    Err(e) => red_ln!("Failed to get sources: {}", e),
                                }
                            }
                        }
                        _ => {
                            red_ln!("Cannot list {}", args[2])
                        }
                    }
                }
            }
            _ => {
                red_ln!("Invalid argument")
            }
        }
    }
    if return_articles {
        match get_articles(&url.to_str()) {
            Ok(d) => print_articles(d),
            Err(e) => red_ln!("Failed to get articles: {}", e),
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
        println!();
        yellow_ln!("{}", article.title);
        blue_ln!(">>> {}", article.url);
    }
}
