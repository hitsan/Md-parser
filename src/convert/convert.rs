use crate::parser::parser::{Md,Word,Words};

fn convert_word<'a>(word: &'a Word) -> &'a String {
    match word {
        Word::Normal(val) => val,
        _ => panic!("illegal word!")
    }
}

fn convert_words<'a>(words: &'a Words) -> &'a String {
    match words.0.first() {
        Some(val) => convert_word(val),
        _ => panic!("illegal words!")
    }
}

fn html(md: Md) -> String {
    match md {
        Md::Heading(size, words) => format!("<h{}>{}</h{}>", size, convert_words(&words), size),
        _ => panic!("testteafdsaf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{normal_word,words};

    #[test]
    fn test_html() {
        let words = words!(normal_word!("Hello"));
        let md = Md::Heading(1, words);
        assert_eq!(html(md), "<h1>Hello</h1>".to_string());
    }

    fn test_word() {
        let word = normal_word!("Hello");
        assert_eq!(convert_word(&word), &"Hello".to_string());
    }
}