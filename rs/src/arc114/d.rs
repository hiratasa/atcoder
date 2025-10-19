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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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

#[allow(dead_code)]
fn solve0(a: &[i64], t: &[i64]) -> i64 {
    let n = a.len();
    let k = t.len();
    assert!(k % 2 == 0);

    let mi = min(a.citer().min().unwrap(), t[0]);
    let ma = max(a.citer().max().unwrap(), t[k - 1]);

    let l = (ma - mi + 1) as usize;

    let colors0 = t.citer().fold(vec![false; l], |mut colors, x| {
        (x..ma).for_each(|xx| colors[(xx - mi) as usize] ^= true);
        colors
    });

    (0..n)
        .map(|_| mi..=ma)
        .multi_cartesian_product()
        .filter_map(|v| {
            let ok = izip!(a.citer(), v.citer()).fold(vec![false; l], |mut colors, (s, e)| {
                (min(s, e)..max(s, e)).for_each(|x| colors[(x - mi) as usize] ^= true);
                colors
            }) == colors0;

            if ok {
                Some(
                    izip!(a.citer(), v.citer())
                        .map(|(s, e)| (s - e).abs())
                        .sum::<i64>(),
                )
            } else {
                None
            }
        })
        .min()
        .unwrap_or(i64::MAX)
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);
    let mut a = read_row::<i64>();
    let t = read_row::<i64>();

    a.sort();

    // 累積endpoint数 + 累積startpoint数 = 累積checkpoint数 mod 2
    // が成り立つようにN個のendpointを配置していけばよい
    let xs = a
        .citer()
        .chain(t.citer())
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count() % 2 > 0))
        .collect::<Vec<_>>();

    let dp = xs.citer().fold(vvec![0; i64::MAX; n + 1], |prev, (x, t)| {
        let mut next = vec![i64::MAX; n + 1];

        if t {
            for i in 0..n {
                next[i + 1] = min(next[i + 1], prev[i].saturating_add((x - a[i]).abs()));
            }
        } else {
            next = prev;
        }

        for i in 0..n - 1 {
            next[i + 2] = min(
                next[i + 2],
                next[i].saturating_add((x - a[i]).abs() + (x - a[i + 1]).abs()),
            )
        }

        next
    });

    let ans = dp[n];

    if ans == i64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
