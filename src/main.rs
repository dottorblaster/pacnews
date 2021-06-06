extern crate clap;
use clap::{App, Arg};
use rss::{Channel, Item};

const FEED_URL: &str = "https://www.archlinux.org/feeds/news/";

pub struct Entry {
    pub title: String,
    pub link: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

fn print_entry(entry: Entry) {
    println!(
        "Title: {}\nPosted: {}\nLink: {}\n{}\n\n",
        entry.title, entry.date, entry.link, entry.content,
    );
}

fn get_items(sort: &str) -> Result<Vec<Item>, Box<dyn::std::error::Error>> {
    let content = reqwest::blocking::get(FEED_URL)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    match sort {
        "desc" => Ok(channel.into_items().into_iter().rev().collect()),
        _ => Ok(channel.into_items()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("pacnews")
        .version("2.1.0")
        .author("Alessio Biancalana <dottorblaster@gmail.com>")
        .about("Read Arch Linux news feed directly from your terminal")
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .help("Sort by date")
                .value_name("asc/desc")
                .possible_values(&["asc", "desc"])
                .default_value("asc")
                .takes_value(true),
        )
        .get_matches();

    let items = get_items(matches.value_of("sort").unwrap())?;

    let entries = items.iter().map(|item| Entry {
        title: item.title().unwrap().to_string(),
        link: item.link().unwrap().to_string(),
        date: item.pub_date().unwrap().to_string(),
        author: item.author().unwrap_or_default().to_string(),
        content: item.description().unwrap_or_default().to_string(),
    });

    for entry in entries {
        print_entry(entry);
    }

    Ok(())
}
