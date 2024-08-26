use crate::parser::parser::*;

fn emphasis<'a>(
    sentence: &'a str,
    pattern: &'a str,
    em: &dyn Fn(Vec<Emphasis>)->Emphasis
) -> Option<ParsedResult<'a, Emphasis>> {
    let sentence = consume(sentence, pattern)?;
    let index = sentence.find(pattern)?;
    if index == 0 { return  None }
    let tokens = terms(&sentence[..index]);
    let len = pattern.len();
    Some(ParsedResult::new(em(tokens), &sentence[(index+len)..]))
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
    let keywords = ["~~", "__", "**", "*"];
    let matched_prefix = keywords.iter().find_map(|k| {
        let sentence = consume(sentence, k)?;
        Some(ParsedResult::new(Emphasis::Text(k.to_string()), &sentence))
    });
    if let Some(ret) = matched_prefix {
        return Some(ret)
    }
    let indexs = keywords.iter().filter_map(|p| sentence.find(p));
    if let Some(n) = indexs.min() {
        let token = &sentence[..n];
        let rest = &sentence[n..];
        return Some(ParsedResult::new(Emphasis::Text(token.to_string()), rest))
    }
    let token = Emphasis::Text(sentence.to_string());
    Some(ParsedResult::new(token,  ""))
}

fn term(sentence: &str) -> ParsedResult<Emphasis> {
    let parsers = vec!(underline, strike_though, bold, italic, text);
    let parsed_ret = parsers.iter().find_map(|f| f(sentence));
    match parsed_ret {
        Some(ret) => ret,
        _ => panic!("parse err!")
    }
}

pub fn terms(sentence: &str) -> Vec<Emphasis> {
    let token = term(&sentence);
    let mut rest = token.rest;
    let mut tokens = vec!(token.token);
    while !rest.is_empty() {
        let ret = term(&rest);
        tokens.push(ret.token);
        rest = ret.rest;
    }
    tokens
}

pub fn line(sentence: &str) -> Option<Md> {
    let tokens = terms(&sentence);
    Some(Md::Line(tokens))
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
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let expectation = Emphasis::Text("Hello World!".to_string());
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Italic(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = Emphasis::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "**__Hello World!__**";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = Emphasis::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "~~**__Hello World!__**~~";
        let expectation = vec!(Emphasis::Text("Hello World!".to_string()));
        let expectation = vec!(Emphasis::Underline(expectation));
        let expectation = vec!(Emphasis::Bold(expectation));
        let expectation = Emphasis::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text_vec() {
        let test_word = "Hello **World!**";
        let hello = Emphasis::Text("Hello ".to_string());
        let world = Emphasis::Text("World!".to_string());
        let world = Emphasis::Bold(vec!(world));
        let expectation = vec!(hello, world);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let world = Emphasis::Text("World!".to_string());
        let expectation = vec!(hello, ast, world);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text_abnormal() {
        let test_word = "Hello ****World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let ast1 = Emphasis::Text("**".to_string());
        let world = Emphasis::Text("World!".to_string());

        let expectation = vec!(hello, ast, ast1, world);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~World!";
        let hello = Emphasis::Text("Hello ".to_string());
        let ast = Emphasis::Text("**".to_string());
        let strike = Emphasis::Text("~~".to_string());
        let world = Emphasis::Text("World!".to_string());
        let expectation = vec!(hello, ast, strike, world);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~World!**";
        let hello = Emphasis::Text("Hello ".to_string());
        let strike = Emphasis::Text("~~".to_string());
        let world = Emphasis::Text("World!".to_string());
        let bo = Emphasis::Bold(vec!(strike, world));
        let expectation = vec!(hello, bo);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~Wor__ld!__**";
        let hello = Emphasis::Text("Hello ".to_string());
        let strike = Emphasis::Text("~~".to_string());
        let wor = Emphasis::Text("Wor".to_string());
        let ld = Emphasis::Text("ld!".to_string());
        let un = Emphasis::Underline(vec!(ld));
        let bo = Emphasis::Bold(vec!(strike, wor, un));
        let expectation = vec!(hello, bo);
        let expectation = Md::Line(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }
}