#[allow(clippy::module_inception)]
pub struct UnionFind {
    parent: Vec<i32>,
    size: usize,
}
impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        let parent = vec![-1; size];
        UnionFind { parent, size }
    }
    /// Returns a (usize, usize) tuple that represents `0` is a new root and `1` is a merged root.
    /// Returns a `None` if `x` and `y` is already merged.        
    pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let x_root = self.root(x);
        let y_root = self.root(y);
        if x_root == y_root {
            return None;
        }
        let size1 = self.parent[x_root];
        let size2 = self.parent[y_root];
        let (new_root, merged_root) = if size1 <= size2 {
            self.parent[x_root] += size2;
            self.parent[y_root] = x_root as i32;
            (x_root, y_root)
        } else {
            self.parent[y_root] += size1;
            self.parent[x_root] = y_root as i32;
            (y_root, x_root)
        };
        self.size -= 1;
        Some((new_root, merged_root))
    }
    #[allow(dead_code)]
    pub fn is_root(&mut self, x: usize) -> bool {
        self.root(x) == x
    }
    #[allow(dead_code)]
    pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let parent = self.parent[x] as usize;
        let root = self.root(parent);
        self.parent[x] = root as i32;
        root
    }
    pub fn union_size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        let set_size = -self.parent[root];
        set_size as usize
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.size
    }
}
