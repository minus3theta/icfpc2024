use std::fmt;

use anyhow::{bail, Context, Ok};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOp {
    // - Integer negation
    Neg,
    // \! Boolean not
    Not,
    // # string-to-int: interpret a string as a base-94 number
    ToInt,
    // $ int-to-string: inverse of the above
    ToString,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::ToInt => write!(f, "ToInt"),
            UnaryOp::ToString => write!(f, "ToString"),
        }
    }
}

pub fn decode(mut stream: impl Iterator<Item = u8>) -> anyhow::Result<UnaryOp> {
    let b = stream.next().context("Expected body")?;
    Ok(match b {
        b'-' => UnaryOp::Neg,
        b'!' => UnaryOp::Not,
        b'#' => UnaryOp::ToInt,
        b'$' => UnaryOp::ToString,
        unk => bail!("Unexpected char: {unk}"),
    })
}
