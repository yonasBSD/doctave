use std::collections::BTreeMap;

pub fn parse(input: &str) -> std::io::Result<BTreeMap<String, String>> {
    let pos = end_pos(input);

    if pos > 0 {
        serde_yaml::from_str(input[0..pos].trim_end().trim_end_matches('-'))
            .map_err(std::io::Error::other)
    } else {
        Ok(BTreeMap::new())
    }
}

pub fn end_pos(input: &str) -> usize {
    if let Some(after_starter_mark) = input.strip_prefix("---\n") {
        let end_mark = after_starter_mark.find("---\n");

        if let Some(item) = end_mark {
            item + 8
        } else {
            0
        }
    } else if let Some(after_starter_mark) = input.strip_prefix("---\r\n") {
        let end_mark = after_starter_mark.find("---\r\n");

        if let Some(item) = end_mark {
            item + 10
        } else {
            0
        }
    } else {
        0
    }
}

pub fn without(input: &str) -> &str {
    &input[end_pos(input)..]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_parse() {
        let input = indoc! {"
            ---
            title: Runbooks
            ---

            # Runbooks
        "};

        let values = parse(input).unwrap();

        let expected = Some("Runbooks".to_owned());
        let actual = values.get("title");

        assert_eq!(actual, expected.as_ref());
    }

    #[test]
    fn missing_frontmatter() {
        let input = indoc! {"
            # Some content
        "};

        let values = parse(input).unwrap();

        assert_eq!(values, BTreeMap::new());
    }

    #[test]
    fn invalid_yaml() {
        let input = indoc! {"
            ---
            :::blarg: @@!~
            ---

            # Some content
        "};

        assert!(parse(input).is_err());
    }

    #[test]
    fn never_ending_frontmatter() {
        let input = indoc! {"
            ---
            title: Runbooks

            # Runbooks
        "};

        assert_eq!(parse(input).unwrap(), BTreeMap::new());
    }

    #[test]
    fn without_basic() {
        let input = indoc! {"
            ---
            title: Runbooks
            ---

            # Runbooks
        "};

        let without_frontmatter = without(input);

        assert_eq!(without_frontmatter, "\n# Runbooks\n");
    }

    #[test]
    fn windows_line_endings() {
        let input = "---\r\ntitle: Runbooks\r\n---\r\n\r\n# More content\r\n";

        let values = parse(input).unwrap();

        let expected = Some("Runbooks".to_owned());
        let actual = values.get("title");

        assert_eq!(actual, expected.as_ref());

        let without_frontmatter = without(input);

        assert_eq!(without_frontmatter, "\r\n# More content\r\n");
    }
}
