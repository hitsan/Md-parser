use crate::parser::parser::*;

fn word_to_html<'a>(word: &'a Word) -> String {
    match word {
        Word::Normal(val) => val.clone(),
        Word::Italic(words) => format!("<i>{}</i>", words_to_html(&words)),
        Word::Bold(words) => format!("<b>{}</b>", words_to_html(&words)),
        Word::StrikeThough(words) => format!("<s>{}</s>", words_to_html(&words)),
        Word::Underline(words) => format!("<u>{}</u>", words_to_html(&words)),
    }
}

fn words_to_html<'a>(words: &'a Words) -> String {
    let words = &words.0;
    words.iter().map(|word| word_to_html(word))
        .collect::<Vec<String>>()
        .join("")
}

fn header_to_html(record: &Record) -> String {
    let header = &record.0;
    header.iter().map(
        |words| format!("<th>{}</th>", words_to_html(words))
    )
    .collect::<Vec<String>>()
    .join("")
}

fn align_to_string<'a>(align: &Align) -> &'a str {
    match align {
        Align::Right => "right",
        Align::Center => "center",
        Align::Left => "left",
    }
}

fn record_to_html(record: &Record, aligns: &Vec<Align>) -> String {
    let record = &record.0;
    record.iter().zip(aligns.iter()).map(
        |(words, align)| {
            let align = align_to_string(align);
            format!("<td align=\"{}\">{}</td>", align, words_to_html(words))
    })
    .collect::<Vec<String>>()
    .join("")
}

fn records_to_html(records: &Vec<Record>, aligns: &Vec<Align>) -> String {
    records.iter().map(|record| {
        format!("<tr>{}</tr>\n", record_to_html(record, aligns))
    })
    .collect::<Vec<String>>()
    .join("")
}

fn table_to_html(table: &Box<Table>) -> String {
    let header = &table.header;
    let aligns = &table.align;
    let records = &table.records;

    let header = header_to_html(header);
    let header = format!("<tr>{}</tr>", header);
    let records = records_to_html(records, aligns);
    format!("<table>\n{}\n{}</table>\n", header, records)
}

fn item_to_html(item: &Item) -> String {
    let words = &item.0;
    let words = words_to_html(words);
    let children = &item.1;
    let children = if children.0.is_empty() {
        "".to_string()
    } else {
        format!("\n{}", items_to_html(children))
    };
    format!("<li>{}{}</li>", words, children)
}

fn items_to_html(items: &Items) -> String {
    let items = &items.0;
    let strings: Vec<String> = items.iter().map(|item| item_to_html(item)).collect();
    let html = strings.join("\n");
    format!("<ul>\n{}\n</ul>\n", html)
}

fn heading_to_html(size: &usize, words: &Words) -> String {
    format!("<h{}>{}</h{}>", size, words_to_html(&words), size)
}

fn sentence_to_html(words: &Words) -> String {
    format!("{}<br />", words_to_html(&words))
}

fn md_to_html(md: &Md) -> String {
    match md {
        Md::Heading(size, words) => heading_to_html(size, words),
        Md::Sentence(words) => sentence_to_html(&words),
        Md::Table(table) => table_to_html(&table),
        Md::List(items) => items_to_html(&items),
    }
}

pub fn mds_to_html(mds: &Vec<Md>) -> String {
    let strings = mds.iter().map(|md| md_to_html(md));
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
        assert_eq!(mds_to_html(&mds), "<h1>Heading</h1>\nHello<br />\nWorld<br />".to_string());
    }

    #[test]
    fn test_to_html() {
        let words = words!(normal_word!("Hello"));
        let md = Md::Heading(1, words);
        assert_eq!(md_to_html(&md), "<h1>Hello</h1>".to_string());

        let words = words!(normal_word!("Hello"));
        let md = Md::Sentence(words);
        assert_eq!(md_to_html(&md), "Hello<br />".to_string());

        let words = words!(normal_word!("item"));
        let items = items!();
        let item = Item(words, items);
        let items = items!(item);
        let md = Md::List(items);
        assert_eq!(md_to_html(&md), "<ul>\n<li>item</li>\n</ul>\n".to_string());
    }

    #[test]
    fn test_word() {
        let word = normal_word!("Hello");
        assert_eq!(word_to_html(&word), "Hello".to_string());

        let word = normal_word!("Hello");
        let italic = Word::Italic(words!(word));
        assert_eq!(word_to_html(&italic), "<i>Hello</i>".to_string());

        let word = normal_word!("Hello");
        let bold = Word::Bold(words!(word));
        assert_eq!(word_to_html(&bold), "<b>Hello</b>".to_string());

        let word = normal_word!("Hello");
        let strike = Word::StrikeThough(words!(word));
        assert_eq!(word_to_html(&strike), "<s>Hello</s>".to_string());

        let word = normal_word!("Hello");
        let line = Word::Underline(words!(word));
        assert_eq!(word_to_html(&line), "<u>Hello</u>".to_string());
    }

    #[test]
    fn test_words_to_html() {
        let word = normal_word!("Hello");
        let word1 = normal_word!("World!");
        let bold = Word::Bold(words!(word1));
        let words = words!(word, bold);
        assert_eq!(words_to_html(&words), "Hello<b>World!</b>".to_string());
    }

    #[test]
    fn test_header_to_html() {
        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let header = Record(vec!(hello, world));
        assert_eq!(header_to_html(&header), "<th>hello</th><th>world</th>".to_string());
    }

    #[test]
    fn test_record_to_html() {
        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let record = Record(vec!(hello, world));
        let align = vec!(Align::Left, Align::Left);
        assert_eq!(record_to_html(&record, &align), "<td align=\"left\">hello</td><td align=\"left\">world</td>".to_string());

        let hello = words!(normal_word!("hello"));
        let world = words!(normal_word!("world"));
        let record = Record(vec!(hello, world));
        let align = vec!(Align::Center, Align::Right);
        assert_eq!(record_to_html(&record, &align), "<td align=\"center\">hello</td><td align=\"right\">world</td>".to_string());
    }

    #[test]
    fn test_records_to_html() {
        let hello = words!(normal_word!("hello"));
        let record0 = Record(vec!(hello));
        let world = words!(normal_word!("world"));
        let record1 = Record(vec!(world));
        let records = vec!(record0, record1);
        let aligns = vec!(Align::Left);
        assert_eq!(records_to_html(&records, &aligns), "<tr><td align=\"left\">hello</td></tr>\n<tr><td align=\"left\">world</td></tr>\n".to_string());
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
        assert_eq!(table_to_html(&table), "<table>\n<tr><th>hello</th></tr>\n<tr><td align=\"left\">world</td></tr>\n</table>\n".to_string());
    }

    #[test]
    fn test_item_to_html() {
        let words = words!(normal_word!("item"));
        let items = items!();
        let item = Item(words, items);
        assert_eq!(item_to_html(&item), "<li>item</li>".to_string());

        let words = words!(normal_word!("parent"));
        let words0 = words!(normal_word!("item"));
        let item0 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item1 = Item(words1, items!());
        let items = items!(item0, item1);
        let item = Item(words, items);
        let expect = "<li>parent\n<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n</li>".to_string();
        assert_eq!(item_to_html(&item), expect);
    }

    #[test]
    fn test_items_to_html() {
        let words0 = words!(normal_word!("item"));
        let item0 = Item(words0, items!());
        let words1 = words!(normal_word!("item1"));
        let item1 = Item(words1, items!());
        let items = items!(item0, item1);
        assert_eq!(items_to_html(&items), "<ul>\n<li>item</li>\n<li>item1</li>\n</ul>\n".to_string());

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
        assert_eq!(items_to_html(&items), expect);

    }
}