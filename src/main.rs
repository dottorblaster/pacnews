extern crate clap;
use clap::{App};
use rss::Channel;

const FEED_URL: &str = "https://www.archlinux.org/feeds/news/";

pub struct Entry {
    pub title: String,
    pub link: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

fn print_entry(entry: Entry) -> () {
    println!(
        "Title: {}\nPosted: {}\nLink: {}\n{}\n\n",
        entry.title, entry.date, entry.link, entry.content,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _matches = App::new("pacnews")
        .version("2.1.0")
        .author("Alessio Biancalana <dottorblaster@gmail.com>")
        .about("Read Arch Linux news feed directly from your terminal").get_matches();

    let content = reqwest::blocking::get(FEED_URL)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    let items = channel.into_items();
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
