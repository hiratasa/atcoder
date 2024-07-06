fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };

    let xy = xy
        .into_iter()
        .map(|(x, y)| (x + 1, y + 1))
        .collect::<Vec<_>>();

    const M: usize = 200000;

    let mut y_to_xs = xy.into_iter().fold(vec![vec![]; M + 3], |mut t, (x, y)| {
        t[y].push(x);
        t
    });
    for xs in &mut y_to_xs {
        xs.sort();
    }

    let intervals = y_to_xs
        .iter()
        .scan(0usize, |idx, xs| {
            Some(
                once(0)
                    .chain(xs.iter().copied().flat_map(|x| [x, x + 1]))
                    .chain(once(M + 3))
                    .tuples()
                    .filter(|&(x0, x1)| x0 < x1)
                    .map(|(x0, x1)| {
                        let i = *idx;
                        *idx += 1;

                        (i, x0, x1)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let k = intervals.last().unwrap().last().unwrap().0 + 1;

    let mut uf = UnionFind::new(k);

    intervals
        .iter()
        .flatten()
        .copied()
        .filter(|&(_, x0, x1)| x0 == 0 || x1 == M + 3)
        .skip(1)
        .for_each(|(i, _, _)| {
            uf.unite(0, i);
        });

    (0..=M + 2).tuple_windows().for_each(|(y0, y1)| {
        let mut j = 0;
        intervals[y0].iter().for_each(|&(idx, x0, x1)| {
            while j < intervals[y1].len() && intervals[y1][j].2 <= x1 {
                if intervals[y1][j].2 > x0 {
                    uf.unite(idx, intervals[y1][j].0);
                }
                j += 1;
            }

            if j < intervals[y1].len() && intervals[y1][j].1 < x1 {
                uf.unite(idx, intervals[y1][j].0);
            }
        });
    });

    let ans = intervals
        .iter()
        .flatten()
        .copied()
        .filter(|&(idx, _, _)| !uf.same(0, idx))
        .map(|(_, x0, x1)| x1 - x0)
        .sum::<usize>();

    println!("{ans}");
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
