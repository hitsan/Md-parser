use crate::parser::parser::*;
use super::sentence::words;

fn item(texts: &str) -> Option<ParsedResult<List>> {
    let p = "-";
    let (text, rest) = if let Some(n) = texts.find("\n") {
        (&texts[..n], &texts[(n+1)..])
    } else {
        (texts, "")
    };
    let text = consume(text, p)?;
    let text = space(text)?;
    let words = words(&text);
    let item = List::Item(words);
    Some(ParsedResult::new(item, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let test_word = "- Hello World!\n";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let l = List::Item(w);
        let rest = "";
        assert_eq!(item(&test_word), Some(ParsedResult{token: l, rest}));

        let test_word = "- Hello World!";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let l = List::Item(w);
        let rest = "";
        assert_eq!(item(&test_word), Some(ParsedResult{token: l, rest}));
    }
}