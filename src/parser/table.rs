use crate::parser::parser::*;
use super::line::words;
use std::collections::HashSet;

fn header(mut texts: &str) -> Option<ParsedResult<Record>> {
    texts = texts.trim_end();
    if !texts.starts_with("|") || !texts.ends_with("|") {
        return None
    }
    let len = texts.len();
    texts = &texts[1..(len-1)];
    let words: Vec<Words> = texts.split("|").map(|text| {
        let text = text.trim();
        let words = words(text);
        Words(words)
    }).collect::<Vec<_>>();
    let record = Record(words);
    Some(ParsedResult{token: record, rest: ""})
}

fn align(mut texts: &str, num: usize) -> Option<ParsedResult<Vec<Align>>> {
    texts = texts.trim_end();
    if !texts.starts_with("|") || !texts.ends_with("|") {
        return None
    }
    let len = texts.len();
    texts = &texts[1..(len-1)];
    let aligns: Vec<Align> = texts.split("|").filter_map(|text| {
        let text = text.trim();
        align_parse(text)
    }).collect::<Vec<_>>();
    if aligns.len() == num {
        Some(ParsedResult{token: aligns, rest: ""})        
    } else {
        None
    }
}

fn align_parse(text: &str) -> Option<Align> {
    let l = text.starts_with(":");
    let r = text.ends_with(":");
    if l && r {
        let t = &text[1..text.len()-1];
        let p: HashSet<char> = t.chars().collect();
        if p.len() == 1 && p.contains(&'-') {
            Some(Align::Center)
        } else {
            None
        }
    } else if l {
        let t = &text[1..];
        let p: HashSet<char> = t.chars().collect();
        if p.len() == 1 && p.contains(&'-') {
            Some(Align::Left)
        } else {
            None
        }
    } else if r {
        let t = &text[..text.len()-1];
        let p: HashSet<char> = t.chars().collect();
        if p.len() == 1 && p.contains(&'-') {
            Some(Align::Right)
        } else {
            None
        }
    } else {
        let t = text;
        let p: HashSet<char> = t.chars().collect();
        if p.len() == 1 && p.contains(&'-') {
            Some(Align::Left)
        } else {
            None
        }
    }
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
        let h = "| A | B | C |";
        let a = Words(vec!(Word::Normal("A".to_string())));
        let b = Words(vec!(Word::Normal("B".to_string())));
        let c = Words(vec!(Word::Normal("C".to_string())));
        let token = Record(vec!(a, b, c));
        let rest = "";
        assert_eq!(header(&h), Some(ParsedResult{token, rest}));

        let h = "| A | B | C ";
        assert_eq!(header(&h), None);
    }

    #[test]
    fn test_align() {
        let h = "| -: | :-: | :- | - |";
        let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
        let rest = "";
        assert_eq!(align(&h, 4), Some(ParsedResult{token, rest}));

    }

    // #[test]
    // fn test_align() {
    //     let h = "| -: | :-: | :- | - |";
    //     let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
    //     let rest = "";
    //     assert_eq!(align(&h), Some(ParsedResult{token, rest}));

    // }
}