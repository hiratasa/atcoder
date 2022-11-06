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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
    let s = read_str();
    let k = read::<usize>();

    let cidxs = s
        .citer()
        .rev()
        .enumerate()
        .fold(vec![vec![]; 26], |mut cidxs, (i, c)| {
            cidxs[c as usize - 'a' as usize].push(i);
            cidxs
        });

    let next = |i: usize, c: usize| {
        let idx = cidxs[c]
            .binary_search_by(|&j| j.cmp(&i).then(Ordering::Greater))
            .unwrap_err();

        if idx == 0 {
            None
        } else {
            Some(cidxs[c][idx - 1])
        }
    };

    let n = s.len();
    let mut dp = s
        .citer()
        .rev()
        .enumerate()
        .fold(vec![0usize; n + 1], |mut dp, (i, c)| {
            let c = c as usize - 'a' as usize;

            dp[i + 1] = if let Some(i1) = next(i, c) {
                dp[i1 + 1..=i]
                    .citer()
                    .fold(0usize, |x, y| x.saturating_add(y))
            } else {
                dp[..=i]
                    .citer()
                    .fold(0usize, |x, y| x.saturating_add(y))
                    .saturating_add(/* empty */ 1)
            };

            dp
        });

    if dp.citer().fold(0usize, |x, y| x.saturating_add(y)) < k {
        println!("Eel");
        return;
    }

    for i in 1..=n {
        dp[i] = dp[i].saturating_add(dp[i - 1]);
    }

    let ans = successors(Some((n, k + /* empty */ 1)), |&(m, r)| {
        if r == 1 {
            None
        } else {
            let r = r - 1;

            let v = (0..26)
                .scan(0, |x, c| {
                    *x = next(m, c)
                        .map_or(0, |ii| dp[ii].saturating_add(1))
                        .saturating_add(*x);
                    Some(*x)
                })
                .collect::<Vec<_>>();
            let c = v.citer().position(|x| x >= r).unwrap();
            let r = r - c.checked_sub(1).map_or(0, |cc| v[cc]);

            Some((next(m, c).unwrap(), r))
        }
    })
    .skip(1)
    .map(|(i, _)| s[n - 1 - i])
    .collect::<Vec<_>>();

    println!("{}", ans.citer().join(""));
}
