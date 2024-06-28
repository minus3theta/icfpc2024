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
