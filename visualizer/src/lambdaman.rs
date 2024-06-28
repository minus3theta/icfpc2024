// copy from tools implementation at the following page
// https://atcoder.jp/contests/masters-qual/tasks/masters_qual_a

#![allow(non_snake_case, dead_code)]

use std::{ops::RangeBounds, vec};

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

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub pos: (usize, usize),
    pub dot: usize,
    pub wall: Vec<Vec<bool>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.wall.len() {
            for y in 0..self.wall[x].len() {
                if x == self.pos.0 && y == self.pos.1 {
                    writeln!(f, "L")?;
                } else {
                    writeln!(f, "{}", if self.wall[y][x] { "#" } else { "." })?;
                }
            }
            writeln!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let l = f.trim_end().split('\n').collect::<Vec<_>>();
    let mut pos = (0, 0);
    let mut dot = 0;
    let mut wall = vec![vec![false; l[0].len()]; l.len()];
    for (i, &s) in l.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            match c {
                'L' => {
                    pos = (i, j);
                }
                '.' => {
                    dot += 1;
                }
                '#' => wall[i][j] = true,
                _ => (),
            }
        }
    }
    Input { pos, dot, wall }
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

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

pub struct Output {
    pub moves: Vec<usize>,
}

pub fn parse_output(_: &Input, f: &str) -> Result<Output, String> {
    let mut moves = vec![];
    for c in f.chars() {
        if let Some(dir) = DIRS.iter().position(|&d| d == c) {
            moves.push(dir);
        } else {
            return Err(format!("Invalid direction: {}", c));
        }
    }
    Ok(Output { moves })
}

const FIXED: [&'static str; 22] = [
    "",
    include_str!("../../data/lambdaman/lambdaman1.in"),
    include_str!("../../data/lambdaman/lambdaman2.in"),
    include_str!("../../data/lambdaman/lambdaman3.in"),
    include_str!("../../data/lambdaman/lambdaman4.in"),
    include_str!("../../data/lambdaman/lambdaman5.in"),
    "",
    include_str!("../../data/lambdaman/lambdaman7.in"),
    include_str!("../../data/lambdaman/lambdaman8.in"),
    "",
    "",
    include_str!("../../data/lambdaman/lambdaman11.in"),
    include_str!("../../data/lambdaman/lambdaman12.in"),
    include_str!("../../data/lambdaman/lambdaman13.in"),
    include_str!("../../data/lambdaman/lambdaman14.in"),
    include_str!("../../data/lambdaman/lambdaman15.in"),
    include_str!("../../data/lambdaman/lambdaman16.in"),
    include_str!("../../data/lambdaman/lambdaman17.in"),
    include_str!("../../data/lambdaman/lambdaman18.in"),
    include_str!("../../data/lambdaman/lambdaman19.in"),
    include_str!("../../data/lambdaman/lambdaman20.in"),
    include_str!("../../data/lambdaman/lambdaman21.in"),
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
    moves: &Vec<usize>,
    max_turn: usize,
) -> (i64, String, (Vec<Vec<bool>>, (usize, usize))) {
    let mut visited = input.wall.clone();
    visited[input.pos.0][input.pos.1] = true;
    let mut rest = input.dot;
    let mut pos = input.pos;
    for (turn, &dir) in moves.iter().take(max_turn).enumerate() {
        pos.0 += DIJ[dir].0;
        pos.1 += DIJ[dir].1;
        if pos.0 >= input.wall.len() || pos.1 >= input.wall[0].len() || input.wall[pos.0][pos.1] {
            return (
                0,
                format!("Invalid move: {} in turn {}", DIRS[dir], turn + 1),
                (visited, pos),
            );
        }
        if !visited[pos.0][pos.1] {
            visited[pos.0][pos.1] = true;
            rest -= 1;
        }
    }
    if moves.len() == max_turn && rest > 0 {
        return (
            0,
            format!("Unvisited squares remain: {}", rest),
            (visited, pos),
        );
    }
    (max_turn as i64, String::new(), (visited, pos))
}
// end -- copy from ./tools/src/lib.rs
