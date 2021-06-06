extern crate alpm;
extern crate clap;
use alpm::Alpm;
use clap::{App, Arg, ArgMatches};
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

fn get_items(sort: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(FEED_URL)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    match sort {
        "desc" => Ok(channel.into_items().into_iter().rev().collect()),
        _ => Ok(channel.into_items()),
    }
}

fn map_rss_items_to_entries(items: Vec<Item>) -> Vec<Entry> {
    items
        .iter()
        .map(|item| Entry {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            date: item.pub_date().unwrap().to_string(),
            author: item.author().unwrap_or_default().to_string(),
            content: item.description().unwrap_or_default().to_string(),
        })
        .collect()
}

fn get_pacman_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pacman = Alpm::new("/", "/var/lib/pacman")?;
    let db = pacman.localdb();
    let packages = db.pkgs().iter().map(|p| p.name().to_string()).collect();
    Ok(packages)
}

fn get_config_options(config: ArgMatches) -> (String, bool) {
    (
        config.value_of("sort").unwrap().to_string(),
        config.is_present("lookup"),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = App::new("pacnews")
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
        .arg(
            Arg::with_name("lookup")
                .short("l")
                .long("lookup")
                .help("Perform a lookup based on installed package names")
                .takes_value(false),
        )
        .get_matches();

    let (sort, lookup) = get_config_options(config);

    let items = get_items(&sort)?;

    let entries = match lookup {
        true => Vec::new(),
        false => map_rss_items_to_entries(items),
    };

    for entry in entries {
        print_entry(entry);
    }

    Ok(())
}
