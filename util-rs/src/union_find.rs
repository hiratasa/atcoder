use cargo_snippet::snippet;

#[snippet("union_find")]
struct UnionFind {
    g: Vec<usize>,
}

#[snippet("union_find")]
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            g: (0..n).collect(),
        }
    }

    fn root(&mut self, v: usize) -> usize {
        if self.g[v] != v {
            self.g[v] = self.root(self.g[v]);
        }

        self.g[v]
    }

    fn unite(&mut self, v: usize, u: usize) {
        let rv = self.root(v);
        let ru = self.root(u);
        self.g[rv] = ru;
    }

    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
}
