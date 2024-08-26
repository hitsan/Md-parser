use crate::parser::parser::*;
use super::line::terms;

pub fn heading(sentence: &str) -> Option<Md> {
    ["#", "##", "###"].iter().find_map(|p| {
        let sentence = consume(sentence, p)?;
        let sentence = space(sentence)?;
        let tokens = terms(&sentence);
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
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(1, expectation)));

        let test_word = "#    Hello World!";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(1, expectation)));

        let test_word = "## Hello World!";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(2, expectation)));

        let test_word = "### Hello World!";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        assert_eq!(heading(&test_word), Some(Md::Heading(3, expectation)));
    }

}