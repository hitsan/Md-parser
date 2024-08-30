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

pub fn sentence(text: &str) -> Option<ParsedResult<Md>> {
    if text == "" { return None }
    let tokens = words(&text);
    Some(ParsedResult::new(Md::Sentence(tokens),""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence() {
        let test_word = "Hello World!";
        let token = Word::Normal("Hello World!".to_string());
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let token = Word::Normal("Hello World!".to_string());
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Word::Italic(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Word::Bold(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Word::StrikeThough(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = Word::Underline(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Bold(token));
        let token = Word::Underline(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "**__Hello World!__**";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Underline(token));
        let token = Word::Bold(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "~~**__Hello World!__**~~";
        let token = vec!(Word::Normal("Hello World!".to_string()));
        let token = vec!(Word::Underline(token));
        let token = vec!(Word::Bold(token));
        let token = Word::StrikeThough(token);
        let token = vec!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_vec() {
        let test_word = "Hello **World!**";
        let hello = Word::Normal("Hello ".to_string());
        let world = Word::Normal("World!".to_string());
        let world = Word::Bold(vec!(world));
        let token = vec!(hello, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let world = Word::Normal("World!".to_string());
        let token = vec!(hello, ast, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_abnormal() {
        let test_word = "Hello ****World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let ast1 = Word::Normal("**".to_string());
        let world = Word::Normal("World!".to_string());

        let token = vec!(hello, ast, ast1, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~World!";
        let hello = Word::Normal("Hello ".to_string());
        let ast = Word::Normal("**".to_string());
        let strike = Word::Normal("~~".to_string());
        let world = Word::Normal("World!".to_string());
        let token = vec!(hello, ast, strike, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~World!**";
        let hello = Word::Normal("Hello ".to_string());
        let strike = Word::Normal("~~".to_string());
        let world = Word::Normal("World!".to_string());
        let bo = Word::Bold(vec!(strike, world));
        let token = vec!(hello, bo);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~Wor__ld!__**";
        let hello = Word::Normal("Hello ".to_string());
        let strike = Word::Normal("~~".to_string());
        let wor = Word::Normal("Wor".to_string());
        let ld = Word::Normal("ld!".to_string());
        let un = Word::Underline(vec!(ld));
        let bo = Word::Bold(vec!(strike, wor, un));
        let token = vec!(hello, bo);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }
}