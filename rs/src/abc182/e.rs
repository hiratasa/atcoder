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

fn main() {
    let (h, w, n, m) = read_tuple!(usize, usize, usize, usize);

    let ab = read_vec(n, || read_tuple!(usize, usize));
    let cd = read_vec(m, || read_tuple!(usize, usize));

    let denkyu = ab
        .citer()
        .fold(vec![vec![false; w]; h], |mut denkyu, (a, b)| {
            denkyu[a - 1][b - 1] = true;
            denkyu
        });
    let block = cd
        .citer()
        .fold(vec![vec![false; w]; h], |mut block, (c, d)| {
            block[c - 1][d - 1] = true;
            block
        });

    let down = (0..h).fold(vec![vec![false; w]; h], |z, i| {
        (0..w).fold(z, |mut z, j| {
            z[i][j] = if i == 0 {
                denkyu[i][j]
            } else {
                denkyu[i][j] || (z[i - 1][j] && !block[i][j])
            };
            z
        })
    });
    let up = (0..h).rev().fold(vec![vec![false; w]; h], |z, i| {
        (0..w).fold(z, |mut z, j| {
            z[i][j] = if i == h - 1 {
                denkyu[i][j]
            } else {
                denkyu[i][j] || (z[i + 1][j] && !block[i][j])
            };
            z
        })
    });
    let right = (0..w).fold(vec![vec![false; w]; h], |z, j| {
        (0..h).fold(z, |mut z, i| {
            z[i][j] = if j == 0 {
                denkyu[i][j]
            } else {
                denkyu[i][j] || (z[i][j - 1] && !block[i][j])
            };
            z
        })
    });
    let left = (0..w).rev().fold(vec![vec![false; w]; h], |z, j| {
        (0..h).fold(z, |mut z, i| {
            z[i][j] = if j == w - 1 {
                denkyu[i][j]
            } else {
                denkyu[i][j] || (z[i][j + 1] && !block[i][j])
            };
            z
        })
    });

    let ans = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| down[i][j] || up[i][j] || right[i][j] || left[i][j])
        .count();
    println!("{}", ans);
}
