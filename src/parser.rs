#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, String),
    Line(Emphasis)
}

#[derive(Debug, PartialEq)]
pub enum Emphasis {
    Text(String),
    Italic(Box<Emphasis>),
    Bold(Box<Emphasis>),
    StrikeThough(Box<Emphasis>),
    Underline(Box<Emphasis>),
    Inline(Box<Emphasis>),
}

#[derive(Debug, PartialEq)]
pub struct ParsedResult<'a, T> {
    token: T,
    rest: &'a str,
}

impl<'a, T> ParsedResult<'a, T> {
    pub fn new(token: T, rest: &'a str) -> ParsedResult<'a, T> {
        ParsedResult { token: token, rest: rest }
    }
}

fn heading(sentence: &str) -> Option<ParsedResult<Md>> {
    ["# ", "## ", "### "].iter().enumerate().find_map(|p| {
        if !sentence.starts_with(p.1) { return None }
        let word = sentence[(p.0+2)..].to_string();
        let ret = ParsedResult::new(Md::Heading(p.0+1, word), &"");
        Some(ret)
    })
    // and_some
}

fn italic(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    if !sentence.starts_with("*") { return None }
    sentence[1..].find("*").and_then(|n| {
        let s = text(&sentence[1..(n+1)]).unwrap();
        let ret = ParsedResult::new(Emphasis::Italic(Box::new(s.token)), &sentence[(n+2)..]);
        Some(ret)
    })
}

fn bold(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    if !sentence.starts_with("**") { return None }
    sentence[2..].find("**").and_then(|n| {
        let s = text(&sentence[2..(n+2)]).unwrap();
        let ret = ParsedResult::new(Emphasis::Bold(Box::new(s.token)), &sentence[(n+4)..]);
        Some(ret)
    })
}

fn underline(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    if !sentence.starts_with("__") { return None }
    sentence[2..].find("__").and_then(|n| {
        let s = text(&sentence[2..(n+2)]).unwrap();
        let ret = ParsedResult::new(Emphasis::Underline(Box::new(s.token)), &sentence[(n+4)..]);
        Some(ret)
    })
}

fn strike_though(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    if !sentence.starts_with("~~") { return None }
    sentence[2..].find("~~").and_then(|n| {
        let s = text(&sentence[2..(n+2)]).unwrap();
        let ret = ParsedResult::new(Emphasis::StrikeThough(Box::new(s.token)), &sentence[(n+4)..]);
        Some(ret)
    })
}

fn text(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let sentence = sentence.to_string();
    let ret = ParsedResult::new(Emphasis::Text(sentence), &"");
    Some(ret)
}

fn line(sentence: &str) -> Option<ParsedResult<Md>> {
    let parsers = vec!(underline, strike_though, bold, italic, text);
    parsers.iter().find_map(|f| f(sentence).and_then(
        |r| Some(ParsedResult::new(Md::Line(r.token), &""))
    ))
}

pub fn parse(sentence: &str) -> ParsedResult<Md> {
    let parsers = vec!(heading, line);
    let ret = parsers.iter().find_map(|f| f(sentence));
    ret.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let test_word = "Hello World!";
        let md_ans = Emphasis::Text("Hello World!".to_string());
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Line(md_ans), rest: &""});
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let md_ans = "Hello World!".to_string();
        let ans = parse(&test_word);
        assert_eq!(ans, ParsedResult{ token: Md::Line(Emphasis::Text(md_ans)), rest: &""});
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let md_ans = Box::new(Emphasis::Text("Hello World!".to_string()));
        let ans = parse(&test_word);
        assert_eq!(ans, ParsedResult{ token: Md::Line(Emphasis::Italic(md_ans)), rest: &""});
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let md_ans = Box::new(Emphasis::Text("Hello World!".to_string()));
        let ans = parse(&test_word);
        assert_eq!(ans, ParsedResult{ token: Md::Line(Emphasis::Bold(md_ans)), rest: &""});
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let md_ans = Box::new(Emphasis::Text("Hello World!".to_string()));
        let ans = parse(&test_word);
        assert_eq!(ans, ParsedResult{ token: Md::Line(Emphasis::StrikeThough(md_ans)), rest: &""});
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let md_ans = Box::new(Emphasis::Text("Hello World!".to_string()));
        let ans = parse(&test_word);
        assert_eq!(ans, ParsedResult{ token: Md::Line(Emphasis::Underline(md_ans)), rest: &""});
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let md_ans = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Heading(1, md_ans), rest: &""});
    }

}