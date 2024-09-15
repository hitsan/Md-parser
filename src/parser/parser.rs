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
#[macro_export]
macro_rules! normal_word {
    ($text:expr) => {{
        Word::Normal($text.to_string())
    }};
}

#[derive(Debug, PartialEq)]
pub struct Item(pub Words, pub Items);

#[derive(Debug, PartialEq)]
pub struct Items(pub Vec<Item>);
#[macro_export]
macro_rules! items {
    () => {{
        Items(vec!()) 
    }};

    ( $( $item:expr), *) => {{
        let mut is = vec!();
        $(
            is.push($item);
        )*
        Items(is) 
    }};
}

#[derive(Debug, PartialEq)]
pub struct Words(pub Vec<Word>);
#[macro_export]
macro_rules! words {
    () => {{
        panic!("No words!")
    }};

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
#[macro_export]
macro_rules! record {
    () => {{
        panic!("No Vec<words>!")
    }};

    ( $( $words:expr), *) => {{
        let mut rd = vec!();
        $(
            rd.push($words);
        )*
        Record(rd) 
    }}
}

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

pub fn split_first_pattern<'a>(texts: &'a str, pattern: &str) -> (&'a str, &'a str) {
    if let Some(n) = texts.find(pattern) {
        let len = pattern.len();
        (&texts[..n], &texts[(n+len)..])
    } else {
        (texts, "")
    }
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
