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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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

fn solve0(points: &[(usize, usize)]) -> f64 {
    let mut set = FxHashSet::default();
    set.insert(points[0]);

    let mut p = points[0];
    let mut len = 0.0;
    while set.len() < points.len() {
        let q = points
            .citer()
            .filter(|&q| !set.contains(&q))
            .min_by_key(|&q| (q.0 as i64 - p.0 as i64).pow(2) + (q.1 as i64 - p.1 as i64).pow(2))
            .unwrap();

        len += ((q.0 as f64 - p.0 as f64).powi(2) + (q.1 as f64 - p.1 as f64).powi(2)).sqrt();

        p = q;
        set.insert(q);
    }

    len
}

fn solve1(points: &[(usize, usize)]) -> f64 {
    let mut set = FxHashSet::default();
    set.insert(points[0]);

    let mut p = points[0];
    let mut len = 0.0;
    while set.len() < points.len() {
        let q = if points.len() - set.len() >= 3 {
            points
                .citer()
                .filter(|&q| !set.contains(&q))
                .sorted_by_key(|&q| {
                    (q.0 as i64 - p.0 as i64).pow(2) + (q.1 as i64 - p.1 as i64).pow(2)
                })
                .nth(1)
                .unwrap()
        } else {
            points
                .citer()
                .filter(|&q| !set.contains(&q))
                .min_by_key(|&q| {
                    (q.0 as i64 - p.0 as i64).pow(2) + (q.1 as i64 - p.1 as i64).pow(2)
                })
                .unwrap()
        };

        len += ((q.0 as f64 - p.0 as f64).powi(2) + (q.1 as f64 - p.1 as f64).powi(2)).sqrt();

        p = q;
        set.insert(q);
    }

    len
}

fn main() {
    let n = 100;

    let mut ans = vec![];

    for i in 0..n / 2 {
        ans.push((i, 0));
    }

    for i in 0..n / 2 - 1 {
        ans.push((i, 1));
    }

    ans.push((n / 2, 1));

    eprintln!("{}", solve0(&ans));
    eprintln!("{}", solve1(&ans));

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
