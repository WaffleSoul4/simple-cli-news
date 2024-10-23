pub mod config;

use crate::request::config::get_config;
use chrono::{serde::ts_seconds_option, DateTime, Duration, Utc};
use colour::{blue_ln, red_ln, yellow_ln};
use serde::{Deserialize, Serialize};
use std::error::Error;
use Endpoint::{Everything, Headlines};

#[derive(Deserialize, Serialize, Debug)]
pub enum Endpoint {
    Everything {
        #[serde(with = "ts_seconds_option")]
        from: Option<DateTime<Utc>>,
    },
    Headlines {
        country: Option<String>,
        category: Option<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Response {
    articles: Vec<Article>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Article {
    source: Source,
    title: String,
    description: Option<String>,
    author: Option<String>,
    url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Source {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub endpoint: Endpoint,
    pub apikey: String,
    pub sources: Option<String>,
    pub q: Option<String>,
    pub page_size: i32,
    pub language: String,
}

impl Request {
    pub fn with_everything(self) -> Request {
        let endpoint = match self.endpoint {
            Everything { from } => Endpoint::new_everything(from),
            _ => Endpoint::new_everything(Some(Utc::now() - Duration::days(10))),
        };
        Request::new(
            endpoint,
            self.apikey,
            self.sources,
            self.q,
            self.page_size,
            self.language,
        )
    }

    #[allow(dead_code)]
    pub fn with_headlines(self) -> Request {
        let endpoint = match self.endpoint {
            Headlines { country, category } => Endpoint::new_headlines(country, category),
            _ => Endpoint::new_headlines(None, None),
        };
        Request::new(
            endpoint,
            self.apikey,
            self.sources,
            self.q,
            self.page_size,
            self.language,
        )
    }

    pub fn new(
        endpoint: Endpoint,
        apikey: String,
        sources: Option<String>,
        q: Option<String>,
        page_size: i32,
        language: String,
    ) -> Request {
        Request {
            endpoint,
            apikey,
            sources,
            q,
            page_size,
            language,
        }
    }

    pub fn new_empty() -> Result<Request, Box<dyn Error>> {
        let config = get_config()?;
        Ok(Request {
            endpoint: Endpoint::new_headlines(None, None),
            apikey: config.apikey,
            sources: None,
            q: None,
            page_size: 20,
            language: config.language,
        })
    }

    fn to_string(self) -> String {
        let mut link = "https://newsapi.org/v2/".to_owned();

        match self.endpoint {
            Everything { from } => {
                link = format!("{}everything?", link);

                if let Some(from_unwrapped) = from {
                    link = format!("{}from={}&", link, from_unwrapped.format("%Y-%d-%m"))
                }
            }
            Headlines { country, category } => {
                link = format!("{}top-headlines?", link);

                match country {
                    Some(q) => {
                        link = format!("{}country={}&", link, q);
                    }
                    _ => {}
                };

                match category {
                    Some(q) => link = format!("{}category={}&", link, q),
                    _ => {}
                };
            }
        }
        match self.q {
            Some(q) => link = format!("{}q={}&", link, q),
            _ => {}
        };

        match self.sources {
            Some(q) => link = format!("{}sources={}&", link, q),
            _ => {}
        };

        format!(
            "{}language={}&pageSize={}&apikey={}",
            link, self.language, self.page_size, self.apikey
        )
    }

    pub fn to_response(self) -> Result<Response, Box<dyn Error>> {
        let response = ureq::get(&self.to_string()).call()?.into_string()?;
        let response: Response = serde_json::from_str(&response)?;

        Ok(response)
    }
}

impl Response {
    pub fn output(self) {
        if self.articles.len() > 0 {
            for article in self.articles {
                println!();
                yellow_ln!("{}", article.title);
                blue_ln!(">>> {}", article.url);
            }
        } else {
            red_ln!("Failed to find any articles")
        }
    }
}

impl Endpoint {
    fn new_everything(from: Option<DateTime<Utc>>) -> Endpoint {
        Everything { from }
    }
    fn new_headlines(country: Option<String>, category: Option<String>) -> Endpoint {
        Headlines { country, category }
    }
}
