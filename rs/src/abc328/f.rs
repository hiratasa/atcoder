#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{Readable, Source},
};

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize, diff: i64 },
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
    fn root(&mut self, v: usize) -> (usize, i64) {
        use UnionFindNode::*;
        let (p, d) = match self.g[v] {
            Root { size: _ } => return (v, 0),
            Child { parent: p, diff: d } => (p, d),
        };
        let (r, d2) = self.root(p);
        self.g[v] = Child {
            parent: r,
            diff: d + d2,
        };
        (r, d + d2)
    }
    fn unite(&mut self, v: usize, u: usize, diff: i64) -> bool {
        use UnionFindNode::*;
        let (rv, dv) = self.root(v);
        let (ru, du) = self.root(u);
        if rv == ru {
            return false;
        }
        let size_rv = self.size(rv);
        let size_ru = self.size(ru);
        let (rsmall, rlarge, diff) = if size_rv < size_ru {
            (rv, ru, -dv + diff + du)
        } else {
            (ru, rv, -du - diff + dv)
        };
        self.g[rsmall] = Child {
            parent: rlarge,
            diff,
        };
        self.g[rlarge] = Root {
            size: size_rv + size_ru,
        };
        true
    }
    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v).0 == self.root(u).0
    }
    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let (rv, _) = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _, diff: _ } => unreachable!(),
        }
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        abd: [(Usize1, Usize1, i64); q],
    };

    let ans = abd
        .citer()
        .enumerate()
        .scan(UnionFind::new(n), |uf, (i, (a, b, d))| {
            if uf.same(a, b) {
                let diff = uf.root(a).1 - uf.root(b).1;

                if d != diff {
                    Some(None)
                } else {
                    Some(Some(i))
                }
            } else {
                uf.unite(a, b, d);

                Some(Some(i))
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("{}", ans.citer().map(|i| i + 1).join(" "));
}
