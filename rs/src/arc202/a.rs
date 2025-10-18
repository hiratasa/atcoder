fn main() {
    input! {
        cases:[[usize]],
    };

    cases.into_iter().map(|a| solve(&a)).for_each(|ans| {
        println!("{ans}");
    });
}

fn solve(a: &[usize]) -> usize {
    let n = a.len();

    let mut uf = (0..n)
        .tuple_windows()
        .fold(UnionFind::new(n), |mut uf, (i, j)| {
            if a[i] == a[j] {
                uf.unite(i, j);
            }

            uf
        });

    let mut blocks: FxHashMap<usize, (usize, usize, usize, usize)> = FxHashMap::default();
    let mut q = BinaryHeap::new();
    for i in 0..n {
        let r = uf.root(i);
        if let Some(block) = blocks.get_mut(&r) {
            block.3 = i;
        } else {
            blocks.insert(r, (uf.size(r), a[r], i, i));
        }
        if r == i {
            q.push(Reverse((a[i], i)));
        }
    }

    let mut ans = 0;
    while let Some(Reverse((v, i))) = q.pop() {
        let (s, u, l, r) = blocks[&i];
        if u != v || uf.root(i) != i {
            continue;
        }

        // eprintln!("Processing block: v = {v}, i = {i}, s = {s}, u = {u}, l = {l}, r = {r}");

        let left = if l == 0 {
            None
        } else {
            Some(blocks[&uf.root(l - 1)].1)
        };
        let right = if r == n - 1 {
            None
        } else {
            Some(blocks[&uf.root(r + 1)].1)
        };

        let target = match (left, right) {
            (Some(l), Some(r)) => min(l, r),
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (None, None) => {
                let mut s = s;
                while s > 1 {
                    if s % 2 == 0 {
                        s /= 2;
                    } else {
                        s = (s + 1) / 2;
                        ans += 1;
                    }
                }
                return ans;
            }
        };

        let mut v = v;
        let mut s = s;
        while v < target {
            if s == 1 {
                ans += target - v;
                v = target;
            } else if s % 2 == 0 {
                s /= 2;
                v += 1;
            } else {
                s = (s + 1) / 2;
                ans += 1;
                v += 1;
            }
        }
        blocks.insert(i, (s, target, l, r));
        if left == Some(target) {
            let left_block = blocks[&uf.root(l - 1)];
            uf.unite(l - 1, i);
            let root = uf.root(i);
            blocks.insert(root, (left_block.0 + s, target, left_block.2, r));
            // eprintln!(
            //     "Inserted {root} ({}, {}, {}, {}) from {left_block:?}",
            //     left_block.0 + s,
            //     target,
            //     left_block.2,
            //     r
            // );
            q.push(Reverse((target, root)));
        }
        if right == Some(target) {
            let block = blocks[&uf.root(i)];
            let right_block = blocks[&uf.root(r + 1)];
            uf.unite(r + 1, i);
            let root = uf.root(i);
            blocks.insert(
                root,
                (block.0 + right_block.0, target, block.2, right_block.3),
            );
            // eprintln!(
            //     "Inserted {root} ({}, {}, {}, {}) from {block:?} + {right_block:?}",
            //     block.0 + right_block.0,
            //     target,
            //     block.2,
            //     right_block.3
            // );
            q.push(Reverse((target, root)));
        }
    }

    ans
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
