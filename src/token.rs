use anyhow::Context;

mod integers;
mod strings;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Boolean(bool),
    Integer(i64),
    String(String),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOp {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOp {}

pub fn decode_token_stream(s: &str) -> anyhow::Result<Vec<Token>> {
    s.split_ascii_whitespace().map(decode_token).collect()
}

pub fn decode_token(s: &str) -> anyhow::Result<Token> {
    let mut bytes = s.bytes();
    match bytes.next().context("Token is empty")? {
        b'T' => Ok(bytes
            .next()
            .is_none()
            .then_some(Token::Boolean(true))
            .context("Unexpected body after T")?),
        b'F' => Ok(bytes
            .next()
            .is_none()
            .then_some(Token::Boolean(false))
            .context("Unexpected body after F")?),
        b'I' => integers::decode(bytes).map(Token::Integer),
        b'S' => strings::decode(bytes).map(Token::String),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_hello_world() -> anyhow::Result<()> {
        assert_eq!(
            decode_token("SB%,,/}Q/2,$_")?,
            Token::String("Hello World!".into())
        );
        Ok(())
    }

    #[test]
    fn decode_1337() -> anyhow::Result<()> {
        assert_eq!(decode_token("I/6")?, Token::Integer(1337));
        Ok(())
    }
}
