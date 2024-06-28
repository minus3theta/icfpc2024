use core::fmt;

use anyhow::{bail, Context, Ok};

use crate::ast::{Env, Thunk, Value};

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
            BinaryOp::Take => "Take",
            BinaryOp::Drop => "Drop",
            BinaryOp::Apply => "Apply",
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
        fun: impl FnOnce(i64, i64) -> R,
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

    pub fn apply(&self, lhs: &Thunk, rhs: &Thunk, env: &Env) -> anyhow::Result<Value> {
        use BinaryOp::*;
        use Value::*;

        let lhs = lhs.eval(env)?;
        match self {
            BinaryOp::Apply => match lhs {
                Value::Closure(mut env0, var, expr) => {
                    env0.push((var, rhs.clone()));
                    let ret = expr.eval(&env0)?;
                    env0.pop();
                    Ok(ret)
                }
                v => bail!("Expected closure: got {:?}", v),
            },
            op => {
                let rhs = rhs.eval(env)?;
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
                    Take => todo!(),
                    Drop => todo!(),
                    Apply => unreachable!(),
                }
            }
        }
    }
}
