#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Hr,
    H1(&'a str),
    H2(&'a str),
    H3(&'a str),
    Word(&'a str),
    Italic(&'a str),
    Bold(&'a str),
    Bar(&'a str),
    Link(&'a str),
    // Line(Vec<Token<'a>, Box<Line>>),
    // Line(Vec<>)
}

fn start<'a>(line: &'a str, pattern: &'a str) -> Option<&'a str> {
    if line.starts_with(pattern) {
        let length = pattern.len();
        let rest = &line[length..];
        Some(rest)
    } else {
        None
    }
}

fn end<'a>(line: &'a str, pattern: &'a str) -> Option<&'a str> {
    if line.ends_with(pattern) {
        let length = line.len() - pattern.len();
        let rest = &line[..length];
        Some(rest)
    } else {
        None
    }
}

fn both_edhes<'a>(line: &'a str, pattern: &'a str) -> Option<&'a str> {
    start(&line, pattern).and_then(|rest| end(rest, pattern))
}

fn word(line: &str) -> Option<Token> {
    Some(Token::Word(&line))
}

fn h1(line: &str) -> Option<Token> {
    start(&line, "# ").and_then(|rest| Some(Token::H1(rest)))
}

fn h2(line: &str) -> Option<Token> {
    start(&line, "## ").and_then(|rest| Some(Token::H2(rest)))
}

fn h3(line: &str) -> Option<Token> {
    start(&line, "### ").and_then(|rest| Some(Token::H3(rest)))
}

fn hr(line: &str) -> Option<Token> {
    if line == "---" { Some(Token::Hr) } else { None }
}

fn italic(line: &str) -> Option<Token> {
    both_edhes(&line, "*").and_then(|rest| Some(Token::Italic(rest)))
}

fn bold(line: &str) -> Option<Token> {
    both_edhes(&line, "**").and_then(|rest| Some(Token::Bold(rest)))
}

fn bar(line: &str) -> Option<Token> {
    both_edhes(&line, "~~").and_then(|rest| Some(Token::Bar(rest)))
}

fn link(line: &str) -> Option<Token> {
    let parsed_line = start(line, "<").and_then(|rest| end(rest, ">"));
    parsed_line.and_then(|rest| Some(Token::Link(rest)))
}

pub fn parse(line: &str) -> Token {
    let parsers = vec!(h3, h2, h1, hr, bold, italic, bar, link, word);
    let ret = parsers.iter().find_map(|f| f(line));
    ret.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        let test_word = "Hello World!";
        assert_eq!(parse(&test_word), Token::Word("Hello World!"));
    }

    #[test]
    fn test_ends() {
        let test_word = "Hello World!";
        let pattern = "!";
        assert_eq!(end(&test_word, &pattern), Some("Hello World"));
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        assert_eq!(parse(&test_word), Token::H1("Hello World!"));
        let test_word = "## Hello World!";
        assert_eq!(parse(&test_word), Token::H2("Hello World!"));
        let test_word = "### Hello World!";
        assert_eq!(parse(&test_word), Token::H3("Hello World!"));
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        assert_eq!(parse(&test_word), Token::Italic("Hello World!"));
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        assert_eq!(parse(&test_word), Token::Bold("Hello World!"));
    }

    #[test]
    fn test_bar() {
        let test_word = "~~Hello World!~~";
        assert_eq!(parse(&test_word), Token::Bar("Hello World!"));
    }

    #[test]
    fn test_link() {
        let test_word = "<https://www.google.com>";
        assert_eq!(parse(&test_word), Token::Link("https://www.google.com"));
    }

    #[test]
    fn test_hr() {
        let test_word = "---";
        assert_eq!(parse(&test_word), Token::Hr);
    }
}