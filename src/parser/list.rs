use crate::parser::parser::*;
use super::sentence::words;

fn count_tab(mut texts: &str, tab_num: usize) -> Option<ParsedResult<usize>> {
    let mut num: usize = 0;
    while let Some(text) = consume(texts, "  ") {
        texts = text;
        num += 1;
    }
    if num <= tab_num {
        None
    } else {
        Some(ParsedResult{token: num, rest: texts})
    }
}

fn item(texts: &str, tab_num: usize) -> Option<ParsedResult<Item>> {
    let (text, rest) = if let Some(n) = texts.find("\n") {
        (&texts[..n], &texts[(n+1)..])
    } else {
        (texts, "")
    };
    let text = consume(text, "-")?;
    let text = space(text)?;
    let words = words(&text);
    let item = Item(words, None);
    Some(ParsedResult::new(item, rest))
}

fn items(mut texts: &str) -> Option<ParsedResult<Items>> {
    let mut items: Vec<Item> = vec!();
    while let Some(i) = item(texts, 0) {
        items.push(i.token);
        texts = i.rest;
    }
    if items.is_empty() { return None }
    let items = Items(items);
    Some(ParsedResult::new(items, texts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let test_word = "- Hello World!\n";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let l = Item(w, None);
        let rest = "";
        assert_eq!(item(&test_word, 0), Some(ParsedResult{token: l, rest}));

        let test_word = "- Hello World!";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let l = Item(w, None);
        let rest = "";
        assert_eq!(item(&test_word, 0), Some(ParsedResult{token: l, rest}));

        let test_word = "Hello World!";
        assert_eq!(item(&test_word, 0), None);

        let test_word = "-Hello World!";
        assert_eq!(item(&test_word, 0), None);
    }

    #[test]
    fn test_items() {
        let test_word = "- Hello\n- World\n- Rust";
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i0 = Item(w, None);

        let n = Word::Normal("World".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, None);

        let n = Word::Normal("Rust".to_string());
        let w = Words(vec!(n));
        let i2 = Item(w, None);

        let token = Items(vec!(i0, i1, i2));
        let rest = "";
        assert_eq!(items(&test_word), Some(ParsedResult{token, rest}));

        let test_word = "Rust";
        assert_eq!(items(&test_word), None);
    }

    #[test]
    fn test_tab() {
        let text = "  hello";
        assert_eq!(count_tab(text, 0), Some(ParsedResult{token: 1, rest: "hello"}));

        let text = "hello";
        assert_eq!(count_tab(text, 1), None);

        let text = "hello";
        assert_eq!(count_tab(text, 0), None);

        let text = "  hello";
        assert_eq!(count_tab(text, 1), None);

        let text = "     hello";
        assert_eq!(count_tab(text, 1), Some(ParsedResult{token: 2, rest: " hello"}));
    }
}