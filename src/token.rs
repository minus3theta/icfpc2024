use anyhow::{bail, Context};

use binary_op::BinaryOp;
use unary_op::UnaryOp;

mod binary_op;
mod integers;
mod strings;
mod unary_op;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Boolean(bool),
    Integer(i64),
    String(String),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    If(),
    Lambda(i64),
    Variable(i64),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Boolean(b) => b.fmt(f),
            Token::Integer(i) => i.fmt(f),
            Token::String(s) => s.fmt(f),
            Token::UnaryOp(op) => unimplemented!("UnaryOp"),
            Token::BinaryOp(op) => op.fmt(f),
            Token::If() => write!(f, "?"),
            Token::Lambda(i) => write!(f, "L{i}"),
            Token::Variable(i) => write!(f, "v{i}"),
        }
    }
}

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
        b'B' => binary_op::decode(bytes).map(Token::BinaryOp),
        b'?' => Ok(Token::If()),
        b'L' => integers::decode(bytes).map(Token::Lambda),
        b'v' => integers::decode(bytes).map(Token::Variable),
        unk => bail!("Unknown token: {}", unk as char),
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
        Token::If() => todo!(),
        Token::Lambda(_) => todo!(),
        Token::Variable(_) => todo!(),
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
    fn decode_binary_op() -> anyhow::Result<()> {
        assert_eq!(decode_token("B+")?, Token::BinaryOp(BinaryOp::Add));
        assert_eq!(decode_token("B-")?, Token::BinaryOp(BinaryOp::Sub));
        Ok(())
    }

    #[test]
    fn decode_if() -> anyhow::Result<()> {
        assert_eq!(decode_token("?")?, Token::If());
        Ok(())
    }

    #[test]
    fn decode_lambda() -> anyhow::Result<()> {
        assert_eq!(decode_token("L#")?, Token::Lambda(2));
        Ok(())
    }

    #[test]
    fn decode_variable() -> anyhow::Result<()> {
        assert_eq!(decode_token("v\"")?, Token::Variable(1));
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
