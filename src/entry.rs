use crate::markdown;
use rss::Item;

#[derive(Clone)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

pub fn print_entry(entry: Entry) {
    let markdown_content = html2text::from_read(entry.content.as_bytes(), 250);
    println!(
        "Title: {}\nPosted: {}\nLink: {}\n{}",
        entry.title, entry.date, entry.link, markdown_content,
    );
}

pub fn map_rss_items_to_entries(items: Vec<Item>) -> Vec<Entry> {
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
