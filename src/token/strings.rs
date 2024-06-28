use anyhow::Context;
use itertools::Itertools;

const ALPHABETS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn decode(stream: impl Iterator<Item = u8>) -> anyhow::Result<String> {
    Ok(stream
        .map(|b| {
            *ALPHABETS
                .get((b - b'!') as usize)
                .expect("Invalid charcter") as char
        })
        .collect())
}

pub fn encode(s: &str) -> anyhow::Result<String> {
    s.bytes()
        .map(|b| -> anyhow::Result<char> {
            let (pos, _) = ALPHABETS
                .iter()
                .find_position(|&&x| x == b)
                .context("Invalid character")?;
            Ok((pos as u8 + b'!') as char)
        })
        .collect()
}
