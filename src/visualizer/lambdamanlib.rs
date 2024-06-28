mod atcoder;
mod shape;

use atcoder::*;
use itertools::Itertools;
use shape::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let input = gen_input(seed as u64);
    // convert input to string:
    format!(
        "{} {}\n{}\n{}\n{}",
        input.ty,
        input.n,
        input
            .vs
            .iter()
            .map(|v| v.iter().collect::<String>())
            .join("\n"),
        input
            .hs
            .iter()
            .map(|v| v.iter().collect::<String>())
            .join("\n"),
        input
            .a
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).join(" "))
            .join("\n")
    )
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

fn visualize(input: &Input, a: &Vec<Vec<i32>>, p1: (usize, usize), p2: (usize, usize)) -> String {
    let n = a.len();
    let size = 800 / n;
    let whole = size * n;
    let nf = n as f64;
    let mut svg = document(whole, whole);
    // draw each grid. represent number as color.
    for i in 0..n {
        for j in 0..n {
            svg = svg.add(rect_with_stroke(
                (j * size) as usize,
                (i * size) as usize,
                size as usize,
                size as usize,
                &color(a[i][j] as f64 / (nf * nf)),
            ));
            svg = svg.add(text(
                j * size + size / 2,
                i * size + size / 2,
                size / 2,
                &a[i][j].to_string(),
            ));
        }
    }
    for i in 0..n {
        for (j, c) in input.vs[i].iter().enumerate() {
            if *c == '1' {
                svg = svg.add(line(
                    (i + 1) * size,
                    j * size,
                    (i + 1) * size,
                    (j + 1) * size,
                    3,
                    "black",
                ));
            }
        }
    }
    for i in 0..n - 1 {
        for (j, c) in input.hs[i].iter().enumerate() {
            if *c == '1' {
                svg = svg.add(line(
                    j * size,
                    (i + 1) * size,
                    (j + 1) * size,
                    (i + 1) * size,
                    3,
                    "black",
                ));
            }
        }
    }
    svg = svg.add(circle(
        p1.1 * size + size / 2,
        p1.0 * size + size / 2,
        size / 4,
        "black",
    ));
    svg = svg.add(circle(
        p2.1 * size + size / 2,
        p2.0 * size + size / 2,
        size / 4,
        "black",
    ));
    svg.to_string()
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parse_input(&_input);
    let output = parse_output(&input, &_output).unwrap();
    let (score, err, (a, p1, p2)) = compute_score_details(&input, output.start, &output.out, turn);
    Ret {
        score: score,
        err: err,
        svg: visualize(&input, &a, p1, p2).to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = parse_input(&_input);
    parse_output(&input, &_output).unwrap().out.len() + 1
}
