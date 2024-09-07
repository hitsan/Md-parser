use crate::parser::parser::*;
use super::sentence::words;

pub fn heading(texts: &str) -> Option<ParsedResult<Md>> {
    ["#", "##", "###"].iter().find_map(|p| {
        let (text, rest) = if let Some(n) = texts.find("\n") {
            (&texts[..n], &texts[(n+1)..])
        } else {
            (texts, "")
        };
        let text = consume(text, p)?;
        let text = space(text)?;
        let tokens = words(&text);
        let token = Md::Heading(p.len(), tokens);
        Some(ParsedResult::new(token, rest))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{words,normal_word};

    #[test]
    fn test_heading() {
        let words = words!(normal_word!("Hello World!"));
        let token = Md::Heading(1, words);
        assert_eq!(heading(&"# Hello World!"), Some(ParsedResult{token, rest: ""}));

        let words = words!(normal_word!("Hello World!"));
        let token = Md::Heading(1, words);
        assert_eq!(heading(&"#    Hello World!"), Some(ParsedResult{token, rest: ""}));

        let words = words!(normal_word!("Hello World!"));
        let token = Md::Heading(2, words);
        assert_eq!(heading(&"## Hello World!"), Some(ParsedResult{token, rest: ""}));

        let words = words!(normal_word!("Hello World!"));
        let token = Md::Heading(3, words);
        assert_eq!(heading(&"### Hello World!"), Some(ParsedResult{token, rest: ""}));
    }

    #[test]
    fn test_heading_multiline() {
        let words = words!(normal_word!("Hello "));
        let token = Md::Heading(1, words);
        assert_eq!(heading(&"# Hello \nWorld!"), Some(ParsedResult{token, rest: "World!"}));
    }

}