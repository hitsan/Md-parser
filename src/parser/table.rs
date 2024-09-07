use crate::parser::parser::*;
use super::sentence::words;
use std::collections::HashSet;

fn record<'a, T>(
    texts: &'a str,
    closur: &dyn Fn(&str)->T
) -> Option<ParsedResult<'a, Vec<T>>> {
    let n = texts.find("\n")?;
    let (text, rest) = (&texts[..n].trim_end(), &texts[(n+1)..]);
    if !text.starts_with("|") || !text.ends_with("|") {
        return None
    }
    let len = text.len();
    let token: Vec<T> = text[1..(len-1)].split("|").map(|t| {
        let t = t.trim();
        closur(t)
    }).collect::<Vec<_>>();
    Some(ParsedResult{token, rest: rest})
}

fn header(texts: &str) -> Option<ParsedResult<Record>> {
    let cells = record(
        texts, &|txt| words(txt)
    )?;
    let record = Record(cells.token);
    Some(ParsedResult{token: record, rest: cells.rest})
}

fn align(texts: &str, num: usize) -> Option<ParsedResult<Vec<Align>>> {
    let cells = record(
        texts, 
        &|txt| align_parse(txt.trim()))?;
    let token = cells.token;
    let aligns: Vec<Align> = token.into_iter()
        .filter_map(|opt| opt)
        .collect();
    if aligns.len() == num {
        Some(ParsedResult{token: aligns, rest: cells.rest})        
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

fn records(mut texts: &str, n: usize) -> Option<ParsedResult<Vec<Record>>> {
    let mut record_list:Vec<Record> = vec!();
    while let Some(cells) = record(
        texts, &|txt| words(txt)) {
        let record = cells.token;
        println!("{:?}", &cells.rest);
        if record.len()!=n {
            break;
        }
        texts = cells.rest;
        let record = Record(record);
        record_list.push(record);
    }
    if record_list.is_empty() {
        None
    } else {
        Some(ParsedResult{token: record_list, rest: texts})
    }
}
fn len(record: &Record) -> usize {
    match record {
        Record(r) => r.len()
    }
}

pub fn table(texts: &str) -> Option<ParsedResult<Md>> {
    let h = header(texts)?;
    let texts = h.rest;
    let h = h.token;
    let num = len(&h);
    let a = align(texts, num)?;
    let texts = a.rest;
    let a = a.token;
    let r = records(texts, num)?;
    let rest = r.rest;
    let r = r.token;
    let t = Table{header: h, align: a, records: r};
    let t = Md::Table(Box::new(t));
    Some(ParsedResult{token: t, rest})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{words,record,normal_word};

    #[test]
    fn test_header() {
        let h = "| A | B | C | \n";
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let token = record!(a, b, c);
        let rest = "";
        assert_eq!(header(&h), Some(ParsedResult{token, rest}));

        let h = "| A | B | C \n";
        assert_eq!(header(&h), None);

        let h = "|  | B | C |\n";
        let a = words!(normal_word!(""));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let token = record!(a, b, c);
        let rest = "";
        assert_eq!(header(&h), Some(ParsedResult{token, rest}));
    }

    #[test]
    fn test_align() {
        let h = "| -: | :-: | :- | --- |\n";
        let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
        let rest = "";
        assert_eq!(align(&h, 4), Some(ParsedResult{token, rest}));

        let h = "| -: | :-b: | :- | - |\n";
        assert_eq!(align(&h, 4), None);

        let h = "|  | :-: | :- | - |\n";
        assert_eq!(align(&h, 4), None);
    }

    #[test]
    fn test_records() {
        let h = "| A | B | C |\n| a | b | c |\n| j | k | l |\n";
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let r0 = record!(a, b, c);
        let a = words!(normal_word!("a"));
        let b = words!(normal_word!("b"));
        let c = words!(normal_word!("c"));
        let r1 = record!(a, b, c);
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let r2 = record!(j, k, l);
        let record = vec!(r0, r1, r2);
        let rest = "";
        assert_eq!(records(&h, 3), Some(ParsedResult{token: record, rest}));
    }
    #[test]
    fn test_table() {
        let test = "| A | B | C | \n|-:|--|:-:|\n| a | b | c |\n| j | k | l |\n";
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let he = record!(a, b, c);
    
        let al = vec!(Align::Right, Align::Left, Align::Center);
    
        let a = words!(normal_word!("a"));
        let b = words!(normal_word!("b"));
        let c = words!(normal_word!("c"));
        let r1 = record!(a, b, c);
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let r2 = record!(j, k, l);
        let re = vec!(r1, r2);

        let t = Table{header: he, align: al, records: re};
        let t = Md::Table(Box::new(t));

        assert_eq!(table(&test), Some(ParsedResult{token: t, rest: ""}));
    }
}