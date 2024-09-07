use crate::parser::parser::*;
use crate::{words,normal_word};

fn emphasis<'a>(
    text: &'a str,
    pattern: &'a str,
    em: &dyn Fn(Words)->Word
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
        Some(ParsedResult::new(normal_word!(k), &text))
    });
    if let Some(ret) = matched_prefix {
        return Some(ret)
    }
    let indexs = keywords.iter().filter_map(|p| text.find(p));
    if let Some(n) = indexs.min() {
        let token = &text[..n];
        let rest = &text[n..];
        return Some(ParsedResult::new(normal_word!(token), rest))
    }
    let token = normal_word!(text);
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

pub fn words(text: &str) -> Words {
    let token = word(&text);
    let mut rest = token.rest;
    let mut tokens = vec!(token.token);
    while !rest.is_empty() {
        let ret = word(&rest);
        tokens.push(ret.token);
        rest = ret.rest;
    }
    Words(tokens)
}

pub fn sentence(texts: &str) -> Option<ParsedResult<Md>> {
    if texts == "" { return None }
    let (text, rest) = if let Some(n) = texts.find("\n") {
        (&texts[..n], &texts[(n+1)..])
    } else {
        (texts, "")
    };
    let tokens = words(&text);
    Some(ParsedResult::new(Md::Sentence(tokens), rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence() {
        let test_word = "Hello World!";
        let token = normal_word!("Hello World!");
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text() {
        let test_word = "Hello World!";
        let token = normal_word!("Hello World!");
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_italic() {
        let test_word = "*Hello World!*";
        let token = words!(normal_word!("Hello World!"));
        let token = Word::Italic(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_bold() {
        let test_word = "**Hello World!**";
        let token = words!(normal_word!("Hello World!"));
        let token = Word::Bold(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_strike_though() {
        let test_word = "~~Hello World!~~";
        let token = words!(normal_word!("Hello World!"));
        let token = Word::StrikeThough(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_underline() {
        let test_word = "__Hello World!__";
        let token = words!(normal_word!("Hello World!"));
        let token = Word::Underline(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_mix() {
        let test_word = "__**Hello World!**__";
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Bold(token));
        let token = Word::Underline(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "**__Hello World!__**";
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Underline(token));
        let token = Word::Bold(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "~~**__Hello World!__**~~";
        let token = words!(normal_word!("Hello World!"));
        let token = words!(Word::Underline(token));
        let token = words!(Word::Bold(token));
        let token = Word::StrikeThough(token);
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_vec() {
        let test_word = "Hello **World!**";
        let hello = normal_word!("Hello ");
        let world = normal_word!("World!");
        let world = Word::Bold(words!(world));
        let token = words!(hello, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **World!";
        let hello = normal_word!("Hello ");
        let ast = normal_word!("**");
        let world = normal_word!("World!");
        let token = words!(hello, ast, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_abnormal() {
        let test_word = "Hello ****World!";
        let hello = normal_word!("Hello ");
        let ast = normal_word!("**");
        let ast1 = normal_word!("**");
        let world = normal_word!("World!");

        let token = words!(hello, ast, ast1, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~World!";
        let hello = normal_word!("Hello ");
        let ast = normal_word!("**");
        let strike = normal_word!("~~");
        let world = normal_word!("World!");
        let token = words!(hello, ast, strike, world);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~World!**";
        let hello = normal_word!("Hello ");
        let strike = normal_word!("~~");
        let world = normal_word!("World!");
        let bo = Word::Bold(words!(strike, world));
        let token = words!(hello, bo);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Hello **~~Wor__ld!__**";
        let hello = normal_word!("Hello ");
        let strike = normal_word!("~~");
        let wor = normal_word!("Wor");
        let ld = normal_word!("ld!");
        let un = Word::Underline(words!(ld));
        let bo = Word::Bold(words!(strike, wor, un));
        let token = words!(hello, bo);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_multiline() {
        let test_word = "Hello\n World!";
        let token = normal_word!("Hello");
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = " World!";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "**Hello**\n World!";
        let token = normal_word!("Hello");
        let token = Word::Bold(words!(token));
        let token = words!(token);
        let token = Md::Sentence(token);
        let rest = " World!";
        assert_eq!(sentence(&test_word), Some(ParsedResult{token, rest}));
    }
}