use cargo_snippet::snippet;

#[snippet("union_find")]
#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}

#[snippet("union_find")]
struct UnionFind {
    g: Vec<UnionFindNode>,
}

#[snippet("union_find")]
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;

        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
        }
    }

    fn root(&mut self, v: usize) -> usize {
        use UnionFindNode::*;

        let p = match self.g[v] {
            Root { size: _ } => return v,
            Child { parent: p } => p,
        };
        let r = self.root(p);
        self.g[v] = Child { parent: r };
        r
    }

    fn unite(&mut self, v: usize, u: usize) -> bool {
        use UnionFindNode::*;

        let rv = self.root(v);
        let ru = self.root(u);

        if rv == ru {
            return false;
        }

        let size_rv = self.size(rv);
        let size_ru = self.size(ru);
        let (rsmall, rlarge) = if size_rv < size_ru {
            (rv, ru)
        } else {
            (ru, rv)
        };

        self.g[rsmall] = Child { parent: rlarge };
        self.g[rlarge] = Root {
            size: size_rv + size_ru,
        };

        true
    }

    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }

    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;

        let rv = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _ } => unreachable!(),
        }
    }
}
