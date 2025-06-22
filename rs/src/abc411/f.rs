fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
        q: usize,
        x: [Usize1; q],
    };

    let adjs = uv
        .iter()
        .copied()
        .fold(vec![FxHashSet::default(); n], |mut adjs, (u, v)| {
            adjs[u].insert(v);
            adjs[v].insert(u);
            adjs
        });

    x.into_iter()
        .scan(
            (UnionFind::new(n), adjs, m),
            |(uf, adjs, num_edges), i_edge| {
                let (u, v) = uv[i_edge];

                if !uf.same(u, v) {
                    *num_edges -= 1;

                    let ru = uf.root(u);
                    let rv = uf.root(v);

                    let (v0, v1) = if adjs[ru].len() < adjs[rv].len() {
                        uf.unite(rv, ru);
                        (ru, rv)
                    } else {
                        uf.unite(ru, rv);
                        (rv, ru)
                    };
                    let r = v1;

                    let a = take(&mut adjs[v0]);
                    let mut b = take(&mut adjs[v1]);
                    assert!(a.len() <= b.len());
                    for z in a {
                        if uf.root(z) != z {
                            continue;
                        }
                        if uf.same(z, r) {
                            continue;
                        }

                        if !b.insert(z) {
                            *num_edges -= 1;
                        }
                        adjs[z].insert(r);
                    }
                    adjs[r] = b;
                }

                Some(*num_edges)
            },
        )
        .for_each(|ans| println!("{ans}"));
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

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
        self.g[ru] = Child { parent: rv };
        self.g[rv] = Root {
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
