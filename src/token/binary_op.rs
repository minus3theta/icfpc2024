use core::fmt;

use anyhow::{bail, Context, Ok};
use num_bigint::BigInt;

use crate::ast::{Thunk, Value};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Less => "<",
            BinaryOp::Greater => ">",
            BinaryOp::Equal => "=",
            BinaryOp::Or => "|",
            BinaryOp::And => "&",
            BinaryOp::Concat => ".",
            BinaryOp::Take => "T",
            BinaryOp::Drop => "D",
            BinaryOp::Apply => "$",
        };
        write!(f, "{}", op_str)
    }
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

impl BinaryOp {
    fn int_op<R: Into<Value>>(
        lhs: Value,
        rhs: Value,
        fun: impl FnOnce(BigInt, BigInt) -> R,
    ) -> anyhow::Result<Value> {
        use Value::*;
        match (lhs, rhs) {
            (Integer(lhs), Integer(rhs)) => Ok(fun(lhs, rhs).into()),
            (lhs, rhs) => bail!("Expected (Integer, Integer), got ({lhs:?}, {rhs:?})"),
        }
    }

    fn bool_op<R: Into<Value>>(
        lhs: Value,
        rhs: Value,
        fun: impl FnOnce(bool, bool) -> R,
    ) -> anyhow::Result<Value> {
        use Value::*;
        match (lhs, rhs) {
            (Boolean(lhs), Boolean(rhs)) => Ok(fun(lhs, rhs).into()),
            (lhs, rhs) => bail!("Expected (Boolean, Boolean), got ({lhs:?}, {rhs:?})"),
        }
    }

    pub fn apply(&self, lhs: &Thunk, rhs: &Thunk) -> anyhow::Result<Value> {
        use BinaryOp::*;
        use Value::*;

        let lhs = lhs.eval()?;
        match self {
            BinaryOp::Apply => match lhs {
                Value::Closure(var, expr) => expr.subst(var, rhs.clone()).eval(),
                v => bail!("Expected closure: got {:?}", v),
            },
            op => {
                let rhs = rhs.eval()?;
                match op {
                    Add => Self::int_op(lhs, rhs, |x, y| x + y),
                    Sub => Self::int_op(lhs, rhs, |x, y| x - y),
                    Mul => Self::int_op(lhs, rhs, |x, y| x * y),
                    Div => Self::int_op(lhs, rhs, |x, y| x / y),
                    Mod => Self::int_op(lhs, rhs, |x, y| x % y),
                    Less => Self::int_op(lhs, rhs, |x, y| x < y),
                    Greater => Self::int_op(lhs, rhs, |x, y| x > y),
                    Equal => Ok((lhs == rhs).into()),
                    Or => Self::bool_op(lhs, rhs, |x, y| x || y),
                    And => Self::bool_op(lhs, rhs, |x, y| x && y),
                    Concat => match (lhs, rhs) {
                        (String(lhs), String(rhs)) => Ok(format!("{lhs}{rhs}").into()),
                        (lhs, rhs) => bail!("Expected (String, String), got ({lhs:?}, {rhs:?})"),
                    },
                    Take => match (lhs, rhs) {
                        (Integer(lhs), String(rhs)) => Ok(rhs
                            .chars()
                            .take(usize::try_from(lhs)?)
                            .collect::<std::string::String>()
                            .into()),
                        (lhs, rhs) => bail!("Expected (Integer, String), got ({lhs:?}, {rhs:?})"),
                    },
                    Drop => match (lhs, rhs) {
                        (Integer(lhs), String(rhs)) => Ok(rhs
                            .chars()
                            .skip(usize::try_from(lhs)?)
                            .collect::<std::string::String>()
                            .into()),
                        (lhs, rhs) => bail!("Expected (Integer, String), got ({lhs:?}, {rhs:?})"),
                    },
                    Apply => unreachable!(),
                }
            }
        }
    }
}
