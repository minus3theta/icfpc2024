use std::collections::BTreeSet;

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
        eprintln!("Budiling grid graph...");
        let mut graph = grid_graph::GridGraph::from(input.board.clone());
        eprintln!("Grid graph size: {}", graph.edges.len());

        // まずGrid Graphを的なサイズのグループに分けます
        eprintln!("Building group...");
        let mut ut = UnionFind::new(graph.edges.len());
        eprintln!("Merging group...");
        let pre_group_size = ut.size();
        let mut rng = xorshift::Xorshift128::new(42);
        // 適当な閾値でグループをマージします
        while ut.size() > 10 {
            // 一番小さいグループと最大サイズのグループを見つけます
            let mut min_size = std::usize::MAX;
            let mut min_group_id = 0;
            let mut group_to_idx = vec![vec![]; graph.edges.len()];
            for i in 0..graph.edges.len() {
                let group_size = ut.union_size(i);
                let group_id = ut.root(i);
                if group_size < min_size || (group_size == min_size && rng.gen() % 2 == 0) {
                    min_size = group_size;
                    min_group_id = group_id;
                }

                group_to_idx[group_id].push(i);
            }

            // 隣接するグループで一番小さいグループを見つけます
            let mut min_neighbor_size = std::usize::MAX;
            let mut min_neighbor_group_id = 0;
            for node in group_to_idx[min_group_id].iter() {
                for &(neighbor, _) in graph.edges[*node].iter() {
                    let neighbor_group_id = ut.root(neighbor);
                    if neighbor_group_id == min_group_id {
                        continue;
                    }
                    let neighbor_group_size = ut.union_size(neighbor);
                    if neighbor_group_size < min_neighbor_size
                        || (neighbor_group_size == min_neighbor_size && rng.gen() % 2 == 0)
                    {
                        min_neighbor_group_id = neighbor_group_id;
                        min_neighbor_size = neighbor_group_size;
                    }
                }
            }
            ut.unite(min_group_id, min_neighbor_group_id);
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

        for counter in 0.. {
            if counter % 100 == 0 {
                eprintln!("Counter: {}", counter);
            }
            let group_id = ut.root(pos);
            group_to_idx[group_id].remove(&pos);
            if group_set.is_empty() {
                break;
            }

            let costs = graph.bfs(pos);
            let min_node = if group_to_idx[group_id].is_empty() {
                // 同じグループのノードを全部たどったらまだ未達のグループで一番近いノードに向かいます
                let mut min_cost = std::usize::MAX;
                let mut min_node = None;
                for &target_group in group_set.iter() {
                    for &target in group_to_idx[target_group].iter() {
                        if costs[target] < min_cost {
                            min_cost = costs[target];
                            min_node = Some(target);
                        }
                    }
                }
                min_node
            } else {
                // まだ未達のグループがある場合はそのグループのノードに向かいます
                let mut min_cost = std::usize::MAX;
                let mut min_node = None;
                for &target in group_to_idx[group_id].iter() {
                    if costs[target] < min_cost {
                        min_cost = costs[target];
                        min_node = Some(target);
                    }
                }
                min_node
            };
            let mut min_cmds = graph.bfs_path(pos, min_node.expect("min_node is None"));
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

#[allow(clippy::module_inception, clippy::many_single_char_names)]
/// The period is 2^128 - 1
pub mod xorshift {
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct Xorshift128 {
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    }
    impl Default for Xorshift128 {
        fn default() -> Self {
            Xorshift128 {
                x: 123456789,
                y: 362436069,
                z: 521288629,
                w: 88675123,
            }
        }
    }
    impl Xorshift128 {
        pub fn new(seed: u32) -> Xorshift128 {
            let mut xorshift = Xorshift128::default();
            xorshift.z ^= seed;
            xorshift
        }
        pub fn gen(&mut self) -> u32 {
            let t = self.x ^ (self.x << 11);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;
            self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
            self.w
        }
    }
}
