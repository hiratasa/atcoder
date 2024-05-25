use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        abc: [(Usize1, Usize1, usize); n - 1],
        uvw: [(Usize1, Usize1, usize); q],
    };

    let mut state = vec![(UnionFind::new(n), n); 11];

    for (a, b, c) in abc {
        for w in c..=10 {
            let (uf, k) = &mut state[w];

            if !uf.same(a, b) {
                uf.unite(a, b);
                *k -= 1;
            }
        }
    }

    for (a, b, c) in uvw {
        for w in c..=10 {
            let (uf, k) = &mut state[w];

            if !uf.same(a, b) {
                uf.unite(a, b);
                *k -= 1;
            }
        }

        let ans = (0..10)
            .map(|i| (state[i].1 - state[i + 1].1) * (i + 1))
            .sum::<usize>();

        println!("{ans}");
    }
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
#[derive(Debug, Clone)]
struct UnionFind {
    g: Vec<UnionFindNode>,
}
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
