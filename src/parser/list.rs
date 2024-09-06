use crate::parser::parser::*;
use super::sentence::words;

fn count_tab(texts: &str) -> usize {
    texts.chars().take_while(|c| c ==&' ' ).count()/2
}

fn item(texts: &str, tab_num: usize) -> Option<ParsedResult<Item>> {
    let (text, rest) = if let Some(n) = texts.find("\n") {
        (&texts[..n], &texts[(n+1)..])
    } else {
        (texts, "")
    };
    let text = text.trim_start();
    let text = consume(text, "-")?;
    let text = space(text)?;
    let words = words(&text);
    let space_num = count_tab(&rest);
    let (i, rest) = if space_num <= tab_num {
        (Items(vec!()), rest)
    } else {
        let c = items(&rest, space_num);
        (c.token, c.rest)
    };
    let item = Item(words, i);
    Some(ParsedResult::new(item, rest))
}

fn items(mut texts: &str, tab_num: usize) -> ParsedResult<Items> {
    let mut items: Vec<Item> = vec!();
    while let Some(i) = item(texts, tab_num) {
        if count_tab(texts) < tab_num { break; }
        items.push(i.token);
        texts = i.rest;
    }
    let items = Items(items);
    ParsedResult::new(items, texts)
}

pub fn list(texts: &str) -> Option<ParsedResult<Md>> {
    let l = items(texts, 0);
    match l.token {
        Items(item) if item.is_empty() => None,
        _ => Some(ParsedResult{token: Md::List(l.token), rest: l.rest})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let test_word = "- Hello World!\n";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let items0 = Items(vec!());
        let l = Item(w, items0);
        let rest = "";
        assert_eq!(item(&test_word, 0), Some(ParsedResult{token: l, rest}));

        let test_word = "- Hello World!";
        let n = Word::Normal("Hello World!".to_string());
        let w = Words(vec!(n));
        let items0 = Items(vec!());
        let l = Item(w, items0);
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
        let items0 = Items(vec!());
        let i0 = Item(w, items0);

        let n = Word::Normal("World".to_string());
        let w = Words(vec!(n));
        let items1 = Items(vec!());
        let i1 = Item(w, items1);

        let n = Word::Normal("Rust".to_string());
        let w = Words(vec!(n));
        let items2 = Items(vec!());
        let i2 = Item(w, items2);

        let token = Items(vec!(i0, i1, i2));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});

        let test_word = "Rust";
        let token = Items(vec!());
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest: test_word});
    }

    #[test]
    fn test_nest_items() {
        let test_word = "- Hello\n  - World";
        let n = Word::Normal("World".to_string());
        let w = Words(vec!(n));
        let items0 = Items(vec!());
        let i0 = Item(w, items0);
        let child = Items(vec!(i0));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let token = Items(vec!(i1));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});


        let test_word = "- Hello\n  - World\n  - End";
        let world = Word::Normal("World".to_string());
        let world = Words(vec!(world));
        let items0 = Items(vec!());
        let world_item = Item(world, items0);

        let end = Word::Normal("End".to_string());
        let end = Words(vec!(end));
        let items0 = Items(vec!());
        let end_item = Item(end, items0);

        let child = Items(vec!(world_item, end_item));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let token = Items(vec!(i1));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});


        let test_word = "- Hello\n  - World\n- End";
        let world = Word::Normal("World".to_string());
        let world = Words(vec!(world));
        let emp = Items(vec!());
        let world_item = Item(world, emp);

        let child = Items(vec!(world_item));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let hello_item = Item(w, child);

        let end = Word::Normal("End".to_string());
        let end = Words(vec!(end));
        let emp = Items(vec!());
        let end_item = Item(end, emp);

        let token = Items(vec!(hello_item, end_item));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});

        let test_word = "- Hello\n  - World\n  - End\n- Reboot";
        let world = Word::Normal("World".to_string());
        let world = Words(vec!(world));
        let items0 = Items(vec!());
        let world_item = Item(world, items0);

        let end = Word::Normal("End".to_string());
        let end = Words(vec!(end));
        let items0 = Items(vec!());
        let end_item = Item(end, items0);

        let child = Items(vec!(world_item, end_item));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let r = Word::Normal("Reboot".to_string());
        let w = Words(vec!(r));
        let nul = Items(vec!());
        let item_r = Item(w, nul);

        let token = Items(vec!(i1, item_r));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});


        let test_word = "- Hello\n  - World\n    - End\n- Reboot";
        let end = Word::Normal("End".to_string());
        let end = Words(vec!(end));
        let items0 = Items(vec!());
        let end_item = Item(end, items0);

        let world = Word::Normal("World".to_string());
        let world = Words(vec!(world));
        let items0 = Items(vec!(end_item));
        let world_item = Item(world, items0);

        let child = Items(vec!(world_item));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let r = Word::Normal("Reboot".to_string());
        let w = Words(vec!(r));
        let nul = Items(vec!());
        let item_r = Item(w, nul);

        let token = Items(vec!(i1, item_r));
        let rest = "";
        assert_eq!(items(&test_word, 0), ParsedResult{token, rest});
    }

    #[test]
    fn test_tab() {
        let text = "  hello";
        assert_eq!(count_tab(text), 1);

        let text = "hello";
        assert_eq!(count_tab(text), 0);

        let text = "     hello";
        assert_eq!(count_tab(text), 2);
    }

    #[test]
    fn test_list() {
        let test_word = "- Hello\n  - World";
        let n = Word::Normal("World".to_string());
        let w = Words(vec!(n));
        let items0 = Items(vec!());
        let i0 = Item(w, items0);
        let child = Items(vec!(i0));
        let n = Word::Normal("Hello".to_string());
        let w = Words(vec!(n));
        let i1 = Item(w, child);

        let token = Md::List(Items(vec!(i1)));
        let rest = "";
        assert_eq!(list(&test_word), Some(ParsedResult{token, rest}));
    }
}