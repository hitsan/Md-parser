use crate::parser::parser::*;

#[derive(Debug, PartialEq)]
pub enum Emphasis {
    Text(String),
    Italic(Vec<Emphasis>),
    Bold(Vec<Emphasis>),
    StrikeThough(Vec<Emphasis>),
    Underline(Vec<Emphasis>),
}

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
