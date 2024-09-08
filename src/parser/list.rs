use crate::parser::parser::*;
use super::sentence::words;
use crate::items;

fn count_tab(texts: &str) -> usize {
    texts.chars().take_while(|c| c ==&' ' ).count()/2
}

fn item(texts: &str, tab_num: usize) -> Option<ParsedResult<Item>> {
    let (text, rest) = split_first_pattern(texts, "\n");
    let text = text.trim_start();
    let text = consume(text, "-")?;
    let text = space(text)?;
    let words = words(&text);
    let space_num = count_tab(&rest);
    let (i, rest) = if space_num <= tab_num {
        (items!(), rest)
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
    use crate::{words,items,normal_word};

    #[test]
    fn test_item() {
        let words = words!(normal_word!("Hello World!"));
        let token = Item(words, items!());
        let rest = "";
        assert_eq!(item(&"- Hello World!\n", 0), Some(ParsedResult{token, rest}));

        let words = words!(normal_word!("Hello World!"));
        let token = Item(words, items!());
        let rest = "";
        assert_eq!(item(&"- Hello World!", 0), Some(ParsedResult{token, rest}));
        assert_eq!(item(&"Hello World!", 0), None);
        assert_eq!(item(&"-Hello World!", 0), None);
    }

    #[test]
    fn test_items() {
        let words = words!(normal_word!("Hello"));
        let item0 = Item(words, items!());
        let words = words!(normal_word!("World"));
        let item1 = Item(words, items!());
        let words = words!(normal_word!("Rust"));
        let item2 = Item(words, items!());
        let token = items!(item0, item1, item2);
        let rest = "";
        assert_eq!(items(&"- Hello\n- World\n- Rust", 0), ParsedResult{token, rest});
        assert_eq!(items(&"Rust", 0), ParsedResult{token: items!(), rest: "Rust"});
    }

    #[test]
    fn test_nest_items() {
        let words = words!(normal_word!("World"));
        let children = items!(Item(words, items!()));
        let words = words!(normal_word!("Hello"));
        let token = items!(Item(words, children));
        let rest = "";
        assert_eq!(items(&"- Hello\n  - World", 0), ParsedResult{token, rest});

        let world = words!(normal_word!("World"));
        let item0 = Item(world, items!());
        let world = words!(normal_word!("End"));
        let item1 = Item(world, items!());
        let children = items!(item0, item1);
        let words = words!(normal_word!("Hello"));
        let token = items!(Item(words, children));
        let rest = "";
        assert_eq!(items(&"- Hello\n  - World\n  - End", 0), ParsedResult{token, rest});

        let words = words!(normal_word!("World"));
        let children = items!(Item(words, items!()));
        let words = words!(normal_word!("Hello"));
        let item0 = Item(words, children);
        let end = words!(normal_word!("End"));
        let item1 = Item(end, items!());
        let token = items!(item0, item1);
        let rest = "";
        assert_eq!(items(&"- Hello\n  - World\n- End", 0), ParsedResult{token, rest});

        let words = words!(normal_word!("World"));
        let item0 = Item(words, items!());
        let words = words!(normal_word!("End"));
        let item1 = Item(words, items!());
        let children = items!(item0, item1);

        let words = words!(normal_word!("Hello"));
        let item0 = Item(words, children);
        let words = words!(normal_word!("Reboot"));
        let item1 = Item(words, items!());
        let token = items!(item0, item1);
        let rest = "";
        assert_eq!(items(&"- Hello\n  - World\n  - End\n- Reboot", 0), ParsedResult{token, rest});


        let words = words!(normal_word!("End"));
        let children = items!(Item(words, items!()));

        let words = words!(normal_word!("World"));
        let children = items!(Item(words, children));

        let words = words!(normal_word!("Hello"));
        let item0 = Item(words, children);

        let words = words!(normal_word!("Reboot"));
        let item1 = Item(words, items!());

        let token = items!(item0, item1);
        let rest = "";
        assert_eq!(items(&"- Hello\n  - World\n    - End\n- Reboot", 0), ParsedResult{token, rest});
    }

    #[test]
    fn test_tab() {
        assert_eq!(count_tab("  hello"), 1);
        assert_eq!(count_tab("hello"), 0);
        assert_eq!(count_tab("     hello"), 2);
    }

    #[test]
    fn test_list() {
        let words = words!(normal_word!("World"));
        let children = items!(Item(words, items!()));
        let words = words!(normal_word!("Hello"));
        let item = Item(words, children);
        let token = Md::List(items!(item));
        let rest = "";
        assert_eq!(list(&"- Hello\n  - World"), Some(ParsedResult{token, rest}));
    }
}