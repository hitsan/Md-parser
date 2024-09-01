use super::heading::heading;
use super::line::sentence;

#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, Vec<Word>),
    Sentence(Vec<Word>),
    Table(Box<Table>),
}

#[derive(Debug, PartialEq)]
pub enum Word {
    Normal(String),
    Italic(Vec<Word>),
    Bold(Vec<Word>),
    StrikeThough(Vec<Word>),
    Underline(Vec<Word>),
}

#[derive(Debug, PartialEq)]
pub struct Words(pub Vec<Word>);

#[derive(Debug, PartialEq)]
pub struct Record(pub Vec<Words>);

#[derive(Debug, PartialEq)]
pub struct Table {
    pub header: Record,
    pub align: Vec<Align>,
    pub records: Vec<Record>,
}

#[derive(Debug, PartialEq)]
pub enum Align {
    Right,
    Center,
    Left
}

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

pub fn parse(mut text: &str) -> Vec<Md> {
    let parsers = vec!(heading, sentence);
    let mut md: Vec<Md> = vec!();
    while let Some(ret) = parsers.iter().find_map(|f| f(text)) {
        md.push(ret.token);
        text = ret.rest;
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let test_word = "__**Hello World!**__";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Bold(token));
        let token = Word::Underline(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "**__Hello World!__**";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Underline(token));
        let token = Word::Bold(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "~~**__Hello World!__**~~";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Underline(token));
        let token = vec!(Word::Bold(token));
        let token = Word::StrikeThough(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "Hello **World!**";
        let hello = Word::Normal("Hello ".to_string());
        let world = Word::Normal("World!".to_string());
        let world = Word::Bold(vec!(world));
        let token = vec!(hello, world);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "# Hello World!";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        assert_eq!(parse(&test_word), vec!(Md::Heading(1, token)));
    }

    #[test]
    fn test_parsing_multiline() {
        let test_word = "# Hello World!\nrust parser\n**lines**";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let heading_token = Md::Heading(1, token);

        let s_token = Word::Normal("rust parser".to_string());
        let s_token = vec!(s_token);
        let s_token = Md::Sentence(s_token);
        
        let b_token = Word::Normal("lines".to_string());
        let b_token = Word::Bold(vec!(b_token));
        let b_token = vec!(b_token);
        let b_token = Md::Sentence(b_token);

        assert_eq!(parse(&test_word), vec!(heading_token, s_token, b_token));
    }

}