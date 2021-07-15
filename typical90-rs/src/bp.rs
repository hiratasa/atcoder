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

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}

struct UnionFind {
    g: Vec<UnionFindNode>,
    c: Vec<(i64, i64)>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;
        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
            c: vec![(0, 1); n],
        }
    }

    fn root(&mut self, v: usize) -> (usize, i64, i64) {
        use UnionFindNode::*;
        let p = match self.g[v] {
            Root { size: _ } => return (v, self.c[v].0, self.c[v].1),
            Child { parent: p } => p,
        };
        let (c0, c1) = self.c[v];
        let (r, d0, d1) = self.root(p);

        // value[p] = d0 + value[r] * d1
        // value[v] = c0 + value[p] * c1 = c0 + d0 * c1 + value[r] * d1 * c1
        self.g[v] = Child { parent: r };
        self.c[v] = (c0 + d0 * c1, d1 * c1);

        (r, self.c[v].0, self.c[v].1)
    }

    fn unite(&mut self, v: usize, u: usize, s: i64) -> bool {
        use UnionFindNode::*;

        let (rv, c0, c1) = self.root(v);
        let (ru, d0, d1) = self.root(u);

        if rv == ru {
            return false;
        }

        let size_rv = self.size(rv);
        let size_ru = self.size(ru);

        let (rsmall, rlarge, c0small, c1small, c0large, c1large) = if size_rv < size_ru {
            (rv, ru, c0, c1, d0, d1)
        } else {
            (ru, rv, d0, d1, c0, c1)
        };

        // (c0small + value[rsmall] * c1small) + (c0large + value[rlarge] * c1large) = s
        // => value[rsmall] = (s - c0small - c0large) / c1small - value[rlarge] * c1large / c1small
        assert!(c1small.abs() == 1);
        self.g[rsmall] = Child { parent: rlarge };
        self.c[rsmall] = ((s - c0small - c0large) / c1small, -c1large / c1small);
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
        let (rv, _, _) = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _ } => unreachable!(),
        }
    }
}

fn main() {
    let n: usize = read();
    let q: usize = read();

    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        let (t, x, y, v) = read_tuple!(usize, usize, usize, i64);
        let x = x - 1;
        let y = y - 1;

        if t == 0 {
            uf.unite(x, y, v);
        } else {
            let (rx, c0, c1) = uf.root(x);
            let (ry, d0, d1) = uf.root(y);

            if rx != ry {
                println!("Ambiguous");
            } else {
                // v = c0 + value[r] * c1
                // ? = d0 + value[r] * d1
                let ans = d0 + (v - c0) / c1 * d1;
                println!("{}", ans);
            }
        }
    }
}
