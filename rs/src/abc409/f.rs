fn main() {
    input! {
        n: usize, q: usize,
        xy: [(i64, i64); n],
    };

    let edges = (0..n)
        .tuple_combinations()
        .map(|(i, j)| {
            (
                Reverse((xy[i].0 - xy[j].0).abs() + (xy[i].1 - xy[j].1).abs()),
                i,
                j,
            )
        })
        .sorted()
        .collect::<BinaryHeap<_>>();

    (0..q)
        .scan((xy, edges, UnionFind::new(n + q)), |(xy, edges, uf), _| {
            input! {
                ty: usize,
            };

            if ty == 1 {
                input! {
                    a: i64, b: i64,
                };

                let n = xy.len();
                (0..n)
                    .map(|i| (i, (xy[i].0 - a).abs() + (xy[i].1 - b).abs()))
                    .for_each(|(i, d)| {
                        edges.push((Reverse(d), i, n));
                    });

                xy.push((a, b));

                Some(None)
            } else if ty == 2 {
                let mut d0 = None;
                while let Some((Reverse(d), i, j)) = edges.pop() {
                    if !uf.same(i, j) {
                        uf.unite(i, j);
                        d0 = Some(d);
                        break;
                    }
                }

                if let Some(d) = d0 {
                    while let Some(&(Reverse(d1), i, j)) = edges.peek() {
                        if d1 == d {
                            edges.pop();
                            uf.unite(i, j);
                        } else {
                            break;
                        }
                    }
                    Some(Some(d.to_string()))
                } else {
                    Some(Some("-1".to_string()))
                }
            } else {
                input! {
                    u: Usize1, v: Usize1,
                };

                if uf.same(u, v) {
                    Some(Some("Yes".to_string()))
                } else {
                    Some(Some("No".to_string()))
                }
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        });
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
