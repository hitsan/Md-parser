use crate::parser::parser::*;

fn convert_word<'a>(word: &'a Word) -> String {
    match word {
        Word::Normal(val) => val.clone(),
        Word::Italic(words) => format!("<i>{}</i>", convert_words(&words)),
        Word::Bold(words) => format!("<b>{}</b>", convert_words(&words)),
        Word::StrikeThough(words) => format!("<s>{}</s>", convert_words(&words)),
        Word::Underline(words) => format!("<u>{}</u>", convert_words(&words)),
    }
}

fn convert_words<'a>(words: &'a Words) -> String {
    words.0
        .iter()
        .fold(
            "".to_string(),
            |html, word| format!("{}{}", html, convert_word(word))
        )
}

fn convert_cells(record: &Record, tag: &str) -> String {
    record.0
        .iter()
        .fold(
            "".to_string(),
            |html, words| format!("{}<{}>{}</{}>", html, tag, convert_words(words), tag)
        )
}

fn convert_header(record: &Record) -> String {
    record.0
    .iter()
    .fold(
        "".to_string(),
        |html, words| format!("{}<th>{}</th>", html, convert_words(words))
    )
}

fn convert_record(record: &Record, aligns: &Vec<Align>) -> String {
    record.0.iter().zip(aligns.iter())
        .fold(
            "".to_string(),
            |html, (words, align)| {
                let a = match align {
                    Align::Right => "right",
                    Align::Center => "center",
                    Align::Left => "left",
                };
                format!("{}<td align=\"{}\">{}</td>", html, a, convert_words(words))
            }
        )
}

fn convert_records(records: &Vec<Record>, aligns: &Vec<Align>) -> String {
    records
        .iter()
        .fold(
            "".to_string(),
            |html, record| format!("{}<tr>{}</tr>\n", html, convert_record(record, &aligns))
        )
}

fn convert_table(table: &Box<Table>) -> String {
    let header = &table.header;
    let aligns = &table.align;
    let records = &table.records;

    let header = convert_header(header);
    let header = format!("<tr>{}</tr>", header);
    let records = convert_records(records, aligns);
    format!("<table>\n{}\n{}</table>\n", header, records)
}

fn convert_item(item: &Item) -> String {
    let words = &item.0;
    let words = convert_words(words);
    let children = &item.1;
    let children = if children.0.is_empty() {
        "".to_string()
    } else {
        format!("\n{}", convert_items(children))
    };
    format!("<li>{}{}</li>", words, children)
}

fn convert_items(items: &Items) -> String {
    let items = items.0.iter().fold(
        "".to_string(),
        |html, item| 
        format!("{}{}\n", html, convert_item(item))
    );
    format!("<ul>\n{}</ul>\n", items)
}

fn heading_to_html(size: &usize, words: &Words) -> String {
    format!("<h{}>{}</h{}>", size, convert_words(&words), size)
}

fn to_html(md: &Md) -> String {
    match md {
        Md::Heading(size, words) => heading_to_html(size, words),
        Md::Sentence(words) => convert_words(&words),
        Md::Table(table) => convert_table(&table),
        Md::List(items) => convert_items(&items),
        _ => panic!("testteafdsaf")
    }
}

pub fn mds_to_html(mds: &Vec<Md>) -> String {
    let strings = mds.iter().map(|md| to_html(md));
    let strings: Vec<String> = strings.collect();
    strings.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{normal_word,words,items};

    #[test]
    fn test_mds_to_html() {
        let words = words!(normal_word!("Heading"));
        let heading = Md::Heading(1, words);
        let words = words!(normal_word!("Hello"));
        let hello_sentence = Md::Sentence(words);
        let words = words!(normal_word!("World"));
        let world_sentence = Md::Sentence(words);

        let mds = vec!(heading, hello_sentence, world_sentence);
        assert_eq!(mds_to_html(&mds), "<h1>Heading</h1>\nHello\nWorld".to_string());
    }

    #[test]
    fn test_to_html() {
        let words = words!(normal_word!("Hello"));
        let md = Md::Heading(1, words);
        assert_eq!(to_html(&md), "<h1>Hello</h1>".to_string());

        let words = words!(normal_word!("Hello"));
        let md = Md::Sentence(words);
        assert_eq!(to_html(&md), "Hello".to_string());

        let words = words!(normal_word!("item"));
        let items = items!();
        let item = Item(words, items);
        let items = items!(item);
        let md = Md::List(items);
        assert_eq!(to_html(&md), "<ul>\n<li>item</li>\n</ul>\n".to_string());
    }

    #[test]
    fn test_word() {
        let word = normal_word!("Hello");
        assert_eq!(convert_word(&word), "Hello".to_string());

        let word = normal_word!("Hello");
        let italic = Word::Italic(words!(word));
        assert_eq!(convert_word(&italic), "<i>Hello</i>".to_string());

        let word = normal_word!("Hello");
        let bold = Word::Bold(words!(word));
        assert_eq!(convert_word(&bold), "<b>Hello</b>".to_string());

        let word = normal_word!("Hello");
        let strike = Word::StrikeThough(words!(word));
        assert_eq!(convert_word(&strike), "<s>Hello</s>".to_string());

        let word = normal_word!("Hello");
        let line = Word::Underline(words!(word));
        assert_eq!(convert_word(&line), "<u>Hello</u>".to_string());
    }

    #[test]
    fn test_words_to_html() {
        let word = normal_word!("Hello");
        let word1 = normal_word!("World!");
        let bold = Word::Bold(words!(word1));
        let words = words!(word, bold);
        assert_eq!(convert_words(&words), "Hello<b>World!</b>".to_string());
    }

    #[test]
    fn test_header_to_html() {
        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let header = Record(vec!(hello, world));
        assert_eq!(convert_header(&header), "<th>hello</th><th>world</th>".to_string());
    }

    #[test]
    fn test_record_to_html() {
        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let record = Record(vec!(hello, world));
        let align = vec!(Align::Left, Align::Left);
        assert_eq!(convert_record(&record, &align), "<td align=\"left\">hello</td><td align=\"left\">world</td>".to_string());

        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let record = Record(vec!(hello, world));
        let align = vec!(Align::Center, Align::Right);
        assert_eq!(convert_record(&record, &align), "<td align=\"center\">hello</td><td align=\"right\">world</td>".to_string());
    }

    #[test]
    fn test_records_to_html() {
        let hello = words!(normal_word!("hello"));
        let record0 = Record(vec!(hello));
        let world = words!(normal_word!("world"));
        let record1 = Record(vec!(world));
        let records = vec!(record0, record1);
        let aligns = vec!(Align::Left);
        assert_eq!(convert_records(&records, &aligns), "<tr><td align=\"left\">hello</td></tr>\n<tr><td align=\"left\">world</td></tr>\n".to_string());
    }

    #[test]
    fn test_table_to_html() {
        let hello = words!(normal_word!("hello"));
        let header = Record(vec!(hello));
        let world = words!(normal_word!("world"));
        let record = Record(vec!(world));
        let records = vec!(record);
        let aligns = vec!(Align::Left);
        let table = Box::new(Table{header, align: aligns, records});
        assert_eq!(convert_table(&table), "<table>\n<tr><th>hello</th></tr>\n<tr><td align=\"left\">world</td></tr>\n</table>\n".to_string());
    }

    #[test]
    fn test_item_to_html() {
        let words = words!(normal_word!("item"));
        let items = items!();
        let item = Item(words, items);
        assert_eq!(convert_item(&item), "<li>item</li>".to_string());

        let words = words!(normal_word!("parent"));
        let words0 = words!(normal_word!("item"));
        let item0 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item1 = Item(words1, items!());
        let items = items!(item0, item1);
        let item = Item(words, items);
        let expect = "<li>parent\n<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n</li>".to_string();
        assert_eq!(convert_item(&item), expect);
    }

    #[test]
    fn test_items_to_html() {
        let words0 = words!(normal_word!("item"));
        let item0 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item1 = Item(words1, items!());
        let items = items!(item0, item1);
        assert_eq!(convert_items(&items), "<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n".to_string());

        let words = words!(normal_word!("parent"));
        let words0 = words!(normal_word!("item"));
        let item0 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item1 = Item(words1, items!());
        let items = items!(item0, item1);
        let item0 = Item(words, items);
    
        let words = words!(normal_word!("parent"));
        let words0 = words!(normal_word!("item"));
        let item2 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item3 = Item(words1, items!());
        let items = items!(item2, item3);
        let item1 = Item(words, items);
        let items = items!(item0, item1);
        let expect = "<ul>\n<li>parent\n<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n</li>\n<li>parent\n<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n</li>\n</ul>\n".to_string();
        assert_eq!(convert_items(&items), expect);

    }
}