mod lambdaman;
mod shape;
mod spaceship;

use shape::*;
use wasm_bindgen::prelude::*;

pub fn lambdaman_gen(seed: i32) -> String {
    let input = lambdaman::gen_input(seed as u64);
    // convert input to string:
    format!("{}", input)
}

pub fn spaceship_gen(seed: i32) -> String {
    let input = spaceship::gen_input(seed as u64);
    // convert input to string:
    format!("{}", input)
}

#[wasm_bindgen]
pub fn gen(seed: i32, game_type: String) -> String {
    if game_type.starts_with("lambdaman") {
        return lambdaman_gen(seed);
    }
    if game_type == "spaceship" {
        return spaceship_gen(seed);
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

fn spaceship_visualize(
    input: &spaceship::Input,
    visited: &std::collections::HashSet<(i32, i32)>,
    pos_history: &std::collections::VecDeque<(i32, i32)>,
) -> String {
    let mut min_x = input.min_x;
    let mut min_y = input.min_y;
    let mut max_x = input.max_x;
    let mut max_y = input.max_y;
    let width = input.max_x - input.min_x + 3;
    let height = input.max_y - input.min_y + 3;
    if width > height {
        min_y -= (width - height) / 2;
        max_y += (width - height + 1) / 2;
    } else {
        min_x -= (height - width) / 2;
        max_x += (height - width + 1) / 2;
    }
    let remains = input
        .target
        .difference(visited)
        .cloned()
        .collect::<std::collections::HashSet<(i32, i32)>>();
    let n = (max_x - min_x + 1) as usize;
    if n < 160 {
        let size = 800 / n;
        let whole = size * n;
        let mut svg = document(whole, whole);
        // draw each grid. represent number as color.
        for i in 0..n {
            for j in 0..n {
                svg = svg.add(rect_with_stroke(
                    (j * size) as usize,
                    (i * size) as usize,
                    size as usize,
                    size as usize,
                    "white",
                ));
            }
        }
        for &p in &remains {
            let x = (p.0 - min_x) as usize;
            let y = (n as i32 - 1 - (p.1 - min_y as i32)) as usize;
            svg = svg.add(rect_with_stroke(
                (x * size) as usize,
                (y * size) as usize,
                size as usize,
                size as usize,
                "gray",
            ));
        }
        let base = pos_history.capacity() - pos_history.len();
        for (idx, &p) in pos_history.iter().enumerate() {
            let x = (p.0 - min_x) as usize;
            let y = (n as i32 - 1 - (p.1 - min_y)) as usize;
            svg = svg.add(circle(
                x * size + size / 2,
                y * size + size / 2,
                size / 4,
                &color((idx + base) as f64 / pos_history.capacity() as f64),
            ));
        }
        svg.to_string()
    } else {
        let mut svg = document(800, 800);
        for &p in &remains {
            let ratio_x = (p.0 - min_x) as f64 / (max_x - min_x) as f64;
            let ratio_y = 1.0 - (p.1 - min_y) as f64 / (max_y - min_y) as f64;
            let x = (ratio_x * 780.0 + 10.0) as usize;
            let y = (ratio_y * 780.0 + 10.0) as usize;
            svg = svg.add(rect_with_stroke(x - 4, y - 4, 8, 8, "gray"));
        }
        let base = pos_history.capacity() - pos_history.len();
        for (idx, &p) in pos_history.iter().enumerate() {
            let ratio_x = (p.0 - min_x) as f64 / (max_x - min_x) as f64;
            let ratio_y = 1.0 - (p.1 - min_y) as f64 / (max_y - min_y) as f64;
            let x = (ratio_x * 780.0 + 10.0) as usize;
            let y = (ratio_y * 780.0 + 10.0) as usize;
            svg = svg.add(circle(
                x,
                y,
                8,
                &color((idx + base) as f64 / pos_history.capacity() as f64),
            ));
        }

        svg.to_string()
    }
}

pub fn spaceship_vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = spaceship::parse_input(&_input);
    let output = spaceship::parse_output(&input, &_output).unwrap();
    let (score, err, (visited, pos_history)) =
        spaceship::compute_score_details(&input, &output.moves, turn);
    Ret {
        score: score,
        err: err,
        svg: spaceship_visualize(&input, &visited, &pos_history).to_string(),
    }
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize, game_type: String) -> Ret {
    if game_type.starts_with("lambdaman") {
        return lambdaman_vis(_input, _output, turn);
    }
    if game_type == "spaceship" {
        return spaceship_vis(_input, _output, turn);
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

fn spaceship_get_max_turn(_input: String, _output: String) -> usize {
    let input = spaceship::parse_input(&_input);
    spaceship::parse_output(&input, &_output)
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
        return spaceship_get_max_turn(_input, _output);
    }
    0
}
