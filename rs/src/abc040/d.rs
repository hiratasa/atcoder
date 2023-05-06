#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

struct UnionFind {
    g: Vec<usize>,
    size: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            g: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn root(&mut self, v: usize) -> usize {
        if self.g[v] != v {
            self.g[v] = self.root(self.g[v]);
        }
        self.g[v]
    }
    fn unite(&mut self, v: usize, u: usize) {
        let rv = self.root(v);
        let ru = self.root(u);
        self.g[rv] = ru;
        if rv != ru {
            self.size[ru] += self.size[rv];
        }
    }

    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }

    fn size(&mut self, v: usize) -> usize {
        let rv = self.root(v);
        self.size[rv]
    }
}

enum Item {
    Road(usize, usize, usize),
    Query(usize, usize, usize),
}

use Item::*;

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let aby = read_vec(m, || read_tuple!(usize, usize, usize));

    let q: usize = read();
    let vw = read_vec(q, || read_tuple!(usize, usize));

    chain(
        aby.citer().map(|(a, b, y)| Road(a - 1, b - 1, y)),
        vw.citer().enumerate().map(|(i, (v, w))| Query(i, v - 1, w)),
    )
    .sorted_by_key(|item| match item {
        &Road(_, _, y) => (Reverse(y), 1),
        &Query(_, _, y) => (Reverse(y), 0),
    })
    .scan(UnionFind::new(n), |uf, item| match item {
        Road(a, b, _y) => {
            uf.unite(a, b);
            Some(None)
        }
        Query(i, v, _w) => Some(Some((i, uf.size(v)))),
    })
    .flatten()
    .sorted()
    .for_each(|(_, ans)| println!("{}", ans));
}
