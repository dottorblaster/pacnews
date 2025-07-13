extern crate alpm;
extern crate clap;
use clap::{ArgAction, Parser, ValueEnum};

mod config;
mod entry;
mod feed;
mod pacman;

static DB_PATH: &str = "/var/lib/pacman";
static FEED_URL: &str = "https://www.archlinux.org/feeds/news/";

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Read Arch Linux news feed directly from your terminal",
    long_about = None
)]
pub struct Args {
    #[arg(short, long, value_enum, value_parser, default_value_t=Sort::Asc, help = "Sort by date")]
    sort: Sort,
    #[clap(
        short,
        long,
        help = "Perform a lookup based on installed package names"
    )]
    lookup: bool,
    #[arg(short, long, action=ArgAction::SetFalse, help = "Enable colored output for entries")]
    colors: bool,
    #[arg(short, long, help = "Number of news articles to print")]
    number: Option<u16>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Sort {
    Asc,
    Desc,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Args::parse();

    let (sort, lookup, colors, count) = config::get_config_options(config);

    let items = feed::get_items(FEED_URL, &sort)?;

    let entries = match lookup {
        true => {
            let total_entries = entry::map_rss_items_to_entries(items);
            total_entries
                .iter()
                .filter(|entry| pacman::has_package_name(DB_PATH, &entry.title))
                .cloned()
                .collect()
        }
        false => entry::map_rss_items_to_entries(items),
    };

    let entries = count
        .map(|c| &entries[..(c as usize).min(entries.len())])
        .unwrap_or(&entries);

    for entry in entries.iter().cloned() {
        if colors {
            let colored_entry = entry::ColoredEntry::from(entry);
            println!("{colored_entry}")
        } else {
            println!("{entry}")
        }
    }

    Ok(())
}
