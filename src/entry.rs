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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rss_items_mapping() {
        let mut first_item = Item::default();
        first_item.set_title("How to cook a carbonara".to_owned());
        first_item.set_link("https://dottorblaster.it/carbonara".to_owned());
        first_item.set_pub_date("Sun, 01 Jan 2017 12:00:00 GMT".to_owned());
        first_item.set_author("Alessio Biancalana".to_owned());
        first_item.set_description("Let's cook the carbonara".to_owned());

        let mut second_item = Item::default();
        second_item.set_title("How to cook an amatriciana".to_owned());
        second_item.set_link("https://dottorblaster.it/amatriciana".to_owned());
        second_item.set_pub_date("Sun, 01 Jan 2021 12:00:00 GMT".to_owned());
        second_item.set_author("Alessio Biancalana".to_owned());
        second_item.set_description("Let's cook the amatriciana".to_owned());

        let items_list = vec![first_item.clone(), second_item.clone()];

        let result = map_rss_items_to_entries(items_list);

        assert_eq!(result.get(0).unwrap().title, first_item.title().unwrap());
        assert_eq!(result.get(0).unwrap().link, first_item.link().unwrap());
        assert_eq!(result.get(0).unwrap().date, first_item.pub_date().unwrap());
        assert_eq!(
            result.get(0).unwrap().content,
            first_item.description().unwrap()
        );

        assert_eq!(result.get(1).unwrap().title, second_item.title().unwrap());
        assert_eq!(result.get(1).unwrap().link, second_item.link().unwrap());
        assert_eq!(result.get(1).unwrap().date, second_item.pub_date().unwrap());
        assert_eq!(
            result.get(1).unwrap().content,
            second_item.description().unwrap()
        );
    }

    #[test]
    fn test_colored_entry_from_entry() {
        let entry = Entry {
            title: "Hellow".to_owned(),
            link: "https://dottorblaster.it".to_owned(),
            date: "26 Jun 2023".to_owned(),
            author: "Alessio Biancalana".to_owned(),
            content: "Now we are testing pacnews thoroughly".to_owned(),
        };

        let colored_entry = ColoredEntry::from(entry.clone());

        assert_eq!(entry.title, colored_entry.title);
        assert_eq!(entry.date, colored_entry.date);
        assert_eq!(entry.author, colored_entry.author);
        assert_eq!(entry.content, colored_entry.content);
    }
}
