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
    println!(
        "Title: {}\nPosted: {}\nLink: {}\n{}\n\n",
        entry.title, entry.date, entry.link, entry.content,
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
