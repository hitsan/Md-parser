use super::heading::heading;
use super::line::line;

#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, Vec<Emphasis>),
    Line(Vec<Emphasis>),
    Table(usize, TableHeader, TableData)
}

#[derive(Debug, PartialEq)]
pub struct TableHeader(pub Vec<Vec<Emphasis>>);

#[derive(Debug, PartialEq)]
pub struct TableData(pub Vec<Vec<Vec<Emphasis>>>);

#[derive(Debug, PartialEq)]
pub enum Emphasis {
    Text(String),
    Italic(Vec<Emphasis>),
    Bold(Vec<Emphasis>),
    StrikeThough(Vec<Emphasis>),
    Underline(Vec<Emphasis>),
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

pub fn space(sentence: &str) -> Option<&str> {
    let sentence = consume(sentence, " ")?;
    Some(sentence.trim_start())
}

pub fn consume<'a>(sentence: &'a str, pattern: &'a str) -> Option<&'a str> {
    if !sentence.starts_with(pattern) { return None }
    let length = pattern.len();
    Some(&sentence[length..])
}

pub fn parse(sentence: &str) -> Md {
    let parsers = vec!(heading, line);
    let ret = parsers.iter().find_map(|f| f(sentence));
    ret.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let test_word = "__**Hello World!**__";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "**__Hello World!__**";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = Emphasis::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "~~**__Hello World!__**~~";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "Hello **World!**";
        let hello = Emphasis::Text("Hello ".to_string());
        let world = Emphasis::Text("World!".to_string());
        let world = Emphasis::Bold(vec!(world));
        let expectation = vec!(hello, world);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), expectation);

        let test_word = "# Hello World!";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        assert_eq!(parse(&test_word), Md::Heading(1, expectation));
    }

}