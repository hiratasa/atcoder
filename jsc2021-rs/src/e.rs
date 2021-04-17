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
    rank: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            g: (0..n).collect(),
            rank: vec![0; n],
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

        if rv == ru {
        } else if self.rank[rv] < self.rank[ru] {
            self.g[rv] = ru;
        } else if self.rank[ru] < self.rank[rv] {
            self.g[ru] = rv;
        } else {
            self.g[ru] = rv;
            self.rank[rv] += 1;
        }
    }

    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
}

fn merge(l: usize, r: usize, k: usize, uf: &mut UnionFind) {
    if k == 0 {
        return;
    }

    let len = r - l;
    for i in 0..len / 2 {
        uf.unite(l + i, r - i - 1);
    }

    merge(l, l + len / 2, k - 1, uf);
    merge(r - len / 2, r, k - 1, uf);
}

fn main() {
    let k: usize = read();
    let s = read_str();

    if k > 0 && s.len() < 2usize.saturating_pow(k as u32 - 1) {
        println!("impossible");
        return;
    }

    let d = s.len() >> k;
    if d == 1 {
        println!("impossible");
        return;
    }

    let mut uf = UnionFind::new(s.len());
    merge(0, s.len(), k, &mut uf);

    let a = s
        .citer()
        .enumerate()
        .fold(vec![BTreeMap::new(); s.len()], |mut a, (i, c)| {
            *a[uf.root(i)].entry(c).or_insert(0usize) += 1;
            a
        });
    let (all_same, x0, x1) = (0..d / 2)
        .map(|i| {
            let r0 = uf.root(i);
            let r1 = uf.root(d - 1 - i);

            assert!(r0 != r1);

            let s0 = a[r0].iter().map(|t| *t.1).sum::<usize>();
            let s1 = a[r1].iter().map(|t| *t.1).sum::<usize>();

            let t0 = a[r0]
                .iter()
                .map(|t| (*t.1, *t.0))
                .chain(once((0, '#')))
                .sorted_by_key(|&t| Reverse(t))
                .collect::<Vec<_>>();
            let t1 = a[r1]
                .iter()
                .map(|t| (*t.1, *t.0))
                .chain(once((0, '#')))
                .sorted_by_key(|&t| Reverse(t))
                .collect::<Vec<_>>();

            (
                t0[0].1 == t1[0].1,
                s0 + s1,
                t0[0].0 + t1[0].0,
                max(t0[0].0 + t1[1].0, t0[1].0 + t1[0].0),
            )
        })
        .fold(
            (true, 0usize, usize::MAX),
            |(all_same, x0, x1), (same, s, m0, m1)| {
                (
                    all_same && same,
                    x0 + (s - m0),
                    min(x1.saturating_add(s - m0), x0 + (s - m1)),
                )
            },
        );
    let ans0 = if d > 0 && all_same { x1 } else { x0 };

    let calced = (0..d / 2).fold(vec![false; s.len()], |mut calced, i| {
        let r0 = uf.root(i);
        let r1 = uf.root(d - 1 - i);

        calced[r0] = true;
        calced[r1] = true;

        calced
    });

    let ans1 = a
        .iter()
        .enumerate()
        .filter(|(i, _)| !calced[*i])
        .map(|(_i, map)| {
            let s = map.iter().map(|t| *t.1).sum::<usize>();
            if s == 0 {
                return 0;
            }

            let m = map.iter().map(|t| *t.1).max().unwrap();
            s - m
        })
        .sum::<usize>();
    let ans = ans0 + ans1;
    println!("{}", ans);
}
