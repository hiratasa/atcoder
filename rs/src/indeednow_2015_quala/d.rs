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

fn calc(
    c: &mut Vec<usize>,
    s: &mut FxHashMap<Vec<usize>, usize>,
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    pi: usize,
    pj: usize,
    r: usize,
) {
    assert!(c[i * w + j] == 0);

    if r < 12 {
        it!((usize::MAX, 0), (0, usize::MAX), (1, 0), (0, 1))
            .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .filter(|&t| t != (pi, pj))
            .for_each(|(ni, nj)| {
                let idx = i * w + j;
                let nidx = ni * w + nj;

                c.swap(idx, nidx);
                calc(c, s, h, w, ni, nj, i, j, r + 1);
                c.swap(idx, nidx);
            });
    }

    s.insert(c.clone(), r);
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let c = read_mat::<usize>(h);

    let (i, j) = iproduct!(0..h, 0..w).find(|&(i, j)| c[i][j] == 0).unwrap();

    let mut c = c.iter().flatten().copied().collect_vec();
    let mut s1 = FxHashMap::default();
    calc(&mut c, &mut s1, h, w, i, j, h, w, 0);

    let mut d = (1..=h * w - 1).chain(once(0)).collect_vec();
    let mut s2 = FxHashMap::default();
    calc(&mut d, &mut s2, h, w, h - 1, w - 1, h, w, 0);

    let ans = s1
        .iter()
        .filter_map(|(k, r1)| s2.get(k).map(|r2| r1 + r2))
        .min()
        .unwrap();
    println!("{}", ans);
}
