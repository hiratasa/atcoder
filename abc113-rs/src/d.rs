#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let (h, w, k) = read_tuple!(usize, usize, usize);

    const M: usize = 1_000_000_007;

    if w == 1 {
        println!("1");
        return;
    }

    let dp1 = (2..=w).dp(vec![1, 1], |dp1, i| (dp1[i - 2] + dp1[i - 1]) % M);

    let ans = (1..h).fold(vvec![dp1[w - 1], dp1[w - 2]; 0; w], |prev, _| {
        (0..w)
            .map(|j| {
                if j == 0 {
                    (prev[j] * dp1[w - 1] % M + prev[j + 1] * dp1[w - 2] % M) % M
                } else if j < w - 1 {
                    (prev[j - 1] * dp1[j - 1] % M * dp1[w - j - 1] % M
                        + prev[j] * dp1[j] % M * dp1[w - j - 1] % M
                        + prev[j + 1] * dp1[j] % M * dp1[w - j - 2] % M)
                        % M
                } else {
                    (prev[j - 1] * dp1[j - 1] % M * dp1[w - j - 1] % M
                        + prev[j] * dp1[j] % M * dp1[w - j - 1] % M)
                        % M
                }
            })
            .collect::<Vec<_>>()
    })[k - 1];

    println!("{}", ans);
}
