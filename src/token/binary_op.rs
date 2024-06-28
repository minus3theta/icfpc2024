use anyhow::{bail, Context, Ok};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Less,
    Greater,
    Equal,
    Or,
    And,
    Concat,
    Take,
    Drop,
    Apply,
}

pub fn decode(mut stream: impl Iterator<Item = u8>) -> anyhow::Result<BinaryOp> {
    let b = stream.next().context("Expected body")?;
    Ok(match b {
        b'+' => BinaryOp::Add,
        b'-' => BinaryOp::Sub,
        b'*' => BinaryOp::Mul,
        b'/' => BinaryOp::Div,
        b'%' => BinaryOp::Mod,
        b'<' => BinaryOp::Less,
        b'>' => BinaryOp::Greater,
        b'=' => BinaryOp::Equal,
        b'|' => BinaryOp::Or,
        b'&' => BinaryOp::And,
        b'.' => BinaryOp::Concat,
        b'T' => BinaryOp::Take,
        b'D' => BinaryOp::Drop,
        b'$' => BinaryOp::Apply,
        _ => bail!("Unexpected char"),
    })
}
