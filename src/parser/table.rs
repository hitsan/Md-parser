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
    let mut records:Vec<Record> = vec!();
    while let Some(result) = record(texts, &|text| words(text)) 
    {
        texts = result.rest;
        let cells = result.token;
        if cells.len()!=n { break; }
        let record = Record(cells);
        records.push(record);
    }
    if records.is_empty() { return None }
    Some(ParsedResult::new(records, texts))
}
fn record_len(record: &Record) -> usize {
    match record {
        Record(r) => r.len()
    }
}

pub fn table(texts: &str) -> Option<ParsedResult<Md>> {
    let header_result = header(texts)?;
    let header = header_result.token;
    let column_num = record_len(&header);

    let align_result = align(header_result.rest, column_num)?;
    let align = align_result.token;

    let records_result = records(align_result.rest, column_num)?;
    let records = records_result.token;

    let token = Md::Table(Box::new(Table{header, align, records}));
    Some(ParsedResult::new(token, records_result.rest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{words,record,normal_word};

    #[test]
    fn test_header() {
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let token = record!(a, b, c);
        let rest = "";
        assert_eq!(header(&"| A | B | C | \n"), Some(ParsedResult{token, rest}));

        let nul = words!(normal_word!(""));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let token = record!(nul, b, c);
        let rest = "";
        assert_eq!(header(&"|  | B | C |\n"), Some(ParsedResult{token, rest}));
        assert_eq!(header(&"| A | B | C \n"), None);
    }

    #[test]
    fn test_align() {
        let token = vec!(Align::Right, Align::Center, Align::Left, Align::Left);
        let rest = "";
        assert_eq!(align(&"| -: | :-: | :- | --- |\n", 4), Some(ParsedResult{token, rest}));
        assert_eq!(align(&"| -: | :-b: | :- | - |\n", 4), None);
        assert_eq!(align(&"|  | :-: | :- | - |\n", 4), None);
    }

    #[test]
    fn test_records() {
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let record0 = record!(a, b, c);
        let a = words!(normal_word!("a"));
        let b = words!(normal_word!("b"));
        let c = words!(normal_word!("c"));
        let record1 = record!(a, b, c);
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let record2 = record!(j, k, l);
        let token = vec!(record0, record1, record2);
        let rest = "";
        assert_eq!(records(&"| A | B | C |\n| a | b | c |\n| j | k | l |\n", 3), Some(ParsedResult{token, rest}));
    }
    #[test]
    fn test_table() {
        let a = words!(normal_word!("A"));
        let b = words!(normal_word!("B"));
        let c = words!(normal_word!("C"));
        let header = record!(a, b, c);
        let align = vec!(Align::Right, Align::Left, Align::Center);
    
        let a = words!(normal_word!("a"));
        let b = words!(normal_word!("b"));
        let c = words!(normal_word!("c"));
        let record0 = record!(a, b, c);
        let j = words!(normal_word!("j"));
        let k = words!(normal_word!("k"));
        let l = words!(normal_word!("l"));
        let record1 = record!(j, k, l);
        let records = vec!(record0, record1);

        let token = Md::Table(Box::new(Table{header, align, records}));
        let rest = "";
        assert_eq!(table(&"| A | B | C | \n|-:|--|:-:|\n| a | b | c |\n| j | k | l |\n"), Some(ParsedResult{token, rest}));
    }
}