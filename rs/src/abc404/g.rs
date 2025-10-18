fn main() {
    input! {
        n: usize, m: usize,
        lrs: [(Usize1, usize, usize); m],
    };

    let mut uf = UnionFind::new(n + 1);
    for &(l, r, s) in &lrs {
        if uf.unite(r, l, s).is_none() {
            println!("-1");
            return;
        }
    }

    let verts = (0..=n).filter(|&i| uf.root(i).0 == i).collect::<Vec<_>>();
    let t = (0..=n).map(|i| uf.root(i)).collect::<Vec<_>>();

    let k = verts.len();
    let edges = t
        .iter()
        .tuple_windows()
        .fold(vec![], |mut edges, (&(x, y), &(z, w))| {
            let x = verts.iter().position(|&v| v == x).unwrap();
            let z = verts.iter().position(|&v| v == z).unwrap();
            edges.push((z, x, (y as i64) - (w as i64) - 1));
            edges
        });

    let mut dists = vec![i64::MAX; k];
    dists[k - 1] = 0;
    for _ in 0..k {
        for &(from, to, w) in &edges {
            dists[to] = min(dists[to], dists[from].saturating_add(w));
        }
    }

    if edges
        .iter()
        .any(|&(from, to, w)| dists[from].saturating_add(w) < dists[to])
    {
        println!("-1");
        return;
    }

    let (x, y) = uf.root(0);
    let x = verts.iter().position(|&v| v == x).unwrap();
    println!("{}", -(dists[x] - y as i64));
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
    Child { parent: usize, w: usize },
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
    fn root(&mut self, v: usize) -> (usize, usize) {
        use UnionFindNode::*;
        let (p, w) = match self.g[v] {
            Root { size: _ } => return (v, 0),
            Child { parent: p, w } => (p, w),
        };
        let (r, rw) = self.root(p);
        self.g[v] = Child {
            parent: r,
            w: w + rw,
        };
        (r, w + rw)
    }
    fn unite(&mut self, v: usize, u: usize, w: usize) -> Option<bool> {
        use UnionFindNode::*;
        let (rv, wv) = self.root(v);
        let (ru, wu) = self.root(u);
        if rv == ru {
            if wv + w == wu {
                return Some(false);
            } else {
                return None;
            }
        }
        let size = self.size(rv) + self.size(ru);
        if wv + w > wu {
            self.g[ru] = Child {
                parent: rv,
                w: wv + w - wu,
            };
            self.g[rv] = Root { size };
        } else {
            self.g[rv] = Child {
                parent: ru,
                w: wu - (wv + w),
            };
            self.g[ru] = Root { size };
        }
        Some(true)
    }
    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let (rv, _) = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _, w: _ } => unreachable!(),
        }
    }
}
