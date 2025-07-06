fn main() {
    input! {
        h: usize, w: usize, k: usize,
        rc: [(Usize1, Usize1); k],
    };

    let mut blocks = rc.into_iter().fold(vec![vec![]; h], |mut blocks, (r, c)| {
        blocks[r].push(c);
        blocks
    });

    blocks.iter_mut().for_each(|row| row.sort());

    let mut next_idx = 0;
    let groups = (0..h)
        .map(|i| {
            once(usize::MAX)
                .chain(blocks[i].iter().copied())
                .chain(once(w))
                .tuple_windows()
                .map(|(j0, j1)| (j0.wrapping_add(1), j1))
                .filter(|&(j0, j1)| j0 < j1)
                .map(|(j0, j1)| {
                    let idx = next_idx;
                    next_idx += 1;
                    (idx, j0, j1)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let l = next_idx;

    let mut uf = UnionFind::new(l);

    for (i0, i1) in (0..h).tuple_windows() {
        groups[i0].iter().fold(0, |mut c, &(idx0, j0, j1)| {
            while c < groups[i1].len() && groups[i1][c].2 <= j0 {
                c += 1;
            }

            while c < groups[i1].len() && groups[i1][c].1 < j1 {
                uf.unite(idx0, groups[i1][c].0);

                if groups[i1][c].2 <= j1 {
                    c += 1;
                } else {
                    break;
                }
            }

            c
        });
    }

    if uf.same(0, l - 1) {
        println!("Yes");
    } else {
        println!("No");
    }
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
