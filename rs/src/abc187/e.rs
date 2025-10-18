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

fn calc_depth(adjs: &Vec<Vec<usize>>, depth: &mut Vec<usize>, v: usize, p: usize) {
    adjs[v].iter().copied().filter(|&u| u != p).for_each(|u| {
        depth[u] = depth[v] + 1;
        calc_depth(adjs, depth, u, v);
    });
}

fn calc_sum(
    adjs: &Vec<Vec<usize>>,
    c: &Vec<i64>,
    sum: &mut Vec<i64>,
    carry: i64,
    v: usize,
    p: usize,
) {
    sum[v] = c[v] + carry;
    adjs[v].iter().copied().filter(|&u| u != p).for_each(|u| {
        calc_sum(adjs, c, sum, sum[v], u, v);
    });
}

fn main() {
    let n: usize = read();

    let ab = read_vec(n - 1, || read_tuple!(usize, usize))
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect_vec();

    let q: usize = read();

    let tex = read_vec(q, || read_tuple!(usize, usize, i64))
        .into_iter()
        .map(|(t, e, x)| (t, e - 1, x))
        .collect_vec();

    let adjs = ab.citer().fold(vec![vec![]; n], |mut adjs, (a, b)| {
        adjs[a].push(b);
        adjs[b].push(a);
        adjs
    });

    let mut depth = vec![0; n];
    calc_depth(&adjs, &mut depth, 0, n);

    let c = tex.citer().fold(vec![0i64; n], |mut c, (t, e, x)| {
        let (a, b) = ab[e];
        let (a, b) = if t == 1 { (a, b) } else { (b, a) };

        if depth[a] < depth[b] {
            c[0] += x;
            c[b] -= x;
        } else {
            c[a] += x;
        }

        c
    });
    let mut sum = vec![0; n];
    calc_sum(&adjs, &c, &mut sum, 0, 0, n);
    for s in sum {
        println!("{}", s);
    }
}
