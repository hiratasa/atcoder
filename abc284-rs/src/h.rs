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

#[allow(dead_code)]
pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
}

fn gcd(x: usize, y: usize) -> usize {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn calc(v: &[usize], t: &[usize], p: usize) -> usize {
    t[v.len()]
        * v.citer()
            .map(|x| pow_mod(2, x / 2, p))
            .fold(1, |x, y| x * y % p)
        % p
        * v.citer()
            .tuple_combinations()
            .map(|(x, y)| pow_mod(2, gcd(x, y), p))
            .fold(1, |x, y| x * y % p)
        % p
}

fn dfs(
    num_selected: usize,
    last_num: usize,
    dup: usize,
    n: usize,
    t: &[usize],
    inv: &[usize],
    p: usize,
    c: usize,
    v: &mut Vec<usize>,
) -> usize {
    if num_selected == n {
        return calc(v, t, p) * c % p;
    }

    (max(1, last_num)..=n - num_selected)
        .map(|s| {
            v.push(s);
            let dup2 = if s == last_num { dup + 1 } else { 1 };
            let c2 = c * (n - num_selected - s + 1..=n - num_selected).fold(1, |x, y| x * y % p)
                % p
                * inv[dup2]
                % p
                * inv[s]
                % p;
            let r = dfs(num_selected + s, s, dup2, n, t, inv, p, c2, v);
            v.pop();
            r
        })
        .fold(0, |x, y| (x + y) % p)
}

fn main() {
    let (n, k, p) = read_tuple!(usize, usize, usize);

    let combi = iterate(vec![1], |prev| {
        once(0)
            .chain(prev.citer())
            .chain(once(0))
            .tuple_windows()
            .map(|(x, y)| x + y)
            .collect()
    })
    .take(31)
    .collect::<Vec<_>>();

    let inv = (2..=n).fold(vec![1usize, 1], |mut inv, i| {
        inv.push((p - p / i) % p * inv[p % i] % p);
        inv
    });

    let t = (0..=n)
        .map(|i| {
            if i < k {
                0
            } else {
                (1..=k)
                    .scan(vec![0], |t, j| {
                        let x = (pow_mod(j, i, p) + p
                            - (1..j).map(|l| t[l] * combi[j][l] % p).sum::<usize>() % p)
                            % p;
                        t.push(x);
                        Some(x)
                    })
                    .last()
                    .unwrap()
            }
        })
        .collect::<Vec<_>>();

    let invnfact = (1..=n).fold(1, |x, y| x * inv[y] % p);

    let ans = dfs(0, 1, 0, n, &t, &inv, p, 1, &mut vec![]) * invnfact % p;

    println!("{}", ans);
}
