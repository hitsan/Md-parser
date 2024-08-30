use crate::parser::parser::*;
use super::line::words;

pub fn heading(text: &str) -> Option<Md> {
    ["#", "##", "###"].iter().find_map(|p| {
        let text = consume(text, p)?;
        let text = space(text)?;
        let tokens = words(&text);
        let ret = Md::Heading(p.len(), tokens);
        Some(ret)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(1, expectation)));

        let test_word = "#    Hello World!";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(1, expectation)));

        let test_word = "## Hello World!";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(2, expectation)));

        let test_word = "### Hello World!";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(3, expectation)));
    }

}