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
    // Return the shortest costs from src to all nodes
    pub fn bfs(&mut self, src: usize) -> Vec<usize> {
        let mut que = std::collections::VecDeque::new();
        que.push_back((src, 0));
        let mut visited = vec![false; self.edges.len()];
        let mut results = vec![std::usize::MAX; self.edges.len()];
        while let Some((node, cost)) = que.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            results[node] = cost;
            for &(to, _) in self.edges[node].iter() {
                if !visited[to] {
                    que.push_back((to, cost + 1));
                }
            }
        }
        results
    }

    // Return the shortest costs from src to all nodes
    pub fn bfs_path(&self, src: usize, target: usize) -> Vec<(usize, LambdamanCommand)> {
        let mut que = std::collections::VecDeque::new();
        que.push_back((src, None));
        let mut visited = vec![false; self.edges.len()];
        let mut prev = vec![None; self.edges.len()];
        while let Some((node, cmd)) = que.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            prev[node] = cmd;
            if node == target {
                break;
            }
            for &(to, cmd) in self.edges[node].iter() {
                if !visited[to] {
                    que.push_back((to, Some((node, cmd))));
                }
            }
        }
        let mut path = vec![];
        let mut node = target;
        while let Some((prev_node, cmd)) = prev[node] {
            path.push((node, cmd));
            node = prev_node;
        }
        path.reverse();
        path
    }
}
