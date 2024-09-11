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
        use crate::parser::parser::Word;
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
        use crate::parser::parser::Items;
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
        use crate::parser::parser::Words;
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
        use crate::parser::parser::Record;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let normal = words!(normal_word!("Hello World!"));
        let bold = words!(Word::Bold(normal));
        let bold_line = Word::Underline(bold);
        let words = words!(bold_line);
        let md = Md::Sentence(words);
        assert_eq!(parse(&"__**Hello World!**__"), vec!(md));

        let normal = words!(normal_word!("Hello World!"));
        let line = words!(Word::Underline(normal));
        let line_bold = Word::Bold(line);
        let words = words!(line_bold);
        let md = Md::Sentence(words);
        assert_eq!(parse(&"**__Hello World!__**"), vec!(md));

        let normal = words!(normal_word!("Hello World!"));
        let line_normal = words!(Word::Underline(normal));
        let bold_line_normal = words!(Word::Bold(line_normal));
        let strike_bold_line_normal = words!(Word::StrikeThough(bold_line_normal));
        let md = Md::Sentence(strike_bold_line_normal);
        assert_eq!(parse(&"~~**__Hello World!__**~~"), vec!(md));

        let hello = normal_word!("Hello ");
        let world = Word::Bold(words!(normal_word!("World!")));
        let word = words!(hello, world);
        let md = Md::Sentence(word);
        assert_eq!(parse(&"Hello **World!**"), vec!(md));

        let normal = words!(normal_word!("Hello World!"));
        let md = Md::Heading(1, normal);
        assert_eq!(parse(&"# Hello World!"), vec!(md));
    }

    #[test]
    fn test_parsing_multiline() {
        let hello_world = words!(normal_word!("Hello World!"));
        let head: Md = Md::Heading(1, hello_world);

        let words = words!(normal_word!("rust parser"));
        let sentence = Md::Sentence(words);
        
        let bold = words!(Word::Bold(words!(normal_word!("lines"))));
        let bold_sentence = Md::Sentence(bold);

        let mds = vec!(head, sentence, bold_sentence);
        assert_eq!(parse(&"# Hello World!\nrust parser\n**lines**"), mds);
    }
    #[test]
    fn test_table() {
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let header = Record(vec!(a, b, c));
        let align = vec!(Align::Right, Align::Left, Align::Center);
        let d = words!(normal_word!("d"));
        let e = words!(normal_word!("e"));
        let f = words!(normal_word!("f"));
        let record0 = Record(vec!(d, e, f));
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let record1 = Record(vec!(j, k, l));
        let records = vec!(record0, record1);
        let md = Md::Table(Box::new(Table{header, align, records}));
        let test_word = "| A | B | C | \n|-:|--|:-:|\n| d | e | f |\n| j | k | l |\n";
        assert_eq!(parse(&test_word), vec!(md));
    }

    #[test]
    fn test_list() {
        let world = words!(normal_word!("World"));
        let item0 = Item(world, items!());
        let children = items!(item0);
        let hello = words!(normal_word!("Hello"));
        let item = Item(hello, children);
        let md = Md::List(items!(item));
        assert_eq!(parse(&"- Hello\n  - World"), vec!(md));
    }
}