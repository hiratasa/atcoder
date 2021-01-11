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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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

const M: usize = 1_000_000_007;

fn mat_mul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();

    iproduct!(0..n, 0..n, 0..n).fold(vec![vec![0; n]; n], |mut c, (i, k, j)| {
        c[i][j] += a[i][k] * b[k][j] % M;
        c[i][j] %= M;
        c
    })
}

fn mat_pow(a: &Vec<Vec<usize>>, mut p: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut b = (0..n)
        .map(|i| {
            repeat_n(0, i)
                .chain(once(1))
                .chain(repeat_n(0, n - (i + 1)))
                .collect_vec()
        })
        .collect_vec();

    let mut x = a.clone();
    while p > 0 {
        if p % 2 > 0 {
            b = mat_mul(&b, &x);
        }

        p >>= 1;
        x = mat_mul(&x, &x);
    }

    b
}

fn main() {
    let (h, r) = read_tuple!(usize, usize);

    let g = read_mat::<u8>(r);

    let a = iproduct!(1..1 << r, 0..r, 0..r)
        .filter(|(s, src, dst)| (s >> src) & 1 > 0 && (s >> dst) & 1 > 0)
        .fold(
            vec![vec![vec![0; 1 << r]; r]; r],
            |mut dp, (s, src, dst)| {
                dp[src][dst][s] = if s.count_ones() == 1 {
                    1
                } else {
                    (0..r)
                        .filter(|&j| j != dst && (s >> j) & 1 > 0 && g[j][dst] > 0)
                        .map(|j| dp[src][j][s ^ (1 << dst)])
                        .fold(0usize, |acc, x| (acc + x) % M)
                };

                dp
            },
        )
        .into_iter()
        .map(|row| {
            row.iter()
                .map(|row2| row2.citer().fold(0usize, |acc, x| (acc + x) % M))
                .collect_vec()
        })
        .collect_vec();
    let b = mat_pow(&a, h);
    println!("{}", b[0][0]);
}
