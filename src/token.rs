use std::{cmp::min, collections::HashSet, str::FromStr};

use anyhow::{bail, Context};
use itertools::Itertools;
use num_bigint::BigInt;

pub use binary_op::BinaryOp;
pub use unary_op::UnaryOp;

mod binary_op;
mod integers;
mod strings;
mod unary_op;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Boolean(bool),
    Integer(BigInt),
    String(String),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    If,
    Lambda(BigInt),
    Variable(BigInt),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Boolean(b) => b.fmt(f),
            Token::Integer(i) => i.fmt(f),
            Token::String(s) => s.fmt(f),
            Token::UnaryOp(op) => op.fmt(f),
            Token::BinaryOp(op) => op.fmt(f),
            Token::If => write!(f, "?"),
            Token::Lambda(i) => write!(f, "L{i}"),
            Token::Variable(i) => write!(f, "v{i}"),
        }
    }
}

pub fn decode_token_stream(s: &str) -> anyhow::Result<Vec<Token>> {
    s.split_ascii_whitespace().map(decode_token).collect()
}

pub fn decode_token(s: &str) -> anyhow::Result<Token> {
    let mut bytes = s.bytes();
    match bytes.next().context("Token is empty")? {
        b'T' => Ok(bytes
            .next()
            .is_none()
            .then_some(Token::Boolean(true))
            .context("Unexpected body after T")?),
        b'F' => Ok(bytes
            .next()
            .is_none()
            .then_some(Token::Boolean(false))
            .context("Unexpected body after F")?),
        b'I' => integers::decode(bytes).map(Token::Integer),
        b'U' => unary_op::decode(bytes).map(Token::UnaryOp),
        b'S' => strings::decode(bytes).map(Token::String),
        b'B' => binary_op::decode(bytes).map(Token::BinaryOp),
        b'?' => Ok(Token::If),
        b'L' => integers::decode(bytes).map(Token::Lambda),
        b'v' => integers::decode(bytes).map(Token::Variable),
        unk => bail!("Unknown token: {}", unk as char),
    }
}

pub fn encode(tokens: &[Token]) -> anyhow::Result<String> {
    Ok(tokens
        .iter()
        .map(encode_token)
        .collect::<Result<Vec<_>, _>>()?
        .join(" "))
}

fn encode_token(token: &Token) -> anyhow::Result<String> {
    match token {
        Token::Boolean(_) => todo!(),
        Token::Integer(v) => Ok(format!("I{}", integers::encode(v.clone())?)),
        Token::String(s) => Ok(format!("S{}", strings::encode(s)?)),
        Token::UnaryOp(op) => Ok(format!("U{}", unary_op::encode(op)?)),
        Token::BinaryOp(op) => Ok(format!("B{}", binary_op::encode(op)?)),
        Token::If => Ok("?".to_owned()),
        Token::Lambda(v) => Ok(format!("L{}", integers::encode(v.clone())?)),
        Token::Variable(v) => Ok(format!("v{}", integers::encode(v.clone())?)),
    }
}

pub fn encode_string(s: &str) -> anyhow::Result<Vec<Token>> {
    let mut current = vec![Token::String(s.to_owned())];
    let mut min_len = encode(&current[..]).unwrap().len();
    if s.split_whitespace()
        .last()
        .unwrap()
        .chars()
        .all(|c| ['D', 'L', 'R', 'U'].contains(&c))
    {
        // lambdaman 用のエンコード
        let len = s.split_whitespace().last().unwrap().len();
        let first_char = s.split_whitespace().last().unwrap().chars().next().unwrap();
        // D L R U のうち登場するもの（0 を終端とするため最初の文字を最後に持っていく）
        let mut order = s
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .collect::<HashSet<_>>()
            .iter()
            .sorted_by_key(|c| **c == first_char)
            .copied()
            .collect_vec();
        if order.len() == 1 {
            order.insert(0, ' ');
        }
        let mut cand = vec![
            Token::BinaryOp(BinaryOp::Concat),
            Token::String(s[..s.len() - len].to_owned()),
            Token::BinaryOp(BinaryOp::Apply),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Lambda(BigInt::from(1)),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Lambda(BigInt::from(2)),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Variable(BigInt::from(1)),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Variable(BigInt::from(2)),
            Token::Variable(BigInt::from(2)),
            Token::Lambda(BigInt::from(2)),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Variable(BigInt::from(1)),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Variable(BigInt::from(2)),
            Token::Variable(BigInt::from(2)),
            Token::Lambda(BigInt::from(3)),
            Token::Lambda(BigInt::from(4)),
            Token::If,
            Token::BinaryOp(BinaryOp::Equal),
            Token::Variable(BigInt::from(4)),
            Token::Integer(BigInt::from(0)),
            Token::String("".to_owned()),
            Token::BinaryOp(BinaryOp::Concat),
            Token::BinaryOp(BinaryOp::Apply),
            Token::Variable(BigInt::from(3)),
            Token::BinaryOp(BinaryOp::Div),
            Token::Variable(BigInt::from(4)),
            Token::Integer(BigInt::from(order.len())),
        ];
        cand.extend(order.iter().enumerate().flat_map(|(i, c)| {
            if i != order.len() - 1 {
                vec![
                    Token::If,
                    Token::BinaryOp(BinaryOp::Equal),
                    Token::BinaryOp(BinaryOp::Mod),
                    Token::Variable(BigInt::from(4)),
                    Token::Integer(BigInt::from(order.len())),
                    Token::Integer(BigInt::from(i)),
                    Token::String(c.to_string()),
                ]
            } else {
                vec![Token::String(c.to_string())]
            }
        }));
        cand.extend(vec![Token::Integer(
            s.split_whitespace()
                .last()
                .unwrap()
                .chars()
                .fold(BigInt::from(0), |acc, c| {
                    acc * order.len() + order.iter().position(|m| c == *m).unwrap()
                }),
        )]);
        let cand_len = encode(&cand[..]).unwrap().len();
        if cand_len < min_len {
            min_len = cand_len;
            current = cand;
        }
        let len_group = s
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .chunk_by(|c| *c)
            .into_iter()
            .map(|(_, chunk)| chunk.count())
            .collect::<HashSet<_>>();
        for len_current in len_group {
            let mut cand = vec![
                Token::BinaryOp(BinaryOp::Concat),
                Token::String(s[..s.len() - len].to_owned()),
                Token::BinaryOp(BinaryOp::Apply),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(2)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(2)),
                Token::Variable(BigInt::from(2)),
                Token::Lambda(BigInt::from(2)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(2)),
                Token::Variable(BigInt::from(2)),
                Token::Lambda(BigInt::from(3)),
                Token::Lambda(BigInt::from(4)),
                Token::If,
                Token::BinaryOp(BinaryOp::Equal),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(0)),
                Token::String("".to_owned()),
                Token::BinaryOp(BinaryOp::Concat),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(3)),
                Token::BinaryOp(BinaryOp::Div),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(order.len() * len_current)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(6)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(6)),
                Token::Variable(BigInt::from(6)),
                Token::Lambda(BigInt::from(6)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(6)),
                Token::Variable(BigInt::from(6)),
                Token::Lambda(BigInt::from(7)),
                Token::Lambda(BigInt::from(8)),
                Token::If,
                Token::BinaryOp(BinaryOp::Equal),
                Token::Variable(BigInt::from(8)),
                Token::Integer(BigInt::from(0)),
                Token::Variable(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Concat),
                Token::Variable(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(7)),
                Token::BinaryOp(BinaryOp::Sub),
                Token::Variable(BigInt::from(8)),
                Token::Integer(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Mod),
                Token::BinaryOp(BinaryOp::Div),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(order.len())),
                Token::Integer(BigInt::from(len_current)),
            ];
            cand.extend(order.iter().enumerate().flat_map(|(i, c)| {
                if i != order.len() - 1 {
                    vec![
                        Token::If,
                        Token::BinaryOp(BinaryOp::Equal),
                        Token::BinaryOp(BinaryOp::Mod),
                        Token::Variable(BigInt::from(4)),
                        Token::Integer(BigInt::from(order.len())),
                        Token::Integer(BigInt::from(i)),
                        Token::String(c.to_string()),
                    ]
                } else {
                    vec![Token::String(c.to_string())]
                }
            }));
            cand.extend(vec![Token::Integer(
                s.split_whitespace()
                    .last()
                    .unwrap()
                    .chars()
                    .chunk_by(|c| *c)
                    .into_iter()
                    .fold(BigInt::from(0), |acc, (c, g)| {
                        let mut res = acc;
                        let mut remain = g.count();
                        while remain > 0 {
                            res = res * order.len() * len_current
                                + (min(remain, len_current) - 1) * order.len()
                                + order.iter().position(|m| c == *m).unwrap();
                            remain -= min(remain, len_current);
                        }
                        res
                    }),
            )]);
            let cand_len = encode(&cand[..]).unwrap().len();
            if cand_len < min_len {
                min_len = cand_len;
                current = cand;
            }
        }
    }
    if s.split_whitespace()
        .last()
        .unwrap()
        .chars()
        .all(|c| c.is_numeric())
    {
        // spaceship 用のエンコード
        {
            let tokens = s
                .chars()
                .scan(false, |cum, c| {
                    let next = c.is_numeric() && (*cum || c != '0');
                    *cum = next;
                    Some((c, next))
                })
                .enumerate()
                .chunk_by(|(_, (_, b))| *b)
                .into_iter()
                .map(|(key, chunk)| {
                    let chunk_array = chunk.collect::<Vec<_>>();
                    // 先頭と末尾では "B. " が不要になるので 8 桁分ボーナス
                    let len = chunk_array.len()
                        + if chunk_array[0].0 == 0 { 8 } else { 0 }
                        + if chunk_array[chunk_array.len() - 1].0 == s.len() - 1 {
                            8
                        } else {
                            0
                        };
                    // "B. B. S{} U$ I{} S{}" とエンコードするので増えた 13 文字以上の改善が得られる 27 桁以上連続しない場合は数値にしない
                    (
                        chunk_array
                            .into_iter()
                            .map(|(_, (c, _))| c)
                            .collect::<String>(),
                        key && len >= 27,
                    )
                })
                .chunk_by(|(_, i)| *i)
                .into_iter()
                .map(|(key, chunk)| {
                    if key {
                        vec![
                            Token::BinaryOp(BinaryOp::Concat),
                            Token::UnaryOp(UnaryOp::ToString),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Lambda(BigInt::from(1)),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Lambda(BigInt::from(2)),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Variable(BigInt::from(1)),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Variable(BigInt::from(2)),
                            Token::Variable(BigInt::from(2)),
                            Token::Lambda(BigInt::from(2)),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Variable(BigInt::from(1)),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Variable(BigInt::from(2)),
                            Token::Variable(BigInt::from(2)),
                            Token::Lambda(BigInt::from(3)),
                            Token::Lambda(BigInt::from(4)),
                            Token::If,
                            Token::BinaryOp(BinaryOp::Equal),
                            Token::Variable(BigInt::from(4)),
                            Token::Integer(BigInt::from(0)),
                            Token::Integer(BigInt::from(0)),
                            Token::BinaryOp(BinaryOp::Add),
                            Token::BinaryOp(BinaryOp::Add),
                            Token::BinaryOp(BinaryOp::Mod),
                            Token::Variable(BigInt::from(4)),
                            Token::Integer(BigInt::from(10)),
                            Token::Integer(BigInt::from(52)),
                            Token::BinaryOp(BinaryOp::Mul),
                            Token::BinaryOp(BinaryOp::Apply),
                            Token::Variable(BigInt::from(3)),
                            Token::BinaryOp(BinaryOp::Div),
                            Token::Variable(BigInt::from(4)),
                            Token::Integer(BigInt::from(10)),
                            Token::Integer(BigInt::from(94)),
                            Token::Integer(
                                BigInt::from_str(chunk.map(|(s, _)| s).join("").as_str()).unwrap(),
                            ),
                        ]
                    } else {
                        vec![
                            Token::BinaryOp(BinaryOp::Concat),
                            Token::String(chunk.map(|(s, _)| s).join("")),
                        ]
                    }
                })
                .collect_vec();
            let len = tokens.len();
            let cand = tokens
                .into_iter()
                .enumerate()
                .flat_map(|(index, iter)| {
                    iter.into_iter().skip(if index != len - 1 { 0 } else { 1 })
                })
                .collect_vec();
            let cand_len = encode(&cand[..]).unwrap().len();
            if cand_len < min_len {
                min_len = cand_len;
                current = cand;
            }
        }
        {
            let len = s.split_whitespace().last().unwrap().len();
            let cand = vec![
                Token::BinaryOp(BinaryOp::Concat),
                Token::String(s[..s.len() - len].to_owned()),
                Token::BinaryOp(BinaryOp::Apply),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(2)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(2)),
                Token::Variable(BigInt::from(2)),
                Token::Lambda(BigInt::from(2)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(2)),
                Token::Variable(BigInt::from(2)),
                Token::Lambda(BigInt::from(3)),
                Token::Lambda(BigInt::from(4)),
                Token::If,
                Token::BinaryOp(BinaryOp::Equal),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(0)),
                Token::String("".to_owned()),
                Token::BinaryOp(BinaryOp::Concat),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(3)),
                Token::BinaryOp(BinaryOp::Div),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(10000)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Lambda(BigInt::from(6)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(6)),
                Token::Variable(BigInt::from(6)),
                Token::Lambda(BigInt::from(6)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(5)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(6)),
                Token::Variable(BigInt::from(6)),
                Token::Lambda(BigInt::from(7)),
                Token::Lambda(BigInt::from(8)),
                Token::If,
                Token::BinaryOp(BinaryOp::Equal),
                Token::Variable(BigInt::from(8)),
                Token::Integer(BigInt::from(0)),
                Token::Variable(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Concat),
                Token::Variable(BigInt::from(9)),
                Token::BinaryOp(BinaryOp::Apply),
                Token::Variable(BigInt::from(7)),
                Token::BinaryOp(BinaryOp::Sub),
                Token::Variable(BigInt::from(8)),
                Token::Integer(BigInt::from(1)),
                Token::BinaryOp(BinaryOp::Mod),
                Token::BinaryOp(BinaryOp::Div),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(10)),
                Token::Integer(BigInt::from(1000)),
                Token::UnaryOp(UnaryOp::ToString),
                Token::BinaryOp(BinaryOp::Add),
                Token::BinaryOp(BinaryOp::Mod),
                Token::Variable(BigInt::from(4)),
                Token::Integer(BigInt::from(10)),
                Token::Integer(BigInt::from(52)),
                Token::Integer(
                    s.split_whitespace()
                        .last()
                        .unwrap()
                        .chars()
                        .chunk_by(|c| *c)
                        .into_iter()
                        .fold(BigInt::from(0), |acc, (c, g)| {
                            let mut res = acc;
                            let mut remain = g.count();
                            while remain > 0 {
                                res = res * 10000
                                    + (min(remain, 1000) - 1) * 10
                                    + c.to_string().parse::<u32>().unwrap();
                                remain -= min(remain, 1000);
                            }
                            res
                        }),
                ),
            ];
            let cand_len = encode(&cand[..]).unwrap().len();
            if cand_len < min_len {
                current = cand;
            }
        }
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn decode_hello_world() -> anyhow::Result<()> {
        assert_eq!(
            decode_token("SB%,,/}Q/2,$_")?,
            Token::String("Hello World!".into())
        );
        Ok(())
    }

    #[test]
    fn decode_binary_op() -> anyhow::Result<()> {
        assert_eq!(decode_token("B+")?, Token::BinaryOp(BinaryOp::Add));
        assert_eq!(decode_token("B-")?, Token::BinaryOp(BinaryOp::Sub));
        Ok(())
    }

    #[test]
    fn decode_if() -> anyhow::Result<()> {
        assert_eq!(decode_token("?")?, Token::If);
        Ok(())
    }

    #[test]
    fn decode_lambda() -> anyhow::Result<()> {
        assert_eq!(decode_token("L#")?, Token::Lambda(BigInt::from(2)));
        Ok(())
    }

    #[test]
    fn decode_variable() -> anyhow::Result<()> {
        assert_eq!(decode_token("v\"")?, Token::Variable(BigInt::from(1)));
        Ok(())
    }

    #[test]
    fn decode_1337() -> anyhow::Result<()> {
        assert_eq!(decode_token("I/6")?, Token::Integer(BigInt::from(1337)));
        assert_eq!(decode_token("I$WD")?, Token::Integer(BigInt::from(31619)));
        Ok(())
    }
    #[test]
    fn encode_decode_bigint() -> anyhow::Result<()> {
        // 2**256
        let value = BigInt::from_str(
            "115792089237316195423570985008687907853269984665640564039457584007913129639936",
        )
        .expect("Failed to parse BigInt");
        let encoded = integers::encode(value.clone())?;
        assert_eq!(
            encoded.as_bytes(),
            b"\"<VHiCBw9\"s6x'adSCTArinu6dOl9m&SMkR*`/=)"
        );
        let decoded = integers::decode(encoded.bytes())?;
        assert_eq!(decoded, value);
        Ok(())
    }

    #[test]
    fn encode_decode_hello_world() -> anyhow::Result<()> {
        assert_eq!(
            encode(&[Token::String("Hello World!".into())])?,
            "SB%,,/}Q/2,$_".to_owned()
        );
        assert_eq!(
            decode_token("SB%,,/}Q/2,$_")?,
            Token::String("Hello World!".into())
        );
        Ok(())
    }

    #[test]
    fn encode_language_test() -> anyhow::Result<()> {
        let value = integers::encode(BigInt::from(38798476154511i64))?;
        let result = strings::decode(value.bytes())?;
        assert_eq!(result, "4w3s0m3");
        Ok(())
    }
}
