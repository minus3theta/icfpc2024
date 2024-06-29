use std::{cell::RefCell, rc::Rc, str::FromStr};

use anyhow::{bail, Context};

use crate::token::{decode_token_stream, BinaryOp, Token, UnaryOp};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThunkEnum {
    Expr(Expr, Env),
    Value(Value),
}

impl From<Value> for ThunkEnum {
    fn from(value: Value) -> Self {
        ThunkEnum::Value(value)
    }
}

impl std::fmt::Display for Thunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0.borrow() {
            ThunkEnum::Expr(e, _) => e.fmt(f),
            ThunkEnum::Value(v) => v.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thunk(pub Rc<RefCell<ThunkEnum>>);

impl Thunk {
    pub fn eval(&self) -> anyhow::Result<Value> {
        let mut t = self.0.borrow_mut();
        match &*t {
            ThunkEnum::Expr(e, env) => {
                let v = e.eval(env)?;
                *t = v.clone().into();
                Ok(v)
            }
            ThunkEnum::Value(v) => Ok(v.clone()),
        }
    }

    pub fn with_env(&self, env: Env) -> Self {
        match &*self.0.borrow() {
            ThunkEnum::Expr(e, _env) => {
                dbg!(_env, &env);
                ThunkEnum::Expr(e.clone(), env).into()
            }
            ThunkEnum::Value(_) => todo!(),
        }
    }
}

impl From<ThunkEnum> for Thunk {
    fn from(value: ThunkEnum) -> Self {
        Thunk(Rc::new(RefCell::new(value)))
    }
}

impl From<Expr> for Thunk {
    fn from(value: Expr) -> Self {
        ThunkEnum::Expr(value, Default::default()).into()
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(v) => v.fmt(f),
            Expr::UnaryOp(o, v) => write!(f, "({o} {v})"),
            Expr::BinaryOp(o, l, r) => match o {
                BinaryOp::Apply => write!(f, "({l} {r})"),
                o => write!(f, "({o} {l} {r})"),
            },
            Expr::If(c, t, e) => write!(f, "(if {c} {t} {e})"),
            Expr::Lambda(var, body) => write!(f, "(λ v{var} . {body})"),
            Expr::Var(var) => write!(f, "v{var}"),
        }
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
            Expr::BinaryOp(o, lhs, rhs) => {
                o.apply(&lhs.with_env(env.clone()), &rhs.with_env(env.clone()))
            }
            Expr::UnaryOp(o, e) => o.apply(&e.with_env(env.clone())),
            Expr::If(flag, t, f) => match flag.eval()? {
                Value::Boolean(true) => t.with_env(env.clone()).eval(),
                Value::Boolean(false) => f.with_env(env.clone()).eval(),
                v => bail!("Expected boolean: got {v:?}"),
            },
            &Expr::Lambda(var, ref body) => Ok(Value::Closure(var, body.with_env(env.clone()))),
            &Expr::Var(var) => env
                .iter()
                .rev()
                .find_map(|&(v, ref t)| (v == var).then(|| t.clone()))
                .with_context(|| format!("Undefined variable: {var}"))?
                .eval(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    String(String),
    Closure(i64, Thunk),
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Boolean(b) => b.fmt(f),
            Integer(i) => i.fmt(f),
            String(s) => write!(f, r#""{s}""#),
            Closure(var, body) => write!(f, "(λ v{var} . {body})"),
        }
    }
}
