#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, String),
    Line(String)
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
}

fn line(sentence: &str) -> Option<ParsedResult<Md>> {
    let li = sentence.to_string();
    let pr = ParsedResult::new(Md::Line(li), &"");
    Some(pr)
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
    fn test_word() {
        let test_word = "Hello World!";
        let md_ans = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Line(md_ans), rest: &""});
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let md_ans = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Heading(1, md_ans), rest: &""});
    }

}