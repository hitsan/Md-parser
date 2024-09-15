use app::parser::parser::*;
use app::{items,words,normal_word};

#[test]
fn test_parser() {
    let normal = words!(normal_word!("Hello World!"));
    let bold = words!(Word::Bold(normal));
    let bold_line = Word::Underline(bold);
    let words = words!(bold_line);
    let md = Md::Sentence(words);
    assert_eq!(parse(&"__**Hello World!**__"), vec!(md));

    let normal = words!(normal_word!("Hello World!"));
    let line = words!(Word::Underline(normal));
    let line_bold = Word::Bold(line);
    let words = words!(line_bold);
    let md = Md::Sentence(words);
    assert_eq!(parse(&"**__Hello World!__**"), vec!(md));

    let normal = words!(normal_word!("Hello World!"));
    let line_normal = words!(Word::Underline(normal));
    let bold_line_normal = words!(Word::Bold(line_normal));
    let strike_bold_line_normal = words!(Word::StrikeThough(bold_line_normal));
    let md = Md::Sentence(strike_bold_line_normal);
    assert_eq!(parse(&"~~**__Hello World!__**~~"), vec!(md));

    let hello = normal_word!("Hello ");
    let world = Word::Bold(words!(normal_word!("World!")));
    let word = words!(hello, world);
    let md = Md::Sentence(word);
    assert_eq!(parse(&"Hello **World!**"), vec!(md));

    let normal = words!(normal_word!("Hello World!"));
    let md = Md::Heading(1, normal);
    assert_eq!(parse(&"# Hello World!"), vec!(md));
}

#[test]
fn test_parsing_multiline() {
    let hello_world = words!(normal_word!("Hello World!"));
    let head: Md = Md::Heading(1, hello_world);

    let words = words!(normal_word!("rust parser"));
    let sentence = Md::Sentence(words);
    
    let bold = words!(Word::Bold(words!(normal_word!("lines"))));
    let bold_sentence = Md::Sentence(bold);

    let mds = vec!(head, sentence, bold_sentence);
    assert_eq!(parse(&"# Hello World!\nrust parser\n**lines**"), mds);
}
#[test]
fn test_table() {
    let a = words!(normal_word!("A"));
    let b = words!(normal_word!("B"));
    let c = words!(normal_word!("C"));
    let header = Record(vec!(a, b, c));
    let align = vec!(Align::Right, Align::Left, Align::Center);
    let d = words!(normal_word!("d"));
    let e = words!(normal_word!("e"));
    let f = words!(normal_word!("f"));
    let record0 = Record(vec!(d, e, f));
    let j = words!(normal_word!("j"));
    let k = words!(normal_word!("k"));
    let l = words!(normal_word!("l"));
    let record1 = Record(vec!(j, k, l));
    let records = vec!(record0, record1);
    let md = Md::Table(Box::new(Table{header, align, records}));
    let test_word = "| A | B | C | \n|-:|--|:-:|\n| d | e | f |\n| j | k | l |\n";
    assert_eq!(parse(&test_word), vec!(md));
}

#[test]
fn test_list() {
    let world = words!(normal_word!("World"));
    let item0 = Item(world, items!());
    let children = items!(item0);
    let hello = words!(normal_word!("Hello"));
    let item = Item(hello, children);
    let md = Md::List(items!(item));
    assert_eq!(parse(&"- Hello\n  - World"), vec!(md));
}