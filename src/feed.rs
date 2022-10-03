use crate::Sort;
use rss::{Channel, Item};

const FEED_URL: &str = "https://www.archlinux.org/feeds/news/";

pub fn get_items(sort: &Sort) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(FEED_URL)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    match sort {
        Sort::Desc => Ok(channel.into_items().into_iter().rev().collect()),
        _ => Ok(channel.into_items()),
    }
}
