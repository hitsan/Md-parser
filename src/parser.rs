#[derive(Debug, PartialEq)]
pub enum Md {
    Heading(usize, String),
    Text(String)
}

#[derive(Debug, PartialEq)]
pub struct ParsedResult<'a> {
    token: Md,
    rest: &'a str,
}

impl<'a> ParsedResult<'a> {
    pub fn new(token: Md, rest: &'a str) -> ParsedResult<'a> {
        ParsedResult { token: token, rest: rest }
    }
}

fn heading(line: &str) -> Option<ParsedResult> {
    ["# ", "## ", "### "].iter().enumerate().find_map(|p| {
        if line.starts_with(p.1){
            let word = line[(p.0+2)..].to_string();
            let ret = ParsedResult::new(Md::Heading(p.0+1, word), &"");
            Some(ret)          
        } else {
            None
        }
    })
}

fn text(line: &str) -> Option<ParsedResult> {
    let li = line.to_string();
    let pr = ParsedResult::new(Md::Text(li), &"");
    Some(pr)
}

pub fn parse(line: &str) -> ParsedResult {
    let parsers = vec!(heading, text);
    let ret = parsers.iter().find_map(|f| f(line));
    ret.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        let test_word = "Hello World!";
        let md_ans = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Text(md_ans), rest: &""});
    }

    #[test]
    fn test_heading() {
        let test_word = "# Hello World!";
        let md_ans = "Hello World!".to_string();
        assert_eq!(parse(&test_word), ParsedResult{ token: Md::Heading(1, md_ans), rest: &""});
    }

}