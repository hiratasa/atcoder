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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

trait ToString {
    fn to_string(self: Self) -> String;
}
impl<I, T> ToString for I
where
    I: IntoIterator<Item = T>,
    T: std::convert::TryInto<u32>,
{
    fn to_string(self: Self) -> String {
        self.into_iter()
            .map(|t| t.try_into().ok().unwrap())
            .map(|t| std::convert::TryInto::<char>::try_into(t).ok().unwrap())
            .collect()
    }
}

trait Pick0 {
    type Output;
    fn pick0(self) -> Self::Output;
}
impl<T, T2> Pick0 for (T, T2) {
    type Output = T;
    fn pick0(self) -> Self::Output {
        self.0
    }
}
impl<T, T2, T3> Pick0 for (T, T2, T3) {
    type Output = T;
    fn pick0(self) -> Self::Output {
        self.0
    }
}
trait IteratorPick0Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick0,
{
    fn pick0(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick0::pick0)
    }
}
impl<T, I> IteratorPick0Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick0,
{
}
trait Pick1 {
    type Output;
    fn pick1(self) -> Self::Output;
}
impl<T, T2> Pick1 for (T, T2) {
    type Output = T2;
    fn pick1(self) -> Self::Output {
        self.1
    }
}
impl<T, T2, T3> Pick1 for (T, T2, T3) {
    type Output = T2;
    fn pick1(self) -> Self::Output {
        self.1
    }
}
trait IteratorPick1Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick1,
{
    fn pick1(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick1::pick1)
    }
}
impl<T, I> IteratorPick1Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick1,
{
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);

    let s = read_str();

    let t = s.citer().sorted().collect_vec();

    let ans = (0..=n)
        .filter_map(|i| {
            let d = izip!(s.citer(), t.citer())
                .take(i)
                .enumerate()
                .filter(|&(_i, (c1, c2))| c1 != c2)
                .collect_vec();

            let t2 = s
                .citer()
                .enumerate()
                .sorted_by_key(|&(idx, c)| (c, i <= idx, Reverse(idx)))
                .collect_vec();
            let (idxs0, idxs1): (FxHashSet<_>, FxHashSet<_>) = t2
                .citer()
                .take(i)
                .filter(|&(idx, c)| idx >= i || t[idx] != c)
                .pick0()
                .partition(|&idx| idx < i);
            assert!(d.len() == idxs0.len() + idxs1.len());
            let k2 = d.len() + idxs1.len();

            let e = d
                .citer()
                .filter(|(idx, _)| !idxs0.contains(idx))
                .pick1()
                .pick0()
                .collect_vec();
            if k2 > k {
                None
            } else if k >= k2 + 1 {
                (i..=n)
                    .filter(|idx| !idxs1.contains(&idx))
                    .map(|idx| {
                        let e = e.citer().chain(s.get(idx).copied()).sorted().collect_vec();
                        chain(
                            t.citer().take(i),
                            s.citer().enumerate().skip(i).scan(0usize, |next, (j, c)| {
                                if j == idx || idxs1.contains(&j) {
                                    Some(e[replace(next, *next + 1)])
                                } else {
                                    Some(c)
                                }
                            }),
                        )
                        .to_string()
                    })
                    .min()
            } else {
                let e = e.citer().sorted().collect_vec();
                Some(
                    chain(
                        t.citer().take(i),
                        s.citer().enumerate().skip(i).scan(0usize, |next, (j, c)| {
                            if idxs1.contains(&j) {
                                Some(e[replace(next, *next + 1)])
                            } else {
                                Some(c)
                            }
                        }),
                    )
                    .to_string(),
                )
            }
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
