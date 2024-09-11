use crate::parser::parser::{Md,Word,Words};

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

fn to_html(md: Md) -> String {
    match md {
        Md::Heading(size, words) => format!("<h{}>{}</h{}>", size, convert_words(&words), size),
        Md::Sentence(words) => convert_words(&words),
        _ => panic!("testteafdsaf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{normal_word,words};

    #[test]
    fn test_to_html() {
        let words = words!(normal_word!("Hello"));
        let md = Md::Heading(1, words);
        assert_eq!(to_html(md), "<h1>Hello</h1>".to_string());

        let words = words!(normal_word!("Hello"));
        let md = Md::Sentence(words);
        assert_eq!(to_html(md), "Hello".to_string());
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
}