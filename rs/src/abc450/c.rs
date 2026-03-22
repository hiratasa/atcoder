fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let mut uf = UnionFind::new(h * w);
    let to_idx = |i: usize, j: usize| i * w + j;
    for i in 0..h {
        for j in 0..w - 1 {
            if s[i][j] == '.' && s[i][j + 1] == '.' {
                uf.unite(to_idx(i, j), to_idx(i, j + 1));
            }
        }
    }
    for i in 0..h - 1 {
        for j in 0..w {
            if s[i][j] == '.' && s[i + 1][j] == '.' {
                uf.unite(to_idx(i, j), to_idx(i + 1, j));
            }
        }
    }

    let ngs = chain(
        (0..h).flat_map(|i| [(i, 0), (i, w - 1)]),
        (0..w).flat_map(|j| [(0, j), (h - 1, j)]),
    )
    .filter(|&(i, j)| s[i][j] == '.')
    .map(|(i, j)| uf.root(to_idx(i, j)))
    .unique()
    .count();

    let nums = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == '.')
        .filter(|&(i, j)| uf.root(to_idx(i, j)) == to_idx(i, j))
        .count();

    let ans = nums - ngs;

    println!("{ans}");
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
