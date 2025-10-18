fn main() {
    input! {
        n: usize,
    };

    if n < 6 {
        println!("-1");
        return;
    }

    let k = n / 2;
    let m = 2 * k;
    let mut edges = vec![];
    for i in 1..k {
        edges.push((0, i));
    }
    for i in k..m - 1 {
        edges.push((i, m - 1));
    }
    for i in 1..k {
        for j in k..m - 1 {
            if i + j + 1 != m {
                edges.push((i, j));
            }
        }
    }
    if n % 2 > 0 {
        edges.push((0, n - 1));
        for i in k..m - 1 {
            edges.push((i, n - 1));
        }
    }

    println!("{}", edges.len());
    for (i, j) in edges {
        println!("{} {}", i + 1, j + 1);
    }
}

#[allow(dead_code)]
fn calc0(n: usize) -> impl Iterator<Item = (usize, Vec<(usize, usize)>)> {
    let edges = (0..n)
        .tuple_combinations::<(usize, usize)>()
        .collect::<Vec<_>>();
    let m = edges.len();

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

    let fix = 3;
    (0usize..1 << (m - fix))
        .map(move |s| (s << fix) + (1 << fix) - 1)
        .filter_map(move |s| {
            let edges = (0..m)
                .filter(|&i| s & (1 << i) != 0)
                .map(|i| edges[i])
                .collect::<Vec<_>>();
            let mut uf = edges.iter().fold(UnionFind::new(n), |mut uf, &(i, j)| {
                uf.unite(i, j);
                uf
            });

            if uf.size(0) != n {
                return None;
            }

            let mut dists = vec![vec![usize::MAX; n]; n];
            for i in 0..n {
                dists[i][i] = 0;
            }
            for &(i, j) in &edges {
                dists[i][j] = 1;
                dists[j][i] = 1;
            }

            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        dists[i][j] = min(dists[i][j], dists[i][k].saturating_add(dists[k][j]));
                    }
                }
            }

            if (0..n)
                .map(|i| {
                    (0..n)
                        .filter(|&j| dists[i][j] <= 2 && j != i)
                        .map(|j| j + 1)
                        .sum::<usize>()
                })
                .all_equal()
            {
                let x = (1..n)
                    .filter(|&j| dists[0][j] <= 2)
                    .map(|j| j + 1)
                    .sum::<usize>();

                Some((x, edges))
            } else {
                None
            }
        })
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
