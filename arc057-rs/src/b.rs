#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::chain;
use itertools::zip;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let (n, k) = read_tuple!(usize, usize);

    let a = read_col::<usize>(n);
    let c = chain(
        std::iter::once(0),
        a.iter().copied().scan(0usize, |acc, aa| {
            *acc += aa;
            Some(*acc)
        }),
    )
    .collect_vec();

    if k == *c.last().unwrap() {
        println!("1");
        return;
    }

    let ans = zip(a, c)
        .skip(1)
        .fold(vec![0usize, 1usize], |prev, (aa, cc)| {
            // pp / cc < bb / (cc + aa)
            // => pp + pp * aa / cc < bb
            let it1 = prev
                .iter()
                .copied()
                .map(|pp| (pp * aa) / cc + 1)
                .filter(|&bb| bb <= aa)
                .zip(&prev)
                .map(|(bb, &pp)| bb + pp);
            let it2 = prev.iter().copied();

            zip(
                chain(std::iter::once(0), it1),
                chain(it2, std::iter::once(std::usize::MAX)),
            )
            .map(|(b1, b2)| min(b1, b2))
            .collect_vec()
        })
        .into_iter()
        // .inspect(|kk| eprintln!("{}", kk))
        .take_while(|&kk| kk <= k)
        .count()
        - 1;

    println!("{}", ans);
}
