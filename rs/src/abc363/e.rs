fn main() {
    input! {
        h: usize, w: usize, y: usize,
        a: [[usize; w]; h],
    };

    println!(
        "{}",
        iproduct!(0..h, 0..w)
            .sorted_by_key(|&(i, j)| a[i][j])
            .group_by(|&(i, j)| a[i][j])
            .into_iter()
            .scan(UnionFind::new(h * w + 1), |uf, (x, it)| {
                it.for_each(|(i, j)| {
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .filter_map(|(di, dj)| {
                            Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                        })
                        .filter(|&(ni, nj)| ni < h && nj < w && a[ni][nj] <= a[i][j])
                        .for_each(|(ni, nj)| {
                            uf.unite(i * w + j, ni * w + nj);
                        });

                    if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                        uf.unite(i * w + j, h * w);
                    }
                });

                Some((x, uf.size(h * w)))
            })
            .chain(once((100001, h * w + 1)))
            .scan((0usize, 1usize), |(prev, q), (x, k)| {
                let r = (x - *prev, h * w + 1 - *q);
                *prev = x;
                *q = k;

                Some(r)
            })
            .flat_map(|(m, c)| repeat_n(c, m))
            .skip(1)
            .take(y)
            .join("\n")
    );
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
