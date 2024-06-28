pub mod strings;

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

pub fn decode_token(s: &str) -> anyhow::Result<Token> {
    let mut bytes = s.bytes();
    match bytes.next().expect("Token is empty") {
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
}
