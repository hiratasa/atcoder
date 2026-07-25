fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize, k: usize,
            abc: [(Usize1, Usize1, usize); m],
        };

        let edges0 = abc
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, (_, _, c))| c == 0)
            .map(|(i, (a, b, _))| (a, b, i))
            .collect::<Vec<_>>();
        let edges1 = abc
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, (_, _, c))| c == 1)
            .map(|(i, (a, b, _))| (a, b, i))
            .collect::<Vec<_>>();

        let uf0 = edges0
            .iter()
            .copied()
            .fold(UnionFind::new(n), |mut uf, (a, b, _)| {
                uf.unite(a, b);
                uf
            });

        let r = edges1
            .iter()
            .copied()
            .scan(uf0, |uf, (a, b, i)| {
                if uf.same(a, b) {
                    Some(None)
                } else {
                    uf.unite(a, b);
                    Some(Some((a, b, i)))
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        if r.len() > k {
            println!("-1");
            continue;
        }

        let uf = r
            .iter()
            .copied()
            .fold(UnionFind::new(n), |mut uf, (a, b, _)| {
                uf.unite(a, b);
                uf
            });
        let (uf, selected) = edges1.iter().copied().fold(
            (uf, r.iter().copied().map(|(_, _, i)| i).collect::<Vec<_>>()),
            |(mut uf, mut selected), (a, b, i)| {
                if selected.len() == k {
                    // NOP
                } else if uf.same(a, b) {
                    // NOP
                } else {
                    uf.unite(a, b);
                    selected.push(i);
                }
                (uf, selected)
            },
        );
        if selected.len() != k {
            println!("-1");
            continue;
        }
        let (uf, selected) =
            edges0
                .iter()
                .copied()
                .fold((uf, selected), |(mut uf, mut selected), (a, b, i)| {
                    if uf.same(a, b) {
                        // NOP
                    } else {
                        uf.unite(a, b);
                        selected.push(i);
                    }
                    (uf, selected)
                });
        if selected.len() != n - 1 {
            println!("-1");
            continue;
        }

        println!("{}", selected.iter().copied().map(|i| i + 1).join(" "));
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng};
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
