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
    let (n, m) = read_tuple!(usize, usize);

    let a = read_row::<usize>();

    let set = a.citer().take(m).collect::<BTreeMultiSet<_>>();
    let set2 = a
        .citer()
        .take(m)
        .fold((0..=n).collect(), |mut s: BTreeSet<usize>, aa| {
            s.remove(&aa);
            s
        });
    let k = (0..=n).find(|i| !set.contains(i)).unwrap();

    let ans = min(
        k,
        (m..n)
            .scan((set, set2, k), |(set, set2, k), i| {
                let a0 = a[i - m];
                let a1 = a[i];

                set.insert(a1);
                set2.remove(&a1);
                *k = *set2.range(*k..).next().unwrap();

                set.remove(&a0);
                if !set.contains(&a0) {
                    set2.insert(a0);
                    if a0 < *k {
                        *k = a0;
                    }
                }

                Some(*k)
            })
            .min()
            .unwrap_or(n),
    );
    println!("{}", ans);
}
