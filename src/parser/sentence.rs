use crate::parser::parser::*;
use crate::normal_word;

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
    use crate::{words,normal_word};

    #[test]
    fn test_sentence() {
        let words = words!(normal_word!("Hello World!"));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello World!"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text() {
        let words = words!(normal_word!("Hello World!"));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello World!"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_italic() {
        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Italic(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"*Hello World!*"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_bold() {
        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Bold(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"**Hello World!**"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_strike_though() {
        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::StrikeThough(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"~~Hello World!~~"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_underline() {
        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Underline(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"__Hello World!__"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_mix() {
        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Bold(words));
        let words = words!(Word::Underline(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"__**Hello World!**__"), Some(ParsedResult{token, rest}));

        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Underline(words));
        let words = words!(Word::Bold(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"**__Hello World!__**"), Some(ParsedResult{token, rest}));

        let words = words!(normal_word!("Hello World!"));
        let words = words!(Word::Underline(words));
        let words = words!(Word::Bold(words));
        let words = words!(Word::StrikeThough(words));
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"~~**__Hello World!__**~~"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_vec() {
        let word0 = normal_word!("Hello ");
        let word1 = Word::Bold(words!(normal_word!("World!")));
        let words = words!(word0, word1);
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello **World!**"), Some(ParsedResult{token, rest}));

        let word0 = normal_word!("Hello ");
        let word1 = normal_word!("**");
        let word2 = normal_word!("World!");
        let words = words!(word0, word1, word2);
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello **World!"), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_text_abnormal() {
        let word0 = normal_word!("Hello ");
        let word1 = normal_word!("**");
        let word2 = normal_word!("**");
        let word3 = normal_word!("World!");
        let words = words!(word0, word1, word2, word3);
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello ****World!"), Some(ParsedResult{token, rest}));

        let word0 = normal_word!("Hello ");
        let word1 = normal_word!("**");
        let word2 = normal_word!("~~");
        let word3 = normal_word!("World!");
        let words = words!(word0, word1, word2, word3);
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello **~~World!"), Some(ParsedResult{token, rest}));

        let word0 = normal_word!("~~");
        let word1 = normal_word!("World!");
        let bold = Word::Bold(words!(word0, word1));
        let normal = normal_word!("Hello ");
        let words = words!(normal, bold);
        let token = Md::Sentence(words);
        let rest = "";
        assert_eq!(sentence(&"Hello **~~World!**"), Some(ParsedResult{token, rest}));

        let strike = normal_word!("~~");
        let wor = normal_word!("Wor");
        let ld = normal_word!("ld!");
        let un = Word::Underline(words!(ld));
        let bold = Word::Bold(words!(strike, wor, un));
        let hello = normal_word!("Hello ");
        let token = words!(hello, bold);
        let token = Md::Sentence(token);
        let rest = "";
        assert_eq!(sentence(&"Hello **~~Wor__ld!__**"), Some(ParsedResult{token, rest}));
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