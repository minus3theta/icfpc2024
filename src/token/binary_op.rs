use core::fmt;

use anyhow::{bail, Context, Ok};

use crate::ast::Expr;

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
    pub fn eval(&self, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Expr> {
        todo!()
    }
}
