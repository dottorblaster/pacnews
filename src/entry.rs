use colored::Colorize;
use rss::Item;
use std::fmt;

#[derive(Clone)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let markdown_content = html2text::from_read(self.content.as_bytes(), 250);
        write!(
            f,
            "Title: {}\nPosted: {}\nLink: {}\n{}",
            self.title,
            self.date,
            self.link,
            termimad::inline(&markdown_content)
        )
    }
}

#[derive(Clone)]
pub struct ColoredEntry {
    pub title: String,
    pub link: String,
    pub date: String,
    pub author: String,
    pub content: String,
}

impl From<Entry> for ColoredEntry {
    fn from(entry: Entry) -> Self {
        ColoredEntry {
            title: entry.title,
            link: entry.link,
            date: entry.date,
            author: entry.author,
            content: entry.content,
        }
    }
}

impl fmt::Display for ColoredEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let markdown_content = html2text::from_read(self.content.as_bytes(), 250);
        write!(
            f,
            "Title: {}\nPosted: {}\nLink: {}\n{}",
            self.title.cyan(),
            self.date.yellow(),
            self.link.purple(),
            termimad::inline(&markdown_content)
        )
    }
}

pub fn map_rss_items_to_entries(items: Vec<Item>) -> Vec<Entry> {
    items
        .iter()
        .map(|item| Entry {
            title: item.title().unwrap_or_default().to_string(),
            link: item.link().unwrap_or_default().to_string(),
            date: item.pub_date().unwrap_or_default().to_string(),
            author: item.author().unwrap_or_default().to_string(),
            content: item.description().unwrap_or_default().to_string(),
        })
        .collect()
}
