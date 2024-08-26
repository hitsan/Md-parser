use crate::parser::parser::*;
use super::line::terms;

pub fn heading(sentence: &str) -> Option<Md> {
    ["#", "##", "###"].iter().find_map(|p| {
        let sentence = consume(sentence, p)?;
        let sentence = space(sentence)?;
        let tokens = terms(&sentence);
        let ret = Md::Heading(p.len(), tokens);
        Some(ret)
    })
}
