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
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
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

fn gcd(x: usize, y: usize) -> usize {
    if x == 0 { y } else { gcd(y % x, x) }
}

fn main() {
    let n = read::<usize>();
    let p = read_row::<usize>();

    let b = ((n as f64) * (n as f64).ln() / ((n + 1) as f64).ln().ln()).sqrt() as usize;

    let primes = (2..=n)
        .scan(vec![true; n + 1], |is_prime, p| {
            if is_prime[p] {
                (2..)
                    .map(|i| i * p)
                    .take_while(|&i| i <= n)
                    .for_each(|i| is_prime[i] = false);
                Some(Some(p))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut u = once(0)
        .chain((1..=n).map(|i| {
            if i <= b {
                let mut t = (1..)
                    .map(|j| i * j)
                    .take_while(|&j| j <= n)
                    .map(|j| p[j - 1])
                    .fold(vec![0usize; n + 1], |mut freq, x| {
                        freq[x] += 1;
                        freq
                    });

                for &p in &primes {
                    for j in (1..=n / p).rev() {
                        t[j] += t[j * p];
                    }
                }

                for i in 1..=n {
                    t[i] = t[i] * t[i].saturating_sub(1) / 2;
                }

                for &p in &primes {
                    for j in 1..=n / p {
                        t[j] -= t[j * p];
                    }
                }

                t[2..].citer().sum::<usize>()
            } else {
                (1..)
                    .map(|j| i * j)
                    .take_while(|&j| j <= n)
                    .map(|j| p[j - 1])
                    .tuple_combinations()
                    .filter(|&(x, y)| gcd(x, y) > 1)
                    .count()
            }
        }))
        .collect::<Vec<_>>();

    for &p in &primes {
        for j in 1..=n / p {
            u[j] -= u[j * p];
        }
    }

    let ans0 = u[2..].citer().sum::<usize>();

    // 同じインデックス同士
    let ans1 = if p[0] == 1 { n - 1 } else { n - 2 };

    let ans = ans0 + ans1;

    println!("{}", ans);
}
