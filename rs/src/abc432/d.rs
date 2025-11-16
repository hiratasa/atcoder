fn main() {
    input! {
        n: usize, x: i64, y: i64,
        cab: [(char, i64, i64); n],
    };

    let rects = cab
        .into_iter()
        .fold(vec![(0, 0, x, y)], |rects, (c, a, b)| {
            if c == 'X' {
                rects
                    .into_iter()
                    .flat_map(|(x0, y0, x1, y1)| {
                        if x1 <= a {
                            [Some((x0, y0 - b, x1, y1 - b)), None]
                        } else if a <= x0 {
                            [Some((x0, y0 + b, x1, y1 + b)), None]
                        } else {
                            [Some((x0, y0 - b, a, y1 - b)), Some((a, y0 + b, x1, y1 + b))]
                        }
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            } else {
                rects
                    .into_iter()
                    .flat_map(|(x0, y0, x1, y1)| {
                        if y1 <= a {
                            [Some((x0 - b, y0, x1 - b, y1)), None]
                        } else if a <= y0 {
                            [Some((x0 + b, y0, x1 + b, y1)), None]
                        } else {
                            [Some((x0 - b, y0, x1 - b, a)), Some((x0 + b, a, x1 + b, y1))]
                        }
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            }
        });

    let mut uf = UnionFind::new(rects.len());
    let m = rects.len();
    for (i, j) in (0..m).tuple_combinations() {
        let (x0, y0, x1, y1) = rects[i];
        let (x2, y2, x3, y3) = rects[j];

        if (x1 == x2 || x0 == x3) && !(y1 <= y2 || y3 <= y0) {
            uf.unite(i, j);
        }
        if (y1 == y2 || y0 == y3) && !(x1 <= x2 || x3 <= x0) {
            uf.unite(i, j);
        }
    }

    let mut ss = vec![0; m];
    for i in 0..m {
        let (x0, y0, x1, y1) = rects[i];
        ss[uf.root(i)] += (y1 - y0) * (x1 - x0);
    }

    ss.sort();
    let i = ss.iter().position(|&s| s > 0).unwrap();
    println!("{}", m - i);
    println!("{}", ss[i..].iter().join(" "));
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
