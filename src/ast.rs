use std::{cell::RefCell, rc::Rc, str::FromStr};

use anyhow::{bail, Context};

use crate::token::{decode_token_stream, BinaryOp, Token, UnaryOp};

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
    If(Thunk, Thunk, Thunk),
    Lambda(i64, Thunk),
    Var(i64),
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

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = decode_token_stream(s)?;
        Self::from_tokens(&tokens)
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
                Token::If => {
                    let flag = stack.pop().context("No operand for If")?;
                    let case_true = stack.pop().context("No operand for If")?;
                    let case_false = stack.pop().context("No operand for If")?;
                    stack.push(Expr::If(flag.into(), case_true.into(), case_false.into()));
                }
                &Token::Lambda(v) => {
                    let body = stack.pop().context("No body for lambda")?;
                    stack.push(Expr::Lambda(v, body.into()))
                }
                &Token::Variable(v) => stack.push(Expr::Var(v)),
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
            Expr::BinaryOp(o, lhs, rhs) => o.apply(lhs, rhs, env),
            Expr::UnaryOp(o, e) => o.apply(e, env),
            Expr::If(flag, t, f) => match flag.eval(env)? {
                Value::Boolean(true) => t.eval(env),
                Value::Boolean(false) => f.eval(env),
                v => bail!("Expected boolean: got {v:?}"),
            },
            &Expr::Lambda(var, ref body) => Ok(Value::Closure(env.clone(), var, body.clone())),
            &Expr::Var(var) => env
                .iter()
                .rev()
                .find_map(|&(v, ref t)| (v == var).then(|| t.clone()))
                .context("Undefined variable: {var}")?
                .eval(env),
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
