extern crate alpm;
extern crate clap;
use clap::{Parser, ValueEnum};

mod config;
mod entry;
mod feed;
mod pacman;

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
    #[arg(short, long, help = "Enable colored output for entries")]
    colors: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Sort {
    Asc,
    Desc,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Args::parse();

    let (sort, lookup, colors) = config::get_config_options(config);

    let items = feed::get_items(&sort)?;

    let entries = match lookup {
        true => {
            let total_entries = entry::map_rss_items_to_entries(items);
            total_entries
                .iter()
                .filter(|entry| pacman::has_package_name(&entry.title))
                .cloned()
                .collect()
        }
        false => entry::map_rss_items_to_entries(items),
    };

    for entry in entries {
        if colors {
            let colored_entry = entry::ColoredEntry::from(entry);
            println!("{}", colored_entry)
        } else {
            println!("{}", entry)
        }
    }

    Ok(())
}
