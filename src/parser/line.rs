use crate::parser::parser::*;

fn emphasis<'a>(
    text: &'a str,
    pattern: &'a str,
    em: &dyn Fn(Vec<Word>)->Word
) -> Option<ParsedResult<'a, Word>> {
    let text = consume(text, pattern)?;
    let index = text.find(pattern)?;
    if index == 0 { return  None }
    let tokens = words(&text[..index]);
    let len = pattern.len();
    Some(ParsedResult::new(em(tokens), &text[(index+len)..]))
}

fn italic(text: &str) -> Option<ParsedResult<Word>> {
    let em = |token| Word::Italic(token);
    emphasis(&text, "*", &em)
}

fn bold(text: &str) -> Option<ParsedResult<Word>> {
    let em = |token| Word::Bold(token);
    emphasis(&text, "**", &em)
}

fn underline(text: &str) -> Option<ParsedResult<Word>> {
    let em = |token| Word::Underline(token);
    emphasis(&text, "__", &em)
}

fn strike_though(text: &str) -> Option<ParsedResult<Word>> {
    let em = |token| Word::StrikeThough(token);
    emphasis(&text, "~~", &em)
}

fn normal(text: &str) -> Option<ParsedResult<Word>> {
    let keywords = ["~~", "__", "**", "*"];
    let matched_prefix = keywords.iter().find_map(|k| {
        let text = consume(text, k)?;
        Some(ParsedResult::new(Word::Normal(k.to_string()), &text))
    });
    if let Some(ret) = matched_prefix {
        return Some(ret)
    }
    let indexs = keywords.iter().filter_map(|p| text.find(p));
    if let Some(n) = indexs.min() {
        let token = &text[..n];
        let rest = &text[n..];
        return Some(ParsedResult::new(Word::Normal(token.to_string()), rest))
    }
    let token = Word::Normal(text.to_string());
    Some(ParsedResult::new(token,  ""))
}

fn word(text: &str) -> ParsedResult<Word> {
    let parsers = vec!(underline, strike_though, bold, italic, normal);
    let parsed_ret = parsers.iter().find_map(|f| f(text));
    match parsed_ret {
        Some(ret) => ret,
        _ => panic!("parse err!")
    }
}

pub fn words(text: &str) -> Vec<Word> {
    let token = word(&text);
    let mut rest = token.rest;
    let mut tokens = vec!(token.token);
    while !rest.is_empty() {
        let ret = word(&rest);
        tokens.push(ret.token);
        rest = ret.rest;
    }
    tokens
}

pub fn line(text: &str) -> Option<Md> {
    let tokens = words(&text);
    Some(Md::Sentence(tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let test_word = "Hello World!";
        let expectation = Word::Normal("Hello World!".to_string());
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let expectation = Word::Normal("Hello World!".to_string());
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = Word::Italic(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = Word::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = Word::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = Word::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Bold(expectation));
        let expectation = Word::Underline(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "**__Hello World!__**";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Underline(expectation));
        let expectation = Word::Bold(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "~~**__Hello World!__**~~";
        let expectation = vec!(Word::Normal("Hello World!".to_string()));
        let expectation = vec!(Word::Underline(expectation));
        let expectation = vec!(Word::Bold(expectation));
        let expectation = Word::StrikeThough(expectation);
        let expectation = vec!(expectation);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text_vec() {
        let test_word = "Hello **World!**";
        let hello = Word::Normal("Hello ".to_string());
        let world = Word::Normal("World!".to_string());
        let world = Word::Bold(vec!(world));
        let expectation = vec!(hello, world);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let world = Word::Normal("World!".to_string());
        let expectation = vec!(hello, ast, world);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }

    #[test]
    fn test_text_abnormal() {
        let test_word = "Hello ****World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let ast1 = Word::Normal("**".to_string());
        let world = Word::Normal("World!".to_string());

        let expectation = vec!(hello, ast, ast1, world);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let strike = Word::Normal("~~".to_string());
        let world = Word::Normal("World!".to_string());
        let expectation = vec!(hello, ast, strike, world);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~World!**";
        let hello = Word::Normal("Hello ".to_string());
        let strike = Word::Normal("~~".to_string());
        let world = Word::Normal("World!".to_string());
        let bo = Word::Bold(vec!(strike, world));
        let expectation = vec!(hello, bo);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));

        let test_word = "Hello **~~Wor__ld!__**";
        let hello = Word::Normal("Hello ".to_string());
        let strike = Word::Normal("~~".to_string());
        let wor = Word::Normal("Wor".to_string());
        let ld = Word::Normal("ld!".to_string());
        let un = Word::Underline(vec!(ld));
        let bo = Word::Bold(vec!(strike, wor, un));
        let expectation = vec!(hello, bo);
        let expectation = Md::Sentence(expectation);
        assert_eq!(line(&test_word), Some(expectation));
    }
}