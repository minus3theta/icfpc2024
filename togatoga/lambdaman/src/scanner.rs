use std::collections::VecDeque;
pub struct Scanner<R> {
    stdin: R,
    buffer: VecDeque<String>,
}
impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(s: R) -> Scanner<R> {
        Scanner {
            stdin: s,
            buffer: VecDeque::new(),
        }
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let line = self.read_line();
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn usize1(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn tuple<T1: std::str::FromStr, T2: std::str::FromStr>(&mut self) -> (T1, T2) {
        (self.read::<T1>(), self.read::<T2>())
    }
    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        let _ = self.stdin.read_line(&mut line);
        line.trim_end().to_string()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn vchars(&mut self, n: usize) -> Vec<Vec<char>> {
        (0..n).map(|_| self.chars()).collect::<Vec<Vec<char>>>()
    }
    pub fn vvec<T: std::str::FromStr>(&mut self, n: usize, m: usize) -> Vec<Vec<T>> {
        (0..n).map(|_| self.vec::<T>(m)).collect::<Vec<Vec<T>>>()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
