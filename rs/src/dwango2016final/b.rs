#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
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

fn lower_bound<F>(mut begin: f64, mut end: f64, epsilon: f64, f: F) -> f64
where
    F: Fn(f64) -> std::cmp::Ordering,
{
    while end - begin >= epsilon && end - begin >= end * epsilon {
        let mid = begin + (end - begin) / 2.0;
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

fn main() {
    let n = read::<usize>();
    let x = read_col::<i64>(n);

    let idx1 = x.citer().position(|xx| xx > 0);
    let idx0 = idx1.map_or(Some(n - 1), |idx| idx.checked_sub(1));

    let ans = lower_bound(1.0, 1e14, 5e-10, |v| {
        let mut init = vec![vec![[i64::MAX; 2]; n]; n];
        for idx in it![idx0, idx1].flatten() {
            init[idx][idx][0] = x[idx].abs();
            init[idx][idx][1] = x[idx].abs();
        }

        let get_x = |i: usize, j: usize, k: usize| {
            if k == 0 { x[i] } else { x[j] }
        };

        let dp = (2..=n)
            .flat_map(|l| (0..=n - l).map(move |i| (i, i + l - 1)))
            .fold(init, |mut dp, (i, j)| {
                if i + 1 < n {
                    dp[i][j][0] = (0..2)
                        .map(|k| dp[i + 1][j][k].saturating_add((x[i] - get_x(i + 1, j, k)).abs()))
                        .min()
                        .unwrap();
                    if dp[i][j][0] != i64::MAX && dp[i][j][0] as f64 > v * x[i].abs() as f64 {
                        dp[i][j][0] = i64::MAX;
                    }
                }
                if j > 0 {
                    dp[i][j][1] = (0..2)
                        .map(|k| dp[i][j - 1][k].saturating_add((x[j] - get_x(i, j - 1, k)).abs()))
                        .min()
                        .unwrap();
                    if dp[i][j][1] != i64::MAX && dp[i][j][1] as f64 > v * x[j].abs() as f64 {
                        dp[i][j][1] = i64::MAX;
                    }
                }

                dp
            });

        if dp[0][n - 1].citer().any(|y| y != i64::MAX) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("{}", ans);
}
