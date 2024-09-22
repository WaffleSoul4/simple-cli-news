use colour::{green_ln_bold, red_ln, yellow_ln};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct Sources {
    sources: Vec<Source>,
}

#[derive(Deserialize)]
struct Source {
    name: String,
    id: String,
    description: String,
}

impl Sources {
    pub fn display(self) {
        for source in self.sources {
            green_ln_bold!("{}", source.name);
            yellow_ln!("{}", source.description);
            yellow_ln!("id: {}", source.id);
            yellow_ln!();
        }
    }
}

pub fn get_sources(country: Option<String>) -> Result<Sources, Box<dyn Error>> {
    let response = match country {
        Some(d) => ureq::get(&format!("https://newsapi.org/v2/top-headlines/sources?apiKey=c314df27e7884185a4720d347f50e1d4&country={}",d)),
        None => ureq::get("https://newsapi.org/v2/top-headlines/sources?apiKey=c314df27e7884185a4720d347f50e1d4"),
    }.call()?.into_string()?;
    let sources: Sources = serde_json::from_str(&response)?;
    if sources.sources.len() == 0 {
        red_ln!("No sources could be found for this country")
    }
    Ok(sources)
}
