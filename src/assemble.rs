use crate::token::{integers, strings};
use anyhow::Context;

fn parse_mnemonic(mn: &str) -> anyhow::Result<String> {
    if mn.chars().all(|c| c.is_ascii() && c.is_numeric()) {
        Ok(format!("I{}", integers::encode(mn.parse()?)?))
    } else if mn.starts_with('"') {
        let (_, mn) = mn.split_once('"').expect("unreachable");
        let (mn, _) = mn
            .rsplit_once('"')
            .context("String literal does not end with '\"'")?;
        Ok(format!("S{}", strings::encode(mn)?))
    } else if mn == "Y" {
        Ok(r#"L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v#"#.to_owned())
    } else {
        Ok(mn.to_owned())
    }
}

pub fn assemble(asm: &str) -> anyhow::Result<Vec<String>> {
    asm.split_ascii_whitespace().map(parse_mnemonic).collect()
}
