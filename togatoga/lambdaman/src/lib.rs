use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use grid_graph::LambdamanCommand;
use nonnan::NonNan;
use union_find::UnionFind;

pub mod grid_graph;
pub mod scanner;
mod union_find;

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solver {
    board: Board,
}

pub struct Config {
    pub group_size: usize,
    pub verbose: bool,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeamState {
    pos: usize,
    pills: BTreeSet<usize>,
    group_to_nodes: Vec<BTreeSet<usize>>,
    group_set: BTreeSet<usize>,
    // scores
    beam_score: NonNan<f64>,
    // for next turn command
    next_cmds: Option<Vec<(usize, LambdamanCommand)>>,
}

impl PartialOrd for BeamState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.beam_score.partial_cmp(&other.beam_score)
    }
}
impl Ord for BeamState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.beam_score.cmp(&other.beam_score)
    }
}

impl Solver {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        Solver { board }
    }
    pub fn solve(&mut self, config: &Config) -> Vec<LambdamanCommand> {
        let input = Input::from(self.board.clone());
        if config.verbose {
            eprintln!("Budiling grid graph...");
        }
        let mut graph = grid_graph::GridGraph::from(input.board.clone());
        if config.verbose {
            eprintln!("Grid graph size: {}", graph.edges.len());
        }
        let mut ut = UnionFind::new(graph.edges.len());
        if config.verbose {
            eprintln!("Merging group...");
        }
        let pre_group_size = ut.size();
        // 適当な閾値でグループをマージします
        while ut.size() > config.group_size {
            // 一番小さいグループと最大サイズのグループを見つけます
            let mut min_size = std::usize::MAX;
            let mut min_group_id = 0;
            let mut group_to_idx = vec![vec![]; graph.edges.len()];
            for i in 0..graph.edges.len() {
                let group_size = ut.union_size(i);
                let group_id = ut.root(i);
                if group_size < min_size {
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
                    if neighbor_group_size < min_neighbor_size {
                        min_neighbor_size = neighbor_group_size;
                        min_neighbor_group_id = neighbor_group_id;
                    }
                }
            }
            ut.unite(min_group_id, min_neighbor_group_id);
        }

        if config.verbose {
            eprintln!("Group size: {} -> {}", pre_group_size, ut.size());
        }
        // ここから探索フェーズです。同じグループのノードを全部たどったらまだ未達のグループを探索します
        if config.verbose {
            eprintln!("Searching...");
        }
        let mut group_to_nodes = vec![BTreeSet::default(); graph.edges.len()];
        let mut group_set = BTreeSet::default();
        let mut pills = BTreeSet::default();
        for y in 0..input.height {
            for x in 0..input.width {
                if input.board[y][x] == '#' || input.board[y][x] == 'L' {
                    continue;
                }
                let idx = graph.pos_to_idx[&(y, x)];
                let group_id = ut.root(idx);
                group_to_nodes[group_id].insert(idx);
                group_set.insert(group_id);
                pills.insert(idx);
            }
        }

        let mut pos = graph.pos_to_idx[&input.pos];
        let mut cmds = vec![];

        assert!(!pills.contains(&pos));
        assert!(!group_to_nodes[ut.root(pos)].contains(&pos));

        for counter in 1.. {
            let interval: usize = max!(5, graph.edges.len() / 20);
            if counter % interval == 0 {
                if config.verbose {
                    eprintln!("Counter: {}", counter);
                }
                if config.verbose {
                    eprintln!("Shrink cache...");
                    eprintln!(
                        "Cache bfs size: {}, bfs_path: {}",
                        graph.bfs_cache.len(),
                        graph.bfs_path_cache.len()
                    );
                }
                graph.shrink_cache();
                if config.verbose {
                    eprintln!(
                        "Cache bfs size: {}, bfs_path: {}",
                        graph.bfs_cache.len(),
                        graph.bfs_path_cache.len()
                    );
                }
            }

            if pills.is_empty() {
                break;
            }

            let next_cmds = if group_to_nodes[ut.root(pos)].is_empty() {
                // 一番近いグループを探します
                let costs = graph.bfs(pos);
                let mut min_cost = std::usize::MAX;
                let mut min_target_node = 0;

                for &other_group_id in group_set.iter() {
                    assert!(other_group_id != ut.root(pos));
                    for group_node in group_to_nodes[other_group_id].iter() {
                        let cost = costs[*group_node];
                        if cost < min_cost {
                            min_cost = cost;
                            min_target_node = *group_node;
                        }
                    }
                }

                graph.bfs_path(pos, min_target_node)
            } else {
                // beam depth
                const BEAM_DEPTH: usize = 1;
                const BEAM_WIDTH: usize = 100;
                let mut heap = BinaryHeap::default();

                heap.push(Reverse(BeamState {
                    pos,
                    pills: pills.clone(),
                    group_to_nodes: group_to_nodes.clone(),
                    group_set: group_set.clone(),
                    beam_score: NonNan::new(0.0),
                    next_cmds: None,
                }));
                // eprintln!("{}", pills.len());
                let root_group_id = ut.root(pos);

                let mut best_cmds = None;
                let mut best_score = 0.0;
                let mut checked = BTreeSet::default();
                for _depth in 0..BEAM_DEPTH {
                    let mut next_heap = BinaryHeap::default();
                    let mut beam_counter = 0;
                    // 5%ずつ探索スコアがdecayします
                    // ep!(heap.len());
                    while let Some(game_state) = heap.pop().map(|s| s.0.clone()) {
                        // ep!(beam_counter, game_state.pos, game_state.beam_score);
                        if game_state.beam_score.0 >= best_score {
                            best_score = game_state.beam_score.0;
                            best_cmds = game_state.next_cmds.clone();
                        }
                        if !checked.insert((game_state.pos, game_state.pills.clone())) {
                            continue;
                        }

                        if game_state.group_to_nodes[root_group_id].is_empty() {
                            continue;
                        }

                        let nodes_sorted = {
                            let mut nodes = game_state.group_to_nodes[root_group_id]
                                .clone()
                                .into_iter()
                                .collect::<Vec<_>>();
                            let costs = graph.bfs(game_state.pos);
                            nodes.sort_by_key(|&node| costs[node]);
                            let min_cost = costs[nodes[0]];
                            nodes
                                .into_iter()
                                .filter(|&node| costs[node] == min_cost)
                                .collect::<Vec<_>>()
                        };

                        for &target in nodes_sorted.iter() {
                            let cmds = graph.bfs_path(game_state.pos, target);
                            let mut next_state = game_state.clone();
                            next_state.pos = target;
                            if next_state.next_cmds.is_none() {
                                next_state.next_cmds = Some(cmds.clone());
                            }
                            let mut eaten_sum = 0;
                            for &(path, _) in cmds.iter() {
                                // assert!(path != next_state.pos);
                                next_state.pills.remove(&path);
                                let group_id = ut.root(path);
                                let eaten = next_state.group_to_nodes[group_id].remove(&path);
                                if next_state.group_to_nodes[group_id].is_empty() {
                                    next_state.group_set.remove(&group_id);
                                }
                                if eaten {
                                    eaten_sum += 1;
                                }
                            }
                            // スコア計算
                            let mut next_beam_score = 0.0;
                            // next_beam_score += (cmds.len() - eaten_sum) as f64;
                            // next_beam_score *= 4.0;

                            // どれだけ近いか
                            let costs = graph.bfs(next_state.pos);
                            let mut topks = vec![];
                            for &node in next_state.group_to_nodes[root_group_id].iter() {
                                topks.push(costs[node]);
                                next_beam_score += 0.05 * costs[node] as f64;
                            }
                            topks.sort();
                            for &cost in topks.iter().take(300) {
                                next_beam_score += cost as f64;
                            }
                            let decay = 0.95f64.powi(cmds.len() as i32);
                            next_state.beam_score += NonNan::new(decay * next_beam_score);
                            if next_state.beam_score.0 >= best_score {
                                best_score = next_state.beam_score.0;
                                best_cmds = next_state.next_cmds.clone();
                            }

                            next_heap.push(Reverse(next_state));
                        }
                        beam_counter += 1;
                        if beam_counter > BEAM_WIDTH {
                            break;
                        }
                    }
                    // assert!(!next_heap.is_empty());
                    // heap = next_heap;
                }

                if let Some(cmds) = best_cmds {
                    cmds
                } else {
                    eprint!("No next cmds");
                    let best_state = heap.pop().expect("No next state").0;
                    best_state.next_cmds.expect("No next cmds")
                }
            };

            for &(node, _) in next_cmds.iter() {
                // eat an pill if exists
                pills.remove(&node);

                let group_id = ut.root(node);
                group_to_nodes[group_id].remove(&node);
                if group_to_nodes[group_id].is_empty() {
                    group_set.remove(&group_id);
                }
                pos = node;
            }
            cmds.extend(next_cmds.into_iter());
        }
        cmds.into_iter().map(|(_, cmd)| cmd).collect()
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

#[allow(clippy::module_inception)]
pub mod nonnan {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
    pub trait NonNanValue:
        PartialOrd
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Copy
        + Sized
    {
    }
    impl NonNanValue for f64 {}
    impl NonNanValue for f32 {}
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct NonNan<T: NonNanValue>(pub T);
    impl<T: NonNanValue> NonNan<T> {
        pub fn new(x: T) -> NonNan<T> {
            NonNan(x)
        }
    }
    impl<T: NonNanValue> From<T> for NonNan<T> {
        fn from(from: T) -> NonNan<T> {
            NonNan::new(from)
        }
    }
    impl<T: NonNanValue> PartialOrd for NonNan<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: NonNanValue> Eq for NonNan<T> {}
    impl<T: NonNanValue> Ord for NonNan<T> {
        fn cmp(&self, other: &NonNan<T>) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }
    impl<T: NonNanValue> Add<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn add(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 + rhs.0)
        }
    }
    impl<T: NonNanValue> Add<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn add(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 + rhs)
        }
    }
    impl<T: NonNanValue> AddAssign<NonNan<T>> for NonNan<T> {
        fn add_assign(&mut self, rhs: NonNan<T>) {
            *self = *self + rhs
        }
    }
    impl<T: NonNanValue> AddAssign<T> for NonNan<T> {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs
        }
    }
    impl<T: NonNanValue> Sub<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn sub(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 - rhs.0)
        }
    }
    impl<T: NonNanValue> Sub<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn sub(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 - rhs)
        }
    }
    impl<T: NonNanValue> SubAssign<NonNan<T>> for NonNan<T> {
        fn sub_assign(&mut self, rhs: NonNan<T>) {
            *self = *self - rhs;
        }
    }
    impl<T: NonNanValue> SubAssign<T> for NonNan<T> {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }
    impl<T: NonNanValue> Mul<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn mul(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::from(self.0 * rhs.0)
        }
    }
    impl<T: NonNanValue> Mul<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn mul(self, rhs: T) -> NonNan<T> {
            NonNan::from(self.0 * rhs)
        }
    }
    impl<T: NonNanValue> MulAssign<NonNan<T>> for NonNan<T> {
        fn mul_assign(&mut self, rhs: NonNan<T>) {
            *self = *self * rhs
        }
    }
    impl<T: NonNanValue> MulAssign<T> for NonNan<T> {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs
        }
    }
    impl<T: NonNanValue> Div<NonNan<T>> for NonNan<T> {
        type Output = NonNan<T>;
        fn div(self, rhs: NonNan<T>) -> NonNan<T> {
            NonNan::new(self.0 / rhs.0)
        }
    }
    impl<T: NonNanValue> Div<T> for NonNan<T> {
        type Output = NonNan<T>;
        fn div(self, rhs: T) -> NonNan<T> {
            NonNan::new(self.0 / rhs)
        }
    }
    impl<T: NonNanValue> DivAssign<NonNan<T>> for NonNan<T> {
        fn div_assign(&mut self, rhs: NonNan<T>) {
            *self = *self / rhs
        }
    }
    impl<T: NonNanValue> DivAssign<T> for NonNan<T> {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs
        }
    }
}
