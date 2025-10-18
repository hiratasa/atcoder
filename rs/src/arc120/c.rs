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

define_monoid!(Sum, usize, 0, std::ops::Add::add);

fn main() {
    let n: usize = read();
    let a = read_row::<usize>();
    let b = read_row::<usize>();

    let c = a
        .citer()
        .enumerate()
        .map(|(i, aa)| (aa + i, i))
        .sorted()
        .collect::<Vec<_>>();
    let d = b
        .citer()
        .enumerate()
        .map(|(i, aa)| (aa + i, i))
        .sorted()
        .collect::<Vec<_>>();

    if !c.citer().map(|t| t.0).eq(d.citer().map(|t| t.0)) {
        println!("-1");
        return;
    }

    let idxs = izip!(c.citer().map(|t| t.1), d.citer().map(|t| t.1)).fold(
        vec![0; n],
        |mut idxs, (i, j)| {
            idxs[i] = j;
            idxs
        },
    );

    let ans = idxs
        .citer()
        .enumerate()
        .scan(BIT::<Sum>::new(n), |bit, (i, idx)| {
            bit.add(idx, 1);
            Some(i - bit.sum(idx))
        })
        .sum::<usize>();
    println!("{}", ans);
}
