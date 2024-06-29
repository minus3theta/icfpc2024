use std::fmt;

use anyhow::{bail, Context, Ok};

use crate::{
    ast::{Thunk, Value},
    token::strings,
};

use super::integers;

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

impl UnaryOp {
    pub fn apply(&self, e: &Thunk) -> anyhow::Result<Value> {
        let value = e.eval()?;
        match self {
            UnaryOp::Neg => {
                if let Value::Integer(i) = value {
                    Ok((-i).into())
                } else {
                    bail!("Expected integer: got {value:?}")
                }
            }
            UnaryOp::Not => {
                if let Value::Boolean(b) = value {
                    Ok((!b).into())
                } else {
                    bail!("Expected boolean: got {value:?}")
                }
            }
            UnaryOp::ToInt => {
                if let Value::String(s) = value {
                    Ok(integers::decode(strings::encode(&s)?.bytes())?.into())
                } else {
                    bail!("Expected string: got {value:?}")
                }
            }
            UnaryOp::ToString => {
                if let Value::Integer(i) = value {
                    Ok(strings::decode(integers::encode(i)?.bytes())?.into())
                } else {
                    bail!("Expected integer: got {value:?}")
                }
            }
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::ToInt => write!(f, "#"),
            UnaryOp::ToString => write!(f, "$"),
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
