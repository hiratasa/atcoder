fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let nexts = a
        .iter()
        .enumerate()
        .cycle()
        .take(2 * n)
        .scan(vec![0; n + 1], |prevs, (i, &j)| {
            let p = prevs[j];
            prevs[j] = i;

            Some((p, i))
        })
        .skip(n)
        .fold(vec![0; n], |mut nexts, (i, j)| {
            nexts[i] = j;
            nexts
        });

    let s = (0..n)
        .scan((0..n).collect::<Vec<_>>(), |idxs, i| {
            let mut s = vec![0; idxs.len()];
            let j = idxs.iter().position(|&j| j == nexts[i]).unwrap();
            s[j] = 1;
            idxs.remove(j);
            Some(s)
        })
        .flatten()
        .collect::<Vec<_>>();
    let t = repeat_n(1, n)
        .chain(repeat(0))
        .take(s.len())
        .collect::<Vec<_>>();

    println!("{}", s.len());
    println!("{}", s.iter().join(""));
    println!("{}", t.iter().join(""));

    // assert!(check(&a, &s, &t));
}

fn check(a: &[usize], s: &[usize], t: &[usize]) -> bool {
    let zeros_s = s.iter().positions(|&x| x == 0).collect::<Vec<_>>();
    let ones_s = s.iter().positions(|&x| x == 1).collect::<Vec<_>>();
    let zeros_t = t.iter().positions(|&x| x == 0).collect::<Vec<_>>();
    let ones_t = t.iter().positions(|&x| x == 1).collect::<Vec<_>>();

    assert!(zeros_s.len() == zeros_t.len());
    assert!(ones_s.len() == ones_t.len());

    let mut uf = UnionFind::new(s.len());
    izip!(zeros_s, zeros_t).for_each(|(i, j)| {
        uf.unite(i, j);
    });
    izip!(ones_s, ones_t).for_each(|(i, j)| {
        uf.unite(i, j);
    });

    let n = a.len();
    (0..n)
        .tuple_combinations()
        .all(|(i, j)| uf.same(i, j) == (a[i] == a[j]))
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
