fn main() {
    input! {
        n: usize,
        pab: [(usize, usize, usize); n],
        q: usize,
        xs: [usize; q],
    };

    const K: usize = 1000;
    let (vals, mut uf, que, offset) = pab.into_iter().fold(
        (
            vec![None; K + 1],
            UnionFind::new(q),
            xs.into_iter()
                .enumerate()
                .map(|(i, x)| (Reverse(x), i))
                .collect::<BinaryHeap<_>>(),
            0,
        ),
        |(mut vals, mut uf, mut que, offset), (p, a, b)| {
            while let Some(&(Reverse(x), idx)) = que.peek() {
                if x - offset <= K {
                    if let Some(idx2) = vals[x - offset] {
                        uf.unite(idx, idx2);
                    } else {
                        vals[x - offset] = Some(idx);
                    }
                    que.pop();
                } else {
                    break;
                }
            }

            let vals2 =
                vals.into_iter()
                    .enumerate()
                    .fold(vec![None; K + 1], |mut vals2, (i, idx_opt)| {
                        if let Some(idx) = idx_opt {
                            let j = if p >= i { i + a } else { i.saturating_sub(b) };

                            if let Some(idx2) = vals2[j] {
                                uf.unite(idx, idx2);
                            } else {
                                vals2[j] = Some(idx);
                            }
                        }

                        vals2
                    });

            let offset = offset + b;

            (vals2, uf, que, offset)
        },
    );

    let mut ans = vec![0; q];
    for i in 0..=K {
        if let Some(idx) = vals[i] {
            ans[uf.root(idx)] = i;
        }
    }
    for i in 0..q {
        ans[i] = ans[uf.root(i)];
    }
    for (Reverse(x), i) in que {
        ans[i] = x - offset;
    }

    for x in ans {
        println!("{x}");
    }
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
