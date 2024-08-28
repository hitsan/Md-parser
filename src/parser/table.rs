use crate::parser::parser::*;
use super::line::terms;

fn header(sentence: &str) -> Option<TableHeader> {
    if !sentence.starts_with("|") && !sentence.ends_with("|") {
        return None
    }
    let len = sentence.len();
    let sentence = &sentence[1..(len-1)];
    let sentences = sentence.split("|");
    let mut ret: Vec<Vec<Emphasis>> = vec!();
    for s in sentences {
        if s == "" { return None }
        ret.push(terms(s));
    }
    Some(TableHeader(ret))
}

pub fn table(sentence: &str) -> Option<Md> {
    let h = header(sentence)?;

    let data = Emphasis::Text("world".to_string());
    let data = TableData(vec!(vec!(vec!(data))));
    Some(Md::Table(1, h, data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let sentence = "
            | Hello |
            | - |
            | world |";
        let header = Emphasis::Text("Hello".to_string());
        let header = TableHeader(vec!(vec!(header)));
    
        let data = Emphasis::Text("world".to_string());
        let data = TableData(vec!(vec!(vec!(data))));
        let expectation = Some(Md::Table(1, header, data));
        assert_eq!(table(&sentence), expectation);
    }
}