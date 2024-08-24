#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, String),
    Line(Vec<Emphasis>)
}

#[derive(Debug, PartialEq)]
pub enum Emphasis {
    Text(String),
    Italic(Vec<Emphasis>),
    Bold(Vec<Emphasis>),
    StrikeThough(Vec<Emphasis>),
    Underline(Vec<Emphasis>),
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
    em: &dyn Fn(Vec<Emphasis>)->Emphasis
) -> Option<ParsedResult<'a, Emphasis>> {
    let sentence = consume(sentence, pattern)?;
    let tem = term(sentence)?;
    let a = consume(tem.rest, pattern)?;
    Some(ParsedResult::new(em(vec!(tem.token)), a))
}

fn italic(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::Italic(token);
    emphasis(&sentence, "*", &em)
}

fn bold(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let em = |token| Emphasis::Bold(token);
    println!("{}", &sentence);
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
    let keywords = ["~~", "__", "**", "*"];
    let a = keywords.iter().find_map(|k| {
        if !sentence.starts_with(k) { return None }
        let len = k.len();
        Some(ParsedResult::new(Emphasis::Text(k.to_string()), &sentence[len..]))
    });
    if a.is_some() {return a }

    let indexs = ["~~", "__", "**", "*"].iter().filter_map(|p| sentence.find(p));
    match indexs.min() {
        Some(n) => {
            let token = &sentence[..n];
            let rest = &sentence[n..];
            Some(ParsedResult::new(Emphasis::Text(token.to_string()), rest))
        },
        None => {
            let token = Emphasis::Text(sentence.to_string());
            Some(ParsedResult::new(token,  ""))
        }
    }
}

fn term(sentence: &str) -> Option<ParsedResult<Emphasis>> {
    let parsers = vec!(underline, strike_though, bold, italic, text);
    parsers.iter().find_map(|f| f(sentence).and_then(
        |r| Some(ParsedResult::new(r.token, &r.rest))
    ))
}

fn line(sentence: &str) -> Option<ParsedResult<Md>> {
    let ret = term(sentence)?;
    let mut tokens = vec!(ret.token);
    let mut rest = ret.rest;
    while rest != "" {
        let ret = term(&rest)?;
        tokens.push(ret.token);
        rest = ret.rest;
    }
    Some(ParsedResult::new(Md::Line(tokens), &rest))
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
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let expectation = Emphasis::Text("Hello World!".to_string());
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Italic(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "**__Hello World!__**";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = Emphasis::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "~~**__Hello World!__**~~";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_text_vec() {
        let test_word = "Hello **World!**";
        let hello = Emphasis::Text("Hello ".to_string());
        let world = Emphasis::Text("World!".to_string());
        let world = Emphasis::Bold(vec!(world));
        let expectation = vec!(hello, world);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "Hello **World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let world = Emphasis::Text("World!".to_string());
        let expectation = vec!(hello, ast, world);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_text_abnormal() {
        let test_word = "Hello ****World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let ita = Emphasis::Italic(vec!(ast));
        let world = Emphasis::Text("World!".to_string());
        let expectation = vec!(hello, ita, world);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});

        let test_word = "Hello **~~World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let strike = Emphasis::Text("~~".to_string());
        let world = Emphasis::Text("World!".to_string());
        let expectation = vec!(hello, ast, strike, world);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }
    #[test]
    fn test_text_abno() {
        let test_word = "Hello **~~World!**";
        let hello = Emphasis::Text("Hello ".to_string());
        // let ast = Emphasis::Text("**".to_string());
        let strike = Emphasis::Text("~~".to_string());
        let world = Emphasis::Text("World!".to_string());
        let bo = Emphasis::Bold(vec!(strike, world));
        let expectation = vec!(hello, bo);
        let expectation = Md::Line(expectation);
        assert_eq!(parse(&test_word), ParsedResult{ token: expectation, rest: ""});
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let expectation = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Heading(1, expectation), rest: ""});
    }

}