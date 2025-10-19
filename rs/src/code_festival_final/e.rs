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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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

trait IteratorExt: Iterator + Sized {
    fn fold_vec<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T);
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T);
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T;
}
impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn fold_vec<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(&v, item);
            v[idx] = t;
            v
        })
    }
    fn fold_vec3<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T,
    {
        self.fold(init, |mut v, item| {
            let t = f(&v, item);
            v.push(t);
            v
        })
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
    let n: usize = read();

    let r = read_row::<i64>();

    let ans = r
        .citer()
        .enumerate()
        .fold_vec3(vec![], |v, (i, rr)| {
            let a0 = izip!(r.citer().take(i), v.citer().pick0(),)
                .filter_map(|(r0, m)| if r0 > rr { Some(m + 1) } else { None })
                .max()
                .unwrap_or(1);

            let a1 = izip!(r.citer().take(i), v.citer().pick1(),)
                .filter_map(|(r0, m)| if r0 < rr { Some(m + 1) } else { None })
                .max()
                .unwrap_or(1);

            (a1, a0)
        })
        .into_iter()
        .flat_map(|(a0, a1)| it!(a0, a1))
        .max()
        .unwrap();

    if ans < 3 {
        println!("0");
    } else {
        println!("{}", ans);
    }
}
