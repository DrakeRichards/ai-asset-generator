/// Get the text of the first H1 heading in a Markdown document.
pub fn get_first_h1_heading(markdown: &str) -> Option<String> {
    let first_h1_heading: String = markdown
        .lines()
        .find(|line| line.starts_with("# "))?
        .trim_start_matches("# ")
        .to_string();
    Some(first_h1_heading)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn test_get_first_h1_heading() {
        let markdown: &str = "# First Heading\n\nSome text.";
        let first_h1_heading: String = get_first_h1_heading(markdown).unwrap();
        assert_eq!(first_h1_heading, "First Heading");
    }
}
