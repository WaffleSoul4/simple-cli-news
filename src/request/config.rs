use colour::red_ln;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, remove_file, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsConfig {
    pub apikey: String,
    pub language: String,
}

pub fn get_config() -> Result<NewsConfig, Box<dyn Error>> {
    let project_dir = match ProjectDirs::from("OwO", "Waffleleroo", "simple-cli-news") {
        Some(dir) => dir,
        None => return Err(Box::from("Failed to get config")),
    };

    let config_file_path = config_path()?;

    if !config_file_path.exists() {
        if !project_dir.config_dir().exists() {
            create_dir_all(project_dir.config_dir())?;
        }
        let content = serde_json::to_string(&NewsConfig::default())?;
        let mut file = File::create(&config_file_path)?;
        file.write_all(content.as_ref())?;
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
                remove_file(config_path()?)?;
                return Ok(get_config()?);
            }
            "2" => panic!("Panicking program"),
            _ => {}
        }
    }
}

pub fn set_config(key: Option<String>, lang: Option<String>) -> Result<(), Box<dyn Error>> {
    let config_file_path = config_path()?;

    let config = get_config()?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file_path)?;

    let lang = match lang {
        Some(t) => t,
        None => config.language,
    };

    let key = match key {
        Some(t) => t,
        None => config.apikey,
    };

    file.set_len(0)?;

    let content = serde_json::to_string(&NewsConfig::new(key.clone(), lang.clone()))?;
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

    fn default() -> NewsConfig {
        NewsConfig::new(
            "c314df27e7884185a4720d347f50e1d4".to_owned(),
            "en".to_owned(),
        )
    }
}
