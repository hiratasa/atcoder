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

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        xy: [(usize, usize); m]
    }

    if 2 * m + 2 < n {
        println!("Impossible");
        return;
    }

    let c = n - m;

    if c == 1 {
        println!("0");
        return;
    }

    let mut uf = xy.citer().fold(UnionFind::new(n), |mut uf, (x, y)| {
        uf.unite(x, y);
        uf
    });

    let table = (0..n).fold(FxHashMap::default(), |mut table, i| {
        table.entry(uf.root(i)).or_insert(vec![]).push(a[i]);
        table
    });

    let (s, mut v) = table
        .into_iter()
        .fold((0, vec![]), |(s, mut v), (_, mut vals)| {
            vals.sort();

            let x = vals[0];
            v.extend(vals[1..].citer());

            (s + x, v)
        });
    v.sort();

    let ans = s + v[..c - 2].citer().sum::<usize>();

    println!("{ans}");
}
