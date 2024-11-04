mod list;
mod request;

use crate::list::get_sources;
use crate::request::Request;
use clap::{command, Arg, Command};
use request::config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output_news = true;

    let args = command!()
        .subcommand(
            Command::new("list")
                .about("List possible args for various commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("sources").about("List possible sources").arg(
                        Arg::new("country")
                            .short('c')
                            .long("country")
                            .alias("location")
                            .help("List sources from a country using its 2-Digit ISO code"),
                    ),
                ),
        )
        .arg(
            Arg::new("apikey")
                .short('a')
                .long("apikey")
                .alias("key")
                .help("Update the config with an api key from NewsApi"),
        )
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .alias("search")
                .help("Search news from the past 14 days"),
        )
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .help("Get news form a certain source with its ID"),
        )
        .arg(
            Arg::new("page_size")
                .short('p')
                .long("page_size")
                .aliases(["page-size", "pagesize", "pgsize", "pg-size"])
                .help("Set how many articles should be displayed"),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .alias("lang")
                .help("Set the default language"),
        )
        .get_matches();

    match args.subcommand() {
        Some(("list", sub_args)) => match sub_args.subcommand() {
            Some(("sources", sub_sub_args)) => {
                let country = if let Some(country) = sub_sub_args.get_one::<String>("country") {
                    Some(country.as_str())
                } else {
                    None
                };

                get_sources(country)?.display();
            }
            _ => {}
        },
        _ => {
            if let Some(apikey) = args.get_one::<String>("apikey") {
                config::set_config(Some(apikey.clone()), None,true)?;
                output_news = false;
            }

            if let Some(language) = args.get_one::<String>("language") {
                config::set_config(None, Some(language.clone()),true)?;
                output_news = false;
            }

            if output_news {
                let mut request = Request::new_empty()?;

                if let Some(query) = args.get_one::<String>("query") {
                    request.q = Some(query.clone());
                    request = request.with_everything();
                }

                if let Some(source) = args.get_one::<String>("source") {
                    request.sources = Some(source.clone());
                    request = request.with_everything();
                }

                if let Some(pagesize) = args.get_one::<String>("page_size") {
                    request.page_size = pagesize.clone().parse::<i32>()?;
                }

                request.to_response()?.output();
            }
        }
    }

    Ok(())
}
