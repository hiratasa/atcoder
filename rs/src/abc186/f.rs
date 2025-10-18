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

trait SliceCopiedExt<T> {
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>>;
}

impl<V, T> SliceCopiedExt<T> for V
where
    V: std::ops::Deref<Target = [T]>,
    T: Copy,
{
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>> {
        self.iter().copied()
    }
}

trait IteratorExt: Iterator + Sized {
    fn fold_vec<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T);
    fn fold_vec2<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T);
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
}

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

struct BIT<M>
where
    M: Monoid,
{
    len: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Monoid,
{
    fn new(len: usize) -> BIT<M> {
        BIT {
            len,
            values: vec![M::id(); len],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let mut bit = Self::new(vals.len());

        for (i, v) in vals.iter().enumerate() {
            bit.add(i, v.clone());
        }

        bit
    }

    // [0, i)の和
    fn sum(&self, i: usize) -> M::Item {
        let mut s = M::id();
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = M::op(&s, &self.values[(idx - 1) as usize]);
            idx -= idx & -idx;
        }

        return s;
    }

    fn add(&mut self, i: usize, a: M::Item) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = M::op(&self.values[(idx - 1) as usize], &a);
            idx += idx & -idx;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

// 可換群
trait Group: Monoid {
    fn inv(value: &Self::Item) -> Self::Item;
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Group,
{
    // [i, j) の和
    fn sum_between(&self, i: usize, j: usize) -> M::Item {
        M::op(&self.sum(j), &M::inv(&self.sum(i)))
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        struct $name;

        impl Monoid for $name {
            type Item = $t;

            fn id() -> Self::Item {
                $id
            }

            fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
                ($op)(*lhs, *rhs)
            }
        }
    };
}

macro_rules! define_group {
    ($name: ident, $t: ty, $id: expr, $op: expr, $inv: expr) => {
        define_monoid!($name, $t, $id, $op);

        impl Group for $name {
            fn inv(value: &Self::Item) -> Self::Item {
                ($inv)(*value)
            }
        }
    };
}

define_group!(Sum, i64, 0, std::ops::Add::add, std::ops::Neg::neg);

fn main() {
    let (h, w, m) = read_tuple!(usize, usize, usize);

    let ij = read_vec(m, || read_tuple!(usize, usize));

    let i_to_minj = ij.citer().fold_vec2(vec![w; h], |i_to_minj, (x, y)| {
        (x - 1, min(i_to_minj[x - 1], y - 1))
    });

    let j_to_mini = ij.citer().fold_vec2(vec![h; w], |j_to_mini, (x, y)| {
        (y - 1, min(j_to_mini[y - 1], x - 1))
    });

    let j0 = i_to_minj[0];
    let i0 = j_to_mini[0];

    let a0 = i_to_minj
        .citer()
        .take(i0)
        .filter(|&j| j > j0)
        .map(|j| j - j0)
        .sum::<usize>();
    let a1 = i_to_minj
        .citer()
        .take(i0)
        .map(|j| min(j, j0))
        .sum::<usize>();

    let t = i_to_minj
        .citer()
        .take(i0)
        .enumerate()
        .fold(vec![vec![]; w + 1], |mut t, (i, minj)| {
            t[minj].push(i);
            t
        });

    let a2 = j_to_mini
        .citer()
        .take(j0)
        .filter(|&mini| mini > i0)
        .map(|mini| mini - i0)
        .sum::<usize>();

    let a3 = j_to_mini
        .citer()
        .take(j0)
        .enumerate()
        .rev()
        .fold(
            (0usize, BIT::<Sum>::new(h + 1)),
            |(a3, mut bit), (j, mini)| {
                let aa = t[j]
                    .citer()
                    .map(|i| bit.sum_between(i + 1, h + 1) as usize)
                    .sum::<usize>();

                bit.add(mini, 1i64);

                (a3 + aa, bit)
            },
        )
        .0;

    let ans = a0 + a1 + a2 + a3;

    println!("{}", ans);
}
