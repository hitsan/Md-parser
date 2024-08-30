use super::heading::heading;
use super::line::line;

#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, Vec<Word>),
    Sentence(Vec<Word>),
}

#[derive(Debug, PartialEq)]
pub enum Word {
    Normal(String),
    Italic(Vec<Word>),
    Bold(Vec<Word>),
    StrikeThough(Vec<Word>),
    Underline(Vec<Word>),
}

pub struct Words(Vec<Word>);

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedResult<'a, T> {
    pub token: T,
    pub rest: &'a str,
}

impl<'a, T> ParsedResult<'a, T> {
    pub fn new(token: T, rest: &'a str) -> ParsedResult<'a, T> {
        ParsedResult { token: token, rest: rest }
    }
}

pub fn space(text: &str) -> Option<&str> {
    let text = consume(text, " ")?;
    Some(text.trim_start())
}

pub fn consume<'a>(text: &'a str, pattern: &'a str) -> Option<&'a str> {
    if !text.starts_with(pattern) { return None }
    let length = pattern.len();
    Some(&text[length..])
}

pub fn parse(text: &str) -> Md {
    let parsers = vec!(heading, line);
    let ret = parsers.iter().find_map(|f| f(text));
    ret.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let test_word = "__**Hello World!**__";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Bold(expectation));
        let expectation = Word::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "**__Hello World!__**";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Underline(expectation));
        let expectation = Word::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "~~**__Hello World!__**~~";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Underline(expectation));
        let expectation = vec!(Word::Bold(expectation));
        let expectation = Word::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "Hello **World!**";
        let hello = Word::Normal("Hello ".to_string());
        let world = Word::Normal("World!".to_string());
        let world = Word::Bold(vec!(world));
        let expectation = vec!(hello, world);
        let expectation = Md::Sentence(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "# Hello World!";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(parse(&test_word), Md::Heading(1, expectation));
    }

}