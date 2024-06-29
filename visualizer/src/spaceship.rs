// copy from tools implementation at the following page
// https://atcoder.jp/contests/masters-qual/tasks/masters_qual_a

#![allow(non_snake_case, dead_code)]

use std::ops::RangeBounds;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub target: std::collections::HashSet<(i32, i32)>,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

pub fn parse_input(f: &str) -> Input {
    let mut target = std::collections::HashSet::new();
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for line in f.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                target.insert((x, y));
                min_x.setmin(x);
                max_x.setmax(x);
                min_y.setmin(y);
                max_y.setmax(y);
            }
        }
    }
    Input {
        target,
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    pub moves: Vec<u8>,
}

pub fn parse_output(_: &Input, f: &str) -> Result<Output, String> {
    let mut moves = Vec::new();

    for c in f.chars() {
        if let Some(digit) = c.to_digit(10) {
            if digit >= 1 && digit <= 9 {
                moves.push((digit - 1) as u8);
            } else {
                return Err(format!("Character '{}' is out of the range 1-9", c));
            }
        } else {
            return Err(format!("Character '{}' is not a valid digit", c));
        }
    }

    Ok(Output { moves })
}

const FIXED: [&'static str; 26] = [
    "",
    include_str!("../../data/spaceship/spaceship1.in"),
    include_str!("../../data/spaceship/spaceship2.in"),
    include_str!("../../data/spaceship/spaceship3.in"),
    include_str!("../../data/spaceship/spaceship4.in"),
    include_str!("../../data/spaceship/spaceship5.in"),
    include_str!("../../data/spaceship/spaceship6.in"),
    include_str!("../../data/spaceship/spaceship7.in"),
    include_str!("../../data/spaceship/spaceship8.in"),
    include_str!("../../data/spaceship/spaceship9.in"),
    include_str!("../../data/spaceship/spaceship10.in"),
    include_str!("../../data/spaceship/spaceship11.in"),
    include_str!("../../data/spaceship/spaceship12.in"),
    include_str!("../../data/spaceship/spaceship13.in"),
    include_str!("../../data/spaceship/spaceship14.in"),
    include_str!("../../data/spaceship/spaceship15.in"),
    include_str!("../../data/spaceship/spaceship16.in"),
    include_str!("../../data/spaceship/spaceship17.in"),
    include_str!("../../data/spaceship/spaceship18.in"),
    include_str!("../../data/spaceship/spaceship19.in"),
    include_str!("../../data/spaceship/spaceship20.in"),
    include_str!("../../data/spaceship/spaceship21.in"),
    include_str!("../../data/spaceship/spaceship22.in"),
    include_str!("../../data/spaceship/spaceship23.in"),
    include_str!("../../data/spaceship/spaceship24.in"),
    include_str!("../../data/spaceship/spaceship25.in"),
];

// change function name from `gen` to `gen_input`
pub fn gen_input(seed: u64) -> String {
    if seed as usize >= FIXED.len() {
        return "".to_string();
    }
    FIXED[seed as usize].to_owned()
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out.moves, out.moves.len());
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

// added argument `max_turn`
pub fn compute_score_details(
    input: &Input,
    moves: &Vec<u8>,
    max_turn: usize,
) -> (
    i64,
    String,
    (
        std::collections::HashSet<(i32, i32)>,
        std::collections::VecDeque<(i32, i32)>,
    ),
) {
    let mut pos = (0, 0);
    let mut vel = (0, 0);
    let mut pos_history = std::collections::VecDeque::with_capacity(10);
    let mut visited = std::collections::HashSet::new();
    pos_history.push_back(pos);
    for &dir in moves.iter().take(max_turn) {
        vel.0 += dir as i32 % 3 - 1;
        vel.1 += dir as i32 / 3 - 1;
        pos.0 += vel.0;
        pos.1 += vel.1;
        if input.target.contains(&pos) {
            visited.insert(pos);
        }
        if pos_history.len() == 10 {
            pos_history.pop_front();
        }
        pos_history.push_back(pos);
    }
    if moves.len() == max_turn && input.target.len() != visited.len() {
        return (
            0,
            format!(
                "Unvisited squares remain: {}",
                input.target.len() - visited.len()
            ),
            (visited, pos_history),
        );
    }
    (max_turn as i64, String::new(), (visited, pos_history))
}
// end -- copy from ./tools/src/lib.rs
