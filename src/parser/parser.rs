use super::heading::heading;
use super::sentence::sentence;
use super::table::table;
use super::list::list;

#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, Words),
    Sentence(Words),
    Table(Box<Table>),
    List(Items),
}

#[derive(Debug, PartialEq)]
pub enum Word {
    Normal(String),
    Italic(Words),
    Bold(Words),
    StrikeThough(Words),
    Underline(Words),
}

#[derive(Debug, PartialEq)]
pub struct Item(pub Words, pub Items);

#[derive(Debug, PartialEq)]
pub struct Items(pub Vec<Item>);

#[derive(Debug, PartialEq)]
pub struct Words(pub Vec<Word>);

macro_rules! words {
    ( $( $word:expr), *) => {{
        let mut ws = vec!();
        $(
            ws.push($word);
        )*
        Words(ws) 
    }}
}

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
    let parsers = vec!(table, list, heading, sentence);
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
        let token = Words(vec!(Word::Normal("Hello World!".to_string())));
        let token = Words(vec!(Word::Bold(token)));
        let token = Word::Underline(token);
        let token = Words(vec!(token));
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "**__Hello World!__**";
        let token = Words(vec!(Word::Normal("Hello World!".to_string())));
        let token = Words(vec!(Word::Underline(token)));
        let token = Word::Bold(token);
        let token = Words(vec!(token));
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "~~**__Hello World!__**~~";
        let token = Words(vec!(Word::Normal("Hello World!".to_string())));
        let token = Words(vec!(Word::Underline(token)));
        let token = Words(vec!(Word::Bold(token)));
        let token = Word::StrikeThough(token);
        let token = Words(vec!(token));
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "Hello **World!**";
        let hello = Word::Normal("Hello ".to_string());
        let world = Word::Normal("World!".to_string());
        let world = Word::Bold(Words(vec!(world)));
        let token = Words(vec!(hello, world));
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "# Hello World!";
        let token = Words(vec!(Word::Normal("Hello World!".to_string())));
        assert_eq!(parse(&test_word), vec!(Md::Heading(1, token)));
    }

    #[test]
    fn test_parsing_multiline() {
        let test_word = "# Hello World!\nrust parser\n**lines**";
        let token = Words(vec!(Word::Normal("Hello World!".to_string())));
        let heading_token = Md::Heading(1, token);

        let s_token = Word::Normal("rust parser".to_string());
        let s_token = Words(vec!(s_token));
        let s_token = Md::Sentence(s_token);
        
        let b_token = Word::Normal("lines".to_string());
        let b_token = Word::Bold(Words(vec!(b_token)));
        let b_token = Words(vec!(b_token));
        let b_token = Md::Sentence(b_token);

        assert_eq!(parse(&test_word), vec!(heading_token, s_token, b_token));
    }
    #[test]
    fn test_table() {
        let test = "| A | B | C | \n|-:|--|:-:|\n| a | b | c |\n| j | k | l |\n";
        let a = Words(vec!(Word::Normal("A".to_string())));
        let b = Words(vec!(Word::Normal("B".to_string())));
        let c = Words(vec!(Word::Normal("C".to_string())));
        let he = Record(vec!(a, b, c));
    
        let al = vec!(Align::Right, Align::Left, Align::Center);
    
        let a = Words(vec!(Word::Normal("a".to_string())));
        let b = Words(vec!(Word::Normal("b".to_string())));
        let c = Words(vec!(Word::Normal("c".to_string())));
        let r1 = Record(vec!(a, b, c));
        let j = Words(vec!(Word::Normal("j".to_string())));
        let k = Words(vec!(Word::Normal("k".to_string())));
        let l = Words(vec!(Word::Normal("l".to_string())));
        let r2 = Record(vec!(j, k, l));
        let re = vec!(r1, r2);

        let t = Table{header: he, align: al, records: re};
        let t = Md::Table(Box::new(t));

        assert_eq!(parse(&test), vec!(t));
    }

    #[test]
    fn test_list() {
        let test_word = "- Hello\n  - World";
        let n = Word::Normal("World".to_string());
        let w = Words(vec!(n));
        let items0 = Items(vec!());
        let i0 = Item(w, items0);
        let child = Items(vec!(i0));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let token = Md::List(Items(vec!(i1)));
        assert_eq!(parse(&test_word), vec!(token));
    }

    #[test]
    fn test_macros() {
        let word0 = Word::Normal("hello".to_string());
        let word1 = Word::Normal("world".to_string());

        let hello = Word::Normal("hello".to_string());
        let world = Word::Normal("world".to_string());
        assert_eq!(words![word0, word1], Words(vec!(hello, world)));
    }
}