mod lambdaman;
mod shape;

use shape::*;
use wasm_bindgen::prelude::*;

pub fn lambdaman_gen(seed: i32) -> String {
    let input = lambdaman::gen_input(seed as u64);
    // convert input to string:
    format!("{}", input)
}

#[wasm_bindgen]
pub fn gen(seed: i32, game_type: String) -> String {
    if game_type.starts_with("lambdaman") {
        return lambdaman_gen(seed);
    }
    if game_type == "spaceship" {
        return "".to_string();
    }
    "".to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

fn lambdaman_visualize(
    input: &lambdaman::Input,
    visited: &Vec<Vec<bool>>,
    pos: (usize, usize),
) -> String {
    let n = visited.len();
    let m = visited[0].len();
    let max_mn = n.max(m);
    let size = 800 / max_mn;
    let whole = size * max_mn;
    let mut svg = document(whole, whole);
    // draw each grid. represent number as color.
    for i in 0..n {
        for j in 0..m {
            svg = svg.add(rect_with_stroke(
                (j * size) as usize,
                (i * size) as usize,
                size as usize,
                size as usize,
                if input.wall[i][j] {
                    "black"
                } else if i == pos.0 && j == pos.1 {
                    "blue"
                } else if !visited[i][j] {
                    "gray"
                } else {
                    "white"
                },
            ));
        }
    }
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                svg = svg.add(circle(
                    j * size + size / 2,
                    i * size + size / 2,
                    size / 4,
                    "green",
                ));
            }
        }
    }
    svg = svg.add(circle(
        pos.1 * size + size / 2,
        pos.0 * size + size / 2,
        size / 8 * 3,
        "orange",
    ));
    svg.to_string()
}

pub fn lambdaman_vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = lambdaman::parse_input(&_input);
    let output = lambdaman::parse_output(&input, &_output).unwrap();
    let (score, err, (visited, pos)) =
        lambdaman::compute_score_details(&input, &output.moves, turn);
    Ret {
        score: score,
        err: err,
        svg: lambdaman_visualize(&input, &visited, pos).to_string(),
    }
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize, game_type: String) -> Ret {
    if game_type.starts_with("lambdaman") {
        return lambdaman_vis(_input, _output, turn);
    }
    if game_type == "spaceship" {
        return Ret {
            score: 0,
            err: "Not implemented".to_string(),
            svg: "".to_string(),
        };
    }
    Ret {
        score: 0,
        err: "Unknown game type".to_string(),
        svg: "".to_string(),
    }
}

fn lambdaman_get_max_turn(_input: String, _output: String) -> usize {
    let input = lambdaman::parse_input(&_input);
    lambdaman::parse_output(&input, &_output)
        .unwrap()
        .moves
        .len()
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String, game_type: String) -> usize {
    if game_type.starts_with("lambdaman") {
        return lambdaman_get_max_turn(_input, _output);
    }
    if game_type == "spaceship" {
        return 0;
    }
    0
}
