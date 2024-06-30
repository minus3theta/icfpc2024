use std::collections::BTreeMap;

use crate::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdamanCommand {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GridGraph {
    // idx -> (to, command)
    pub edges: Vec<Vec<(usize, LambdamanCommand)>>,
    // idx -> (y, x)
    pub idx_to_pos: BTreeMap<usize, (usize, usize)>,
    // (y, x) -> idx
    pub pos_to_idx: BTreeMap<(usize, usize), usize>,
}

impl From<Board> for GridGraph {
    fn from(board: Board) -> Self {
        let height = board.len();
        let width = board[0].len();
        let mut pos_to_idx = BTreeMap::default();
        let mut idx_to_pos = BTreeMap::default();
        for (y, line) in board.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != '#' {
                    let idx = pos_to_idx.len();
                    pos_to_idx.insert((y, x), idx);
                    idx_to_pos.insert(idx, (y, x));
                }
            }
        }
        let mut edges = vec![vec![]; pos_to_idx.len()];
        for y in 0..height {
            for x in 0..width {
                if board[y][x] == '#' {
                    continue;
                }
                let idx = pos_to_idx[&(y, x)];
                // Up
                if y > 0 && board[y - 1][x] != '#' {
                    let to = pos_to_idx[&(y - 1, x)];
                    edges[idx].push((to, LambdamanCommand::Up));
                }

                // Down
                if y + 1 < height && board[y + 1][x] != '#' {
                    let to = pos_to_idx[&(y + 1, x)];
                    edges[idx].push((to, LambdamanCommand::Down));
                }
                // Right
                if x + 1 < width && board[y][x + 1] != '#' {
                    let to = pos_to_idx[&(y, x + 1)];
                    edges[idx].push((to, LambdamanCommand::Right));
                }
                // Left
                if x > 0 && board[y][x - 1] != '#' {
                    let to = pos_to_idx[&(y, x - 1)];
                    edges[idx].push((to, LambdamanCommand::Left));
                }
            }
        }

        GridGraph {
            edges,
            idx_to_pos,
            pos_to_idx,
        }
    }
}

impl GridGraph {
    // Returns the bfs path from src to target.
    pub fn bfs_path(&mut self, src: usize, target: usize) -> Vec<(usize, LambdamanCommand)> {
        let mut que = std::collections::VecDeque::new();
        let mut prev = vec![None; self.edges.len()];
        que.push_back(src);
        while let Some(idx) = que.pop_front() {
            if idx == target {
                break;
            }
            for &(to, cmd) in &self.edges[idx] {
                if to == src {
                    continue;
                }
                if prev[to].is_none() {
                    prev[to] = Some((cmd, idx));
                    que.push_back(to);
                }
            }
        }

        let mut result = vec![];
        let mut idx = target;
        while let Some((cmd, pre)) = prev[idx] {
            result.push((idx, cmd));
            idx = pre;
        }
        result.reverse();
        result
    }
}
