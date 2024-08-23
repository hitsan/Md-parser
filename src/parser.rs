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
        let word = &sentence[(p.0+2)..];
        let ret = ParsedResult::new(Md::Heading(p.0+1, word.to_string()), "");
        Some(ret)
    })
    // and_some
}

fn consume<'a>(sentence: &'a str, pattern: &'a str) -> Option<&'a str> {
    if !sentence.starts_with(pattern) { return None }
    let length = pattern.len();
    Some(&sentence[length..])
}

fn emphasis<'a>(
    sentence: &'a str,
    pattern: &'a str,
    em: &dyn Fn(Box<Emphasis>)->Emphasis
) -> Option<ParsedResult<'a, Emphasis>> {
    let ret = consume(sentence, pattern)?;
    ret.find(pattern).and_then(|n| {
        term(&ret[..n]).and_then(|s| {
            let token = em(Box::new(s.token));
            let rest = &s.rest;
            Some(ParsedResult::new(token, rest))
        })
    })
}

fn italic(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::Italic(token);
    emphasis(&sentence, "*", &em)
}

fn bold(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::Bold(token);
    emphasis(&sentence, "**", &em)
}

fn underline(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::Underline(token);
    emphasis(&sentence, "__", &em)
}

fn strike_though(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::StrikeThough(token);
    emphasis(&sentence, "~~", &em)
}

fn text(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    ["~~", "__", "**", "*"].iter().find_map(|p| {
        sentence.find(p).and_then(|n| {
            let token = &sentence[..n];
            let rest = &sentence[n..];
            Some(ParsedResult::new(Emphasis::Text(token.to_string()), rest))  
        })
    }).or_else(||{
        let token = Emphasis::Text(sentence.to_string());
        Some(ParsedResult::new(token,  ""))
    })
}

fn term(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let parsers = vec!(underline, strike_though, bold, italic, text);
    parsers.iter().find_map(|f| f(sentence).and_then(
        |r| Some(ParsedResult::new(r.token, &r.rest))
    ))
}

fn line(sentence: &str) -> Option<ParsedResult<Md>> {
    term(sentence).and_then(
        |r| Some(ParsedResult::new(Md::Line(r.token), &r.rest))
    )
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
        let expectation = Emphasis::Text("Hello World!".to_string());
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let expectation = Emphasis::Text("Hello World!".to_string());
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Italic(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Bold(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Underline(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Box::new(Emphasis::Bold(expectation));
        let expectation = Emphasis::Underline(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "**__Hello World!__**";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Box::new(Emphasis::Underline(expectation));
        let expectation = Emphasis::Bold(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "~~**__Hello World!__**~~";
        let expectation = Box::new(Emphasis::Text("Hello World!".to_string()));
        let expectation = Box::new(Emphasis::Underline(expectation));
        let expectation = Box::new(Emphasis::Bold(expectation));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_text_fun() {
        let test_word = "Hello **World!**";
        let expectation = Emphasis::Text("Hello ".to_string());
        assert_eq!(text(&test_word), Some(ParsedResult{ token: expectation, rest: "**World!**"}));

        let test_word = "Hello **World!";
        let expectation = Emphasis::Text("Hello ".to_string());
        assert_eq!(text(&test_word), Some(ParsedResult{ token: expectation, rest: "**World!"}));
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let expectation = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Heading(1, expectation), rest: ""});
    }

}