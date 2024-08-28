use crate::parser::parser::*;

// fn repeate() -> {
    
// }

pub fn table(sentence: &str) -> Option<Md> {
    let header = Emphasis::Text("Hello".to_string());
    let header = TableHeader(vec!(vec!(header)));

    let data = Emphasis::Text("world".to_string());
    let data = TableData(vec!(vec!(vec!(data))));
    Some(Md::Table(1, header, data))
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