extern crate alpm;
extern crate clap;
use clap::{App, Arg};

mod config;
mod entry;
mod feed;
mod pacman;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = App::new("pacnews")
        .version(clap::crate_version!())
        .author("Alessio Biancalana <dottorblaster@gmail.com>")
        .about("Read Arch Linux news feed directly from your terminal")
        .arg(
            Arg::with_name("sort")
                .short('s')
                .long("sort")
                .help("Sort by date")
                .value_name("asc/desc")
                .possible_values(&["asc", "desc"])
                .default_value("asc")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lookup")
                .short('l')
                .long("lookup")
                .help("Perform a lookup based on installed package names")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("colors")
                .short('c')
                .long("colors")
                .help("Enable colored output for entries")
                .takes_value(false),
        )
        .get_matches();

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
            entry::print_colored_entry(entry);
        } else {
            entry::print_entry(entry);
        }
    }

    Ok(())
}
