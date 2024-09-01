use crate::parser::parser::*;
use super::line::words;
use std::collections::HashSet;

fn header(texts: &str) -> Option<ParsedResult<Record>> {
    let n = texts.find("\n")?;
    let (text, rest) = (&texts[..n].trim_end(), &texts[(n+1)..]);
    if !text.starts_with("|") || !text.ends_with("|") {
        return None
    }
    let len = text.len();
    let words: Vec<Words> = text[1..(len-1)].split("|").map(|t| {
        let t = t.trim();
        let words = words(t);
        Words(words)
    }).collect::<Vec<_>>();
    let record = Record(words);
    Some(ParsedResult{token: record, rest: rest})
}

fn align(texts: &str, num: usize) -> Option<ParsedResult<Vec<Align>>> {
    let n = texts.find("\n")?;
    let (text, rest) = (&texts[..n].trim_end(), &texts[(n+1)..]);
    if !text.starts_with("|") || !text.ends_with("|") {
        return None
    }
    let len = text.len();
    let aligns: Vec<Align> = text[1..(len-1)].split("|").filter_map(|t| {
        let t = t.trim();
        align_parse(t)
    }).collect::<Vec<_>>();
    if aligns.len() == num {
        Some(ParsedResult{token: aligns, rest})        
    } else {
        None
    }
}

fn align_parse(text: &str) -> Option<Align> {
    let l = text.starts_with(":");
    let r = text.ends_with(":");
    let is_only_hyphen = |text: &str| {
        let p: HashSet<char> = text.chars().collect();
        p.len() == 1 && p.contains(&'-')
    };
    match (l, r) {
        (true, true) if is_only_hyphen(&text[1..text.len()-1]) => Some(Align::Center),
        (true, false) if is_only_hyphen(&text[1..]) => Some(Align::Left),
        (false, true) if is_only_hyphen(&text[..text.len()-1]) => Some(Align::Right),
        (false, false) if is_only_hyphen(&text) => Some(Align::Left),
        _ => None,
    }
}

fn records(texts: &str, n: usize) -> Option<ParsedResult<Vec<Record>>> {
    // let n = texts.find("\n");
    let a = Words(vec!(Word::Normal("A".to_string())));
    let b = Words(vec!(Word::Normal("B".to_string())));
    let c = Words(vec!(Word::Normal("C".to_string())));
    let r0 = Record(vec!(a, b, c));
    let a = Words(vec!(Word::Normal("a".to_string())));
    let b = Words(vec!(Word::Normal("b".to_string())));
    let c = Words(vec!(Word::Normal("c".to_string())));
    let r1 = Record(vec!(a, b, c));
    let record = vec!(r0, r1);
    let rest = "";
    Some(ParsedResult{token: record, rest})
}

// pub fn table(texts: &str) -> Option<ParsedResult<Md>> {
//     ["#", "##", "###"].iter().find_map(|p| {
//         let (text, rest) = if let Some(n) = texts.find("\n") {
//             (&texts[..n], &texts[(n+1)..])
//         } else {
//             (texts, "")
//         };
//         let text = consume(text, p)?;
//         let text = space(text)?;
//         let tokens = words(&text);
//         let token = Md::Heading(p.len(), tokens);
//         Some(ParsedResult::new(token, rest))
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let h = "| A | B | C | \n";
        let a = Words(vec!(Word::Normal("A".to_string())));
        let b = Words(vec!(Word::Normal("B".to_string())));
        let c = Words(vec!(Word::Normal("C".to_string())));
        let token = Record(vec!(a, b, c));
        let rest = "";
        assert_eq!(header(&h), Some(ParsedResult{token, rest}));

        let h = "| A | B | C \n";
        assert_eq!(header(&h), None);
    }

    #[test]
    fn test_align() {
        let h = "| -: | :-: | :- | --- |\n";
        let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
        let rest = "";
        assert_eq!(align(&h, 4), Some(ParsedResult{token, rest}));

        let h = "| -: | :-b: | :- | - |\n";
        assert_eq!(align(&h, 4), None);
    }

    #[test]
    fn test_records() {
        let h = "| A | B | C |\n| a | b | c |";
        let a = Words(vec!(Word::Normal("A".to_string())));
        let b = Words(vec!(Word::Normal("B".to_string())));
        let c = Words(vec!(Word::Normal("C".to_string())));
        let r0 = Record(vec!(a, b, c));
        let a = Words(vec!(Word::Normal("a".to_string())));
        let b = Words(vec!(Word::Normal("b".to_string())));
        let c = Words(vec!(Word::Normal("c".to_string())));
        let r1 = Record(vec!(a, b, c));
        let record = vec!(r0, r1);
        let rest = "";
        assert_eq!(records(&h, 3), Some(ParsedResult{token: record, rest}));
    }
    // #[test]
    // fn test_align() {
    //     let h = "| -: | :-: | :- | - |";
    //     let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
    //     let rest = "";
    //     assert_eq!(align(&h), Some(ParsedResult{token, rest}));

    // }
}