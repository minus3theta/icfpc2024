use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Context};

use crate::token::{BinaryOp, Token, UnaryOp};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThunkEnum {
    Expr(Expr),
    Value(Value),
}

impl From<Value> for ThunkEnum {
    fn from(value: Value) -> Self {
        ThunkEnum::Value(value)
    }
}

impl Thunk {
    pub fn eval(&self, env: &Env) -> anyhow::Result<Value> {
        let mut t = self.0.borrow_mut();
        match &*t {
            ThunkEnum::Expr(e) => {
                let v = e.eval(env)?;
                *t = v.clone().into();
                Ok(v)
            }
            ThunkEnum::Value(v) => Ok(v.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thunk(Rc<RefCell<ThunkEnum>>);

impl From<Expr> for Thunk {
    fn from(value: Expr) -> Self {
        Thunk(Rc::new(RefCell::new(ThunkEnum::Expr(value))))
    }
}

pub type Env = Vec<(i64, Thunk)>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Value),
    UnaryOp(UnaryOp, Thunk),
    BinaryOp(BinaryOp, Thunk, Thunk),
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Literal(value)
    }
}

impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Value::from(value).into()
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Value::from(value).into()
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Value::from(value).into()
    }
}

impl Expr {
    pub fn from_tokens(tokens: &[Token]) -> anyhow::Result<Self> {
        let mut stack: Vec<Self> = vec![];
        for token in tokens.iter().rev() {
            match token {
                &Token::Boolean(v) => stack.push(v.into()),
                &Token::Integer(v) => stack.push(v.into()),
                Token::String(v) => stack.push(v.clone().into()),
                Token::UnaryOp(o) => {
                    let top = stack.pop().context("No operand for UnaryOp")?;
                    stack.push(Expr::UnaryOp(o.clone(), top.into()));
                }
                Token::BinaryOp(o) => {
                    let x = stack.pop().context("No operand for BinaryOp")?;
                    let y = stack.pop().context("No operand for BinaryOp")?;
                    stack.push(Expr::BinaryOp(o.clone(), x.into(), y.into()));
                }
                Token::If() => todo!(),
                Token::Lambda(_) => todo!(),
                Token::Variable(_) => todo!(),
            }
        }
        let expr = stack.pop().context("Empty expression")?;
        if !stack.is_empty() {
            bail!("Expr remains");
        }
        Ok(expr)
    }

    pub fn eval(&self, env: &Env) -> anyhow::Result<Value> {
        match self {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::BinaryOp(BinaryOp::Apply, fun, arg) => match fun.eval(env)? {
                Value::Closure(mut env0, var, expr) => {
                    env0.push((var, arg.clone()));
                    let ret = expr.eval(&env0)?;
                    env0.pop();
                    Ok(ret)
                }
                v => bail!("Expected closure: got {:?}", v),
            },
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    String(String),
    Closure(Env, i64, Thunk),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_
// }
