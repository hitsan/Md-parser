use crate::parser::parser::*;
use super::line::words;

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

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Md::Heading(1, token);
        assert_eq!(heading(&test_word), Some(ParsedResult{token, rest: ""}));

        let test_word = "#    Hello World!";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Md::Heading(1, token);
        assert_eq!(heading(&test_word), Some(ParsedResult{token, rest: ""}));

        let test_word = "## Hello World!";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Md::Heading(2, token);
        assert_eq!(heading(&test_word), Some(ParsedResult{token, rest: ""}));

        let test_word = "### Hello World!";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Md::Heading(3, token);
        assert_eq!(heading(&test_word), Some(ParsedResult{token, rest: ""}));
    }

    #[test]
    fn test_heading_multiline() {
        let test_word = "# Hello \nWorld!";
        let token = vec!(Word::Normal("Hello ".to_string()));
        let token = Md::Heading(1, token);
        assert_eq!(heading(&test_word), Some(ParsedResult{token, rest: "World!"}));
    }

}