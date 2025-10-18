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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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

#[derive(Clone)]
struct UnionFind {
    g: Vec<u32>,
    size: Vec<u32>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            g: (0..n as u32).collect(),
            size: vec![1; n],
        }
    }
    fn root(&mut self, v: u32) -> u32 {
        if self.g[v as usize] != v {
            self.g[v as usize] = self.root(self.g[v as usize]);
        }
        self.g[v as usize]
    }
    fn unite(&mut self, v: u32, u: u32) {
        let rv = self.root(v);
        let ru = self.root(u);
        self.g[rv as usize] = ru as u32;
        if rv != ru {
            self.size[ru as usize] += self.size[rv as usize];
        }
    }
    fn same(&mut self, v: u32, u: u32) -> bool {
        self.root(v) == self.root(u)
    }
    fn size(&mut self, v: u32) -> usize {
        let rv = self.root(v);
        self.size[rv as usize] as usize
    }
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, mut f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: FnMut(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: FnMut(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let ab = read_vec(m, || read_tuple!(u32, u32));
    let q: usize = read();
    let xyz = read_vec(q, || read_tuple!(usize, usize, usize));

    const B: usize = 330;

    let count = |uf: &mut UnionFind, x: usize, y: usize| {
        if uf.same(x as u32 - 1, y as u32 - 1) {
            uf.size(x as u32 - 1)
        } else {
            uf.size(x as u32 - 1) + uf.size(y as u32 - 1)
        }
    };

    let mut ufs = chain(
        once(UnionFind::new(n)),
        (0..)
            .take_while(|i| i * B < m)
            .scan(UnionFind::new(n), |uf, i| {
                (i * B..(i + 1) * B).take_while(|&j| j < m).for_each(|j| {
                    uf.unite(ab[j].0 - 1, ab[j].1 - 1);
                });
                Some(uf.clone())
            }),
    )
    .collect_vec();

    let targets = (0..q).fold(vec![vec![]; ufs.len()], |mut targets, i| {
        let idx = lower_bound_int(0, ufs.len(), |j| {
            let uf = &mut ufs[j];
            count(uf, xyz[i].0, xyz[i].1).cmp(&xyz[i].2)
        }) - 1;
        targets[idx].push(i);
        targets
    });

    let ans = (0..ufs.len())
        .filter(|&i| !targets[i].is_empty())
        .fold(vec![0; q], |ans, i| {
            let uf = &mut ufs[i];
            let ans = (i * B..(i + 1) * B)
                .take_while(|&j| j < m)
                .fold(ans, |ans, j| {
                    uf.unite(ab[j].0 - 1, ab[j].1 - 1);

                    let ans = targets[i]
                        .citer()
                        .map(|k| (k, xyz[k]))
                        .filter(|&(_k, (x, y, z))| count(uf, x, y) >= z)
                        .fold(ans, |mut ans, (k, _)| {
                            if ans[k] == 0 {
                                ans[k] = j + 1;
                            }
                            ans
                        });
                    ans
                });

            ans
        });
    for a in ans {
        println!("{}", a);
    }
}
