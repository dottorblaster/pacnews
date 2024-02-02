use crate::Sort;
use rss::{Channel, Item};

pub fn get_items(feed_url: &str, sort: &Sort) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(feed_url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    match sort {
        Sort::Desc => Ok(channel.into_items().into_iter().rev().collect()),
        _ => Ok(channel.into_items()),
    }
}
