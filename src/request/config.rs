use colour::red_ln;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsConfig {
    pub apikey: String,
    pub language: String,
}

pub fn get_config() -> Result<NewsConfig, Box<dyn Error>> {
    let config_file_path = config_path()?;

    if !config_file_path.exists() {
        panic!("Please set an apikey with 'simple-cli-news -a [apikey from newsapi.org]");
    }

    let config_contents = &read_to_string(config_file_path.clone())?;

    let news_config: NewsConfig = match serde_json::from_str(config_contents) {
        Err(e) => invalid_config(e)?,
        Ok(t) => t,
    };

    Ok(news_config)
}

fn invalid_config(error: serde_json::Error) -> Result<NewsConfig, Box<dyn Error>> {
    loop {
        red_ln!("Error reading config: {}", error);
        red_ln!("How would you like to proceed:");
        red_ln!("1. Reset configuration file to default (1 or enter) or panic and manually alter files (2): ");

        let mut input: String = "".to_owned();
        std::io::stdin().read_line(&mut input)?;

        match input.replace("\n", "").as_str() {
            "1" | "" => {
                OpenOptions::new().write(true).truncate(true).open(config_path()?)?;
                return Ok({
                    set_config(None, None, false)?;
                    get_config()?
                });
            }
            "2" => panic!("Panicking program"),
            _ => {}
        }
    }
}

pub fn set_config(key: Option<String>, lang: Option<String>, recover: bool) -> Result<(), Box<dyn Error>> {
    let config_file_path = &config_path()?;
    if !config_file_path.exists() {
        create_dir_all(config_file_path.parent().unwrap())?;
        File::create(config_file_path)?;
    }

    let mut config =
        if recover {
            get_config().unwrap_or_else(|_| NewsConfig::new("".to_owned(), "en".to_owned()))
        } else {
            NewsConfig::new("".to_owned(), "en".to_owned())
        };
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file_path)?;

    if let Some(key) = key {
        config.apikey = key;
    }

    if let Some(language) = lang {
        config.language = language;
    }

    let content = serde_json::to_string(&config)?;
    file.write_all(content.as_ref())?;

    Ok(())
}

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = match ProjectDirs::from("OwO", "Waffleleroo", "simple-cli-news") {
        Some(dir) => dir,
        None => return Err(Box::from("Failed to get config")),
    };

    let config_file_path = PathBuf::from(format!("{}/config", project_dir.config_dir().display()));

    Ok(config_file_path)
}
impl NewsConfig {
    fn new(apikey: String, language: String) -> NewsConfig {
        NewsConfig { apikey, language }
    }
}
