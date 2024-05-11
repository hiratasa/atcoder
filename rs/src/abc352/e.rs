use std::iter::once;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
    }

    let cliques = (0..m)
        .map(|_| {
            input! {
                k: usize, c: usize,
                a: [Usize1; k],
            };

            (c, a)
        })
        .collect::<Vec<_>>();

    let ans = cliques
        .into_iter()
        .sorted_by_key(|(c, _)| *c)
        .flat_map(|(c, a)| a.into_iter().tuple_windows().map(move |(x, y)| (c, x, y)))
        .chain(once((0, 0, 0)))
        .scan(UnionFind::new(n), |uf, (c, x, y)| {
            if c == 0 {
                if uf.size(0) != n {
                    Some(1 << 60)
                } else {
                    Some(0)
                }
            } else if uf.same(x, y) {
                Some(0)
            } else {
                uf.unite(x, y);
                Some(c)
            }
        })
        .sum::<usize>();

    if ans >= 1 << 60 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
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
