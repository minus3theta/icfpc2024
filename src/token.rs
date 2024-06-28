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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::String(s) => s.fmt(f),
            _ => todo!(),
        }
    }
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
        _ => Err(anyhow::anyhow!("Unknown token type")),
    }
}

pub fn encode(tokens: &[Token]) -> anyhow::Result<String> {
    Ok(tokens
        .iter()
        .map(encode_token)
        .collect::<Result<Vec<_>, _>>()?
        .join(" "))
}

fn encode_token(token: &Token) -> anyhow::Result<String> {
    match token {
        Token::Boolean(_) => todo!(),
        Token::Integer(_) => todo!(),
        Token::String(s) => Ok(format!("S{}", strings::encode(s)?)),
        Token::UnaryOp(_) => todo!(),
        Token::BinaryOp(_) => todo!(),
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

    #[test]
    fn encode_hello_world() -> anyhow::Result<()> {
        assert_eq!(
            encode(&[Token::String("Hello World!".into())])?,
            "SB%,,/}Q/2,$_".to_owned()
        );
        Ok(())
    }
}
