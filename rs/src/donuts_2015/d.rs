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
use itertools::{Itertools, chain, iproduct, izip};
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
    let (_n, k) = read_tuple!(usize, usize);

    let c0 = read_row::<usize>();
    let c = c0.iter().copied().collect::<BTreeSet<_>>();

    let q: usize = read();
    let d = read_vec(q, || read::<usize>());

    let total = c.iter().copied().last().unwrap() - c.iter().copied().next().unwrap();
    let w = c
        .iter()
        .copied()
        .tuple_windows()
        .map(|(c1, c2)| c2 - c1)
        .collect::<BTreeMultiSet<_>>();
    let (w1, s, r) = w
        .iter()
        .rev()
        .scan((0usize, 0usize, usize::MAX), |(cumsum, r, prev), &ww| {
            if *prev == ww {
                *r += 1;
            } else {
                *r = 1;
                *prev = ww;
            }
            *cumsum += ww;
            Some((ww, *cumsum, *r))
        })
        .take(k - 1)
        .last()
        .unwrap_or((usize::MAX, 0, 0));

    println!("{}", total - s);

    let fix_state = |w: &BTreeMultiSet<usize>, w1: &mut usize, r: &mut usize, s: &mut usize| {
        if *r > w.count(w1) {
            let next_w1 = *w.range(..*w1).next_back().unwrap();

            *s -= *w1;
            *s += next_w1;

            *r = 1;
            *w1 = next_w1;
        }
    };

    let update_state_by_add = |w: &mut BTreeMultiSet<usize>,
                               w1: &mut usize,
                               r: &mut usize,
                               s: &mut usize,
                               wadd: usize| {
        w.insert(wadd);
        if wadd > *w1 {
            *r -= 1;
            *s = *s - *w1 + wadd;
            if *r == 0 {
                *w1 = *w.range(*w1 + 1..).next().unwrap();
                *r = w.count(w1);
            }
        }
    };

    let update_state_by_remove = |w: &mut BTreeMultiSet<usize>,
                                  w1: &mut usize,
                                  r: &mut usize,
                                  s: &mut usize,
                                  wremove: usize| {
        w.remove(&wremove);
        if wremove < *w1 {
        } else if wremove == *w1 {
            fix_state(w, w1, r, s);
        } else {
            *r += 1;
            *s = *s + *w1 - wremove;
            fix_state(w, w1, r, s);
        }
    };

    d.iter()
        .copied()
        .map(|dd| c0[dd - 1])
        .scan((c, w, w1, r, s, total), |(c, w, w1, r, s, total), cc| {
            let ccprev = c.range(..cc).copied().next_back();
            let ccnext = c.range(cc + 1..).copied().next();

            c.remove(&cc);

            if let Some(ccprev) = ccprev {
                if let Some(ccnext) = ccnext {
                    let wremove1 = cc - ccprev;
                    let wremove2 = ccnext - cc;
                    let wadd = ccnext - ccprev;
                    update_state_by_add(w, w1, r, s, wadd);
                    update_state_by_remove(w, w1, r, s, wremove1);
                    update_state_by_remove(w, w1, r, s, wremove2);
                } else {
                    let wremove = cc - ccprev;
                    *total -= wremove;
                    update_state_by_remove(w, w1, r, s, wremove);
                }
            } else if let Some(ccnext) = ccnext {
                let wremove = ccnext - cc;
                *total -= wremove;
                update_state_by_remove(w, w1, r, s, wremove);
            } else {
                unreachable!();
            }

            Some(*total - *s)
        })
        .for_each(|ans| println!("{}", ans));
}
