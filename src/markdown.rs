pub fn to_markdown(string: String) -> String {
    let pairs = vec![
        ("<b>", "**"),
        ("</b>", "**"),
        ("<strong>", "**"),
        ("</strong>", "**"),
        ("<i>", "_"),
        ("</i>", "_"),
        ("<em>", "_"),
        ("</em>", "_"),
        ("<p>", ""),
        ("</p>", "\n\n"),
        ("<ul>", "\n"),
        ("</ul>", "\n"),
        ("<li>", "- "),
        ("</li>", "\n"),
    ];
    pairs
        .iter()
        .fold(string, |accumulator, (html_entity, markdown_entity)| {
            accumulator.replace(html_entity, markdown_entity)
        })
}

pub fn strip_new_lines(content: String) -> String {
    // TODO implement a more reasonable parsing mechanic for newlines and tags
    content.replace("\n", " ").replace("</p> <p>", "</p><p>")
}

#[cfg(test)]
mod tests {
    #[test]
    fn paragraph_test() {
        let text = "<p>hello world</p>".to_string();
        let result = "hello world\n\n";
        assert_eq!(crate::markdown::to_markdown(text), result);
    }

    #[test]
    fn bold_test() {
        let text = "<b>hello world</b>".to_string();
        let result = "**hello world**";
        assert_eq!(crate::markdown::to_markdown(text), result);
    }

    #[test]
    fn italic_test() {
        let text = "<em>hello world</em>".to_string();
        let result = "_hello world_";
        assert_eq!(crate::markdown::to_markdown(text), result);
    }

    #[test]
    fn mixed_entities_test() {
        let text = "<p><em>hello</em> <b>world</b></p>".to_string();
        let result = "_hello_ **world**\n\n";
        assert_eq!(crate::markdown::to_markdown(text), result);
    }

    #[test]
    fn list_test() {
        let text = "<ul><li><a href=\"https://archlinux.org/packages/extra/x86_64/php7-apcu/\">php7-apcu</a></li><li><a href=\"https://archlinux.org/packages/community/x86_64/php7-geoip/\">php7-geoip</a></li></ul>".to_string();
        let result = "\n- <a href=\"https://archlinux.org/packages/extra/x86_64/php7-apcu/\">php7-apcu</a>\n- <a href=\"https://archlinux.org/packages/community/x86_64/php7-geoip/\">php7-geoip</a>\n\n";
        assert_eq!(crate::markdown::to_markdown(text), result);
    }
}
