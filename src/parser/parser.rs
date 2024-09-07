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
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Bold(token));
        let token = Word::Underline(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "**__Hello World!__**";
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Underline(token));
        let token = Word::Bold(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "~~**__Hello World!__**~~";
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Underline(token));
        let token = words!(Word::Bold(token));
        let token = Word::StrikeThough(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "Hello **World!**";
        let hello = normal_word!("Hello ".to_string());
        let world = normal_word!("World!".to_string());
        let world = Word::Bold(words!(world));
        let token = words!(hello, world);
        let token = Md::Sentence(token);
        assert_eq!(parse(&test_word), vec!(token));

        let test_word = "# Hello World!";
        let token = words!(normal_word!("Hello World!"));
        assert_eq!(parse(&test_word), vec!(Md::Heading(1, token)));
    }

    #[test]
    fn test_parsing_multiline() {
        let test_word = "# Hello World!\nrust parser\n**lines**";
        let token = words!(normal_word!("Hello World!"));
        let heading_token = Md::Heading(1, token);

        let s_token = normal_word!("rust parser".to_string());
        let s_token = words!(s_token);
        let s_token = Md::Sentence(s_token);
        
        let b_token = normal_word!("lines".to_string());
        let b_token = Word::Bold(words!(b_token));
        let b_token = words!(b_token);
        let b_token = Md::Sentence(b_token);

        assert_eq!(parse(&test_word), vec!(heading_token, s_token, b_token));
    }
    #[test]
    fn test_table() {
        let test = "| A | B | C | \n|-:|--|:-:|\n| a | b | c |\n| j | k | l |\n";
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let he = Record(vec!(a, b, c));
    
        let al = vec!(Align::Right, Align::Left, Align::Center);
    
        let a = words!(normal_word!("a"));
        let b = words!(normal_word!("b"));
        let c = words!(normal_word!("c"));
        let r1 = Record(vec!(a, b, c));
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let r2 = Record(vec!(j, k, l));
        let re = vec!(r1, r2);

        let t = Table{header: he, align: al, records: re};
        let t = Md::Table(Box::new(t));

        assert_eq!(parse(&test), vec!(t));
    }

    #[test]
    fn test_list() {
        let test_word = "- Hello\n  - World";
        let n = normal_word!("World");
        let w = words!(n);
        let items0 = Items(vec!());
        let i0 = Item(w, items0);
        let child = Items(vec!(i0));
        let n = normal_word!("Hello");
        let w = words!(n);
        let i1 = Item(w, child);

        let token = Md::List(Items(vec!(i1)));
        assert_eq!(parse(&test_word), vec!(token));
    }

    #[test]
    fn test_macros() {
        let word0 = normal_word!("hello");
        let word1 = normal_word!("world");

        let hello = normal_word!("hello");
        let world = normal_word!("world");
        assert_eq!(words!(word0, word1), words!(hello, world));
    }
}