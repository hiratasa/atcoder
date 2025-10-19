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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
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

fn main() {
    let (n, m, p, q, r) = read_tuple!(usize, usize, usize, usize, usize);
    let xyz = read_vec(r, || read_tuple!(usize, usize, usize));

    let yz = xyz.citer().fold(vec![vec![]; n], |mut a, (x, y, z)| {
        a[x - 1].push((y - 1, z));
        a
    });

    let ans = iterate((1usize << p) - 1, |s| {
        // combination with fixed number of elements
        let z = s.trailing_zeros();
        let x = 1 << z;
        let y = s + x;
        y | ((!y & s) >> (z + 1))
    })
    .take_while(|&s| s < 1 << n)
    .map(|s| (s, bitset!(n, s)))
    .map(|(s, bs)| {
        (0..n)
            .filter(|&i| bs[i])
            .flat_map(|i| yz[i].citer())
            .fold_vec2(vec![0; m], |w, (y, z)| (y, w[y] + z))
            .into_iter()
            .sorted_by_key(|ww| Reverse(*ww))
            .take(q)
            .sum::<usize>()
    })
    .max()
    .unwrap();

    println!("{}", ans);
}
