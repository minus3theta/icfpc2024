use std::collections::{BTreeSet, VecDeque};

use grid_graph::LambdamanCommand;
use union_find::UnionFind;


pub mod grid_graph;
pub mod scanner;
mod union_find;

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solver {
    board: Board,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    board: Board,
    height: usize,
    width: usize,
    pos: (usize, usize), // (y, x)
}

impl From<Board> for Input {
    fn from(board: Board) -> Self {
        let height = board.len();
        let width = board[0].len();
        let mut pos = None;
        for (y, line) in board.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'L' {
                    pos = Some((y, x));
                }
            }
        }
        Input {
            board,
            height,
            width,
            pos: pos.expect("L is not found"),
        }
    }
}

impl Solver {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        Solver { board }
    }
    pub fn solve(&self) -> Vec<LambdamanCommand> {
        let input = Input::from(self.board.clone());
        let mut graph = grid_graph::GridGraph::from(input.board.clone());

        // まずGrid Graphを的なサイズのグループに分けます
        eprintln!("Building group...");
        let mut que = VecDeque::default();
        let pos = graph.pos_to_idx[&input.pos];

        let mut visited = vec![false; graph.edges.len()];
        que.push_back((pos, pos));
        let mut ut = UnionFind::new(graph.edges.len());
        // TODO ここ結構雑なので、もう少し工夫できるかも
        let max_group_size = graph.edges.len() / 3;
        while let Some((pos, pre)) = que.pop_front() {
            if visited[pos] {
                continue;
            }
            visited[pos] = true;
            if !ut.is_same_set(pos, pre)
                && ut.union_size(pos) + ut.union_size(pre) <= max_group_size
            {
                ut.unite(pos, pre);
            }
            for (to, _) in graph.edges[pos].iter() {
                if visited[*to] {
                    continue;
                }
                que.push_back((*to, pos));
            }
        }

        eprintln!("Merging group...");
        let pre_group_size = ut.size();
        // 適当に最大サイズと最小サイズの差が半分以下になるまでグループをマージします
        loop {
            // 一番小さいグループと最大サイズのグループを見つけます
            let mut min_size = std::usize::MAX;
            let mut min_group = 0;
            let mut max_size = 0;
            for i in 0..graph.edges.len() {
                if ut.is_root(i) {
                    let size = ut.union_size(i);
                    if size < min_size {
                        min_size = size;
                        min_group = i;
                    }
                    if size > max_size {
                        max_size = size;
                    }
                }
            }
            if max_size < min_size * 2 {
                break;
            }

            // 隣接するグループで次に小さいグループを見つけます
            let mut min_neighbor_size = std::usize::MAX;
            let mut min_neighbor_group = 0;
            for i in 0..graph.edges.len() {
                let group_id = ut.root(i);
                if group_id == min_group {
                    continue;
                }
                let size = ut.union_size(group_id);
                if size < min_neighbor_size {
                    min_neighbor_size = size;
                    min_neighbor_group = group_id;
                }
            }
            ut.unite(min_group, min_neighbor_group);
        }
        eprintln!("Group size: {} -> {}", pre_group_size, ut.size());

        // ここから探索フェーズです。同じグループのノードを全部たどったらまだ未達のグループを探索します
        eprintln!("Searching...");
        let mut group_to_idx = vec![BTreeSet::default(); graph.edges.len()];
        let mut group_set = BTreeSet::default();
        for y in 0..input.height {
            for x in 0..input.width {
                if input.board[y][x] == '#' || input.board[y][x] == 'L' {
                    continue;
                }
                let idx = graph.pos_to_idx[&(y, x)];
                let group_id = ut.root(idx);
                group_set.insert(group_id);
                group_to_idx[group_id].insert(idx);
            }
        }

        let mut pos = graph.pos_to_idx[&input.pos];
        let mut result_cmds = vec![];
        loop {
            let group_id = ut.root(pos);
            group_to_idx[group_id].remove(&pos);
            if group_set.is_empty() {
                break;
            }

            let mut min_cmds = if group_to_idx[group_id].is_empty() {
                // 同じグループのノードを全部たどったらまだ未達のグループで一番近いノードに向かいます
                let mut min_cost = std::usize::MAX;
                let mut min_cmds = vec![];
                for &target_group in group_set.iter() {
                    for &target in group_to_idx[target_group].iter() {
                        let cmds = graph.bfs_path(pos, target);

                        let cost = cmds.len();
                        if cost < min_cost {
                            min_cost = cost;
                            min_cmds = cmds;
                        }
                    }
                }
                min_cmds
            } else {
                // まだ未達のグループがある場合はそのグループのノードに向かいます
                let mut min_cost = std::usize::MAX;
                let mut min_cmds = vec![];
                for &target in group_to_idx[group_id].iter() {
                    let cmds = graph.bfs_path(pos, target);
                    let cost = cmds.len();
                    if cost < min_cost {
                        min_cost = cost;
                        min_cmds = cmds;
                    }
                }
                min_cmds
            };
            // 移動します
            for &(node, _) in min_cmds.iter() {
                let group_id = ut.root(node);
                group_to_idx[group_id].remove(&node);
                if group_to_idx[group_id].is_empty() {
                    group_set.remove(&group_id);
                }
                pos = node;
            }
            result_cmds.append(&mut min_cmds);
        }
        eprintln!("Cmd size: {}", result_cmds.len());
        result_cmds.into_iter().map(|(_, cmd)| cmd).collect()
    }
}
#[allow(clippy::module_inception)]
pub mod macros {
    #[macro_export]
    #[allow(unused_macros)]
    macro_rules ! max {($ x : expr ) => ($ x ) ; ($ x : expr , $ ($ y : expr ) ,+ ) => {std :: cmp :: max ($ x , max ! ($ ($ y ) ,+ ) ) } }
    #[macro_export]
    #[allow(unused_macros)]
    macro_rules ! min {($ x : expr ) => ($ x ) ; ($ x : expr , $ ($ y : expr ) ,+ ) => {std :: cmp :: min ($ x , min ! ($ ($ y ) ,+ ) ) } }
    #[macro_export]
    #[allow(unused_macros)]
    macro_rules ! ep {() => {{use std :: io :: Write ; writeln ! (std :: io :: stderr () , "\x1b[34;1m{}\x1b[m:" , line ! () ) . unwrap () ; } } ; ($ e : expr , $ ($ es : expr ) ,+ ) => {{use std :: io :: Write ; write ! (std :: io :: stderr () , "\x1b[34;1m{}\x1b[m:" , line ! () ) . unwrap () ; write ! (std :: io :: stderr () , " \x1b[92;1m{}\x1b[m = {:?}" , stringify ! ($ e ) , $ e ) . unwrap () ; $ (write ! (std :: io :: stderr () , ", \x1b[92;1m{}\x1b[m = {:?}" , stringify ! ($ es ) , $ es ) . unwrap () ; ) + writeln ! (std :: io :: stderr () ) . unwrap () ; } } ; ($ e : expr ) => {{use std :: io :: Write ; let result = $ e ; writeln ! (std :: io :: stderr () , "\x1b[34;1m{}\x1b[m: \x1b[92;1m{}\x1b[m = {:?}" , line ! () , stringify ! ($ e ) , result ) . unwrap () ; } } ; }
    #[macro_export]
    #[allow(unused_macros)]
    macro_rules ! dep {() => {if cfg ! (debug_assertions ) {{use std :: io :: Write ; write ! (std :: io :: stderr () , "\x1b[31;1m{}\x1b[m " , "[DEBUG]" ) . unwrap () ; } ep ! () ; } } ; ($ e : expr , $ ($ es : expr ) ,+ ) => {if cfg ! (debug_assertions ) {{use std :: io :: Write ; write ! (std :: io :: stderr () , "\x1b[31;1m{}\x1b[m " , "[DEBUG]" ) . unwrap () ; } ep ! ($ e , $ ($ es ) ,+ ) ; } } ; ($ e : expr ) => {if cfg ! (debug_assertions ) {{use std :: io :: Write ; write ! (std :: io :: stderr () , "\x1b[31;1m{}\x1b[m " , "[DEBUG]" ) . unwrap () ; } ep ! ($ e ) ; } } ; }
}
