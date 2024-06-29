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

impl std::fmt::Display for Thunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0.borrow() {
            ThunkEnum::Expr(e) => e.fmt(f),
            ThunkEnum::Value(v) => v.fmt(f),
        }
    }
}

impl Thunk {
    pub fn eval(&self) -> anyhow::Result<Value> {
        let mut t = self.0.borrow_mut();
        match &*t {
            ThunkEnum::Expr(e) => {
                let v = e.eval()?;
                *t = v.clone().into();
                Ok(v)
            }
            ThunkEnum::Value(v) => Ok(v.clone()),
        }
    }

    pub fn subst(&self, var: i64, target: Thunk) -> Self {
        match &*self.0.borrow() {
            ThunkEnum::Expr(e) => e.subst(var, target),
            ThunkEnum::Value(v) => v.clone().into(),
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

impl From<Value> for Thunk {
    fn from(value: Value) -> Self {
        Thunk(Rc::new(RefCell::new(ThunkEnum::Value(value))))
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

    pub fn eval(&self) -> anyhow::Result<Value> {
        match self {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::BinaryOp(o, lhs, rhs) => o.apply(lhs, rhs),
            Expr::UnaryOp(o, e) => o.apply(e),
            Expr::If(flag, t, f) => match flag.eval()? {
                Value::Boolean(true) => t.eval(),
                Value::Boolean(false) => f.eval(),
                v => bail!("Expected boolean: got {v:?}"),
            },
            &Expr::Lambda(var, ref body) => Ok(Value::Closure(var, body.clone())),
            Expr::Var(_) => unreachable!(),
        }
    }

    pub fn subst(&self, var: i64, target: Thunk) -> Thunk {
        match self {
            Expr::Literal(_) => self.clone().into(),
            Expr::UnaryOp(o, e) => Expr::UnaryOp(o.clone(), e.subst(var, target)).into(),
            Expr::BinaryOp(o, l, r) => Expr::BinaryOp(
                o.clone(),
                l.subst(var, target.clone()),
                r.subst(var, target),
            )
            .into(),
            Expr::If(c, t, e) => Expr::If(
                c.subst(var, target.clone()),
                t.subst(var, target.clone()),
                e.subst(var, target),
            )
            .into(),
            &Expr::Lambda(v, ref body) => {
                let y = fresh();
                Expr::Lambda(y, body.subst(v, Expr::Var(y).into()).subst(var, target)).into()
            }
            &Expr::Var(v) => {
                if v == var {
                    target
                } else {
                    Expr::Var(v).into()
                }
            }
        }
    }
}

thread_local! {
    static COUNTER: RefCell<i64> = const { RefCell::new(-1) };
}

fn fresh() -> i64 {
    COUNTER.with(|c| {
        let mut c = c.borrow_mut();
        let var = *c;
        *c = var - 1;
        var
    })
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
