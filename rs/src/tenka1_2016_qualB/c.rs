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

fn main() {
    let (n, pq) = read_tuple!(usize, String);
    let p = pq
        .chars()
        .take_while(|&c| c != '/')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let q = pq
        .chars()
        .skip_while(|&c| c != '/')
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let a = read_mat::<usize>(n);

    let p = p as f64 / q as f64;
    let q = 1.0 - p;

    let t = a
        .iter()
        .enumerate()
        .map(|(i, row)| (row.citer().sum::<usize>(), i))
        .collect::<Vec<_>>();
    let u = t
        .citer()
        .sorted_by_key(|&(w, i)| (Reverse(w), i))
        .collect::<Vec<_>>();

    let combi = iterate(vec![1], |prev| {
        once(0)
            .chain(prev.citer())
            .chain(once(0))
            .tuple_windows()
            .map(|(x, y)| x + y)
            .collect::<Vec<_>>()
    })
    .take(n + 1)
    .collect::<Vec<_>>();

    let dp = u
        .citer()
        .enumerate()
        .fold(vec![], |prev: Vec<f64>, (rank, (w, i))| {
            let l = n - 1 - w;
            let table = iproduct!(0..=w, 0..=l).fold(vec![0.0; n + 1], |mut table, (ww, ll)| {
                let r = combi[w][ww] as f64
                    * p.powi(ww as i32)
                    * q.powi((w - ww) as i32)
                    * combi[l][ll] as f64
                    * p.powi(ll as i32)
                    * q.powi((l - ll) as i32);

                table[ww + (l - ll)] += r;

                table
            });

            if rank == 0 {
                // first
                table
            } else {
                iproduct!(0..=n, 0..=n)
                    .filter(|&(prevw, currentw)| {
                        prevw > currentw || (prevw == currentw && u[rank - 1].1 < i)
                    })
                    .fold(vec![0.0; n + 1], |mut dp, (prevw, currentw)| {
                        dp[currentw] += prev[prevw] * table[currentw];
                        dp
                    })
            }
        });

    let ans = dp.citer().sum::<f64>();

    println!("{}", ans);
}
