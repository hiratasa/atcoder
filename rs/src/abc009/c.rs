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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct BTreeMultiSet<T>
where
    T: Ord,
{
    length: usize,
    m: std::collections::BTreeMap<T, usize>,
}
#[allow(dead_code)]
impl<T> BTreeMultiSet<T>
where
    T: Ord,
{
    fn new() -> BTreeMultiSet<T> {
        Self {
            length: 0,
            m: std::collections::BTreeMap::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.m.is_empty()
    }
    fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        self.m.contains_key(value)
    }
    fn len(&self) -> usize {
        self.length
    }
    fn count<Q>(&self, value: &Q) -> usize
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        *self.m.get(value).unwrap_or(&0)
    }
    fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        self.m.get_key_value(value).map(|(k, _v)| k)
    }
    fn first(&self) -> Option<&T> {
        self.m.iter().next().map(|(k, _v)| k)
    }
    fn last(&self) -> Option<&T> {
        self.m.iter().next_back().map(|(k, _v)| k)
    }
    fn clear(&mut self) {
        self.length = 0;
        self.m.clear();
    }
    fn insert(&mut self, value: T) {
        self.length += 1;
        *self.m.entry(value).or_insert(0) += 1;
    }
    fn append(&mut self, other: &mut BTreeMultiSet<T>) {
        self.length += other.length;
        other.length = 0;
        std::mem::take(&mut other.m).into_iter().for_each(|(k, v)| {
            *self.m.entry(k).or_insert(0) += v;
        });
    }
    fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        if let Some(c) = self.m.get_mut(value) {
            self.length -= 1;
            *c -= 1;
            if *c == 0 {
                self.m.remove(value);
            }
            true
        } else {
            false
        }
    }
    fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.m.iter().flat_map(|(v, m)| (0..*m).map(move |_| v))
    }
    fn unique(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.m.keys()
    }
    fn range<'a, K, R>(&'a self, range: R) -> impl 'a + DoubleEndedIterator<Item = &T>
    where
        K: Ord + ?Sized,
        R: std::ops::RangeBounds<K>,
        T: std::borrow::Borrow<K>,
    {
        self.m
            .range(range)
            .flat_map(|(v, m)| (0..*m).map(move |_| v))
    }
}
#[allow(dead_code)]
impl<T> BTreeMultiSet<T>
where
    T: Ord + Clone,
{
    fn pop_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let first = self.first().unwrap().clone();
        self.remove(&first);
        Some(first)
    }
    fn pop_last(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let last = self.last().unwrap().clone();
        self.remove(&last);
        Some(last)
    }
}
#[allow(dead_code)]
impl<'a, T> Extend<&'a T> for BTreeMultiSet<T>
where
    T: 'a + Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for value in iter {
            self.insert(value.clone());
        }
    }
}
#[allow(dead_code)]
impl<T> Extend<T> for BTreeMultiSet<T>
where
    T: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.insert(value);
        }
    }
}
#[allow(dead_code)]
impl<T> std::iter::FromIterator<T> for BTreeMultiSet<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);

    let s = read_str();

    let t = s.citer().collect::<BTreeMultiSet<_>>();

    let ans = (0..n)
        .scan((t, 0usize), |(t, d), i| {
            let c = t
                .unique()
                .copied()
                .map(|c| {
                    let mut t2 = t.clone();
                    t2.remove(&c);
                    s.citer().skip(i + 1).for_each(|cc| {
                        t2.remove(&cc);
                    });
                    let d2 = if c == s[i] {
                        t2.len() + *d
                    } else {
                        t2.len() + *d + 1
                    };
                    (c, d2)
                })
                .find(|&(_c, kk)| kk <= k)
                .unwrap()
                .0;
            t.remove(&c);
            if c != s[i] {
                *d += 1;
            }
            Some(c)
        })
        .to_string();

    println!("{}", ans);
}
