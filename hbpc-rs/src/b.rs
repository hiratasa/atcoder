#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

fn dfs(edges: &[Vec<(usize, i64)>], v: usize, diffs: &mut [i64]) {
    edges[v].citer().for_each(|(u, w)| {
        if diffs[u] != i64::MAX {
            assert!(diffs[u] == diffs[v] + w);
        } else {
            diffs[u] = diffs[v] + w;
            dfs(edges, u, diffs);
        }
    });
}

fn main() {
    let city0 = read::<String>();
    let names = read_row::<String>();
    let n = read::<usize>();
    let rels = read_vec(n, || read_tuple!(String, String, String, String));
    let city1 = read::<String>();
    let time = read::<String>();

    let parse = |time_str: &str| {
        let h = time_str[..2].parse::<i64>().unwrap();
        let m = time_str[3..].parse::<i64>().unwrap();

        h * 60 + m
    };

    let idxs = once(city0.as_str())
        .chain(
            rels.iter()
                .flat_map(|(x, _, y, _)| it![x.as_str(), y.as_str()]),
        )
        .chain(once(city1.as_str()))
        .unique()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect::<FxHashMap<_, _>>();

    let m = idxs.len();

    let edges = rels
        .iter()
        .map(|(x, t0, y, t1)| {
            let idx0 = idxs[x.as_str()];
            let idx1 = idxs[y.as_str()];

            (idx0, idx1, parse(&t1) - parse(&t0))
        })
        .fold(vec![vec![]; m], |mut edges, (src, dst, w)| {
            edges[src].push((dst, w));
            edges[dst].push((src, -w));

            edges
        });

    let mut diffs = vec![i64::MAX; m];
    let idx0 = idxs[city0.as_str()];
    let idx1 = idxs[city1.as_str()];
    diffs[idx0] = 0;
    dfs(&edges, idx0, &mut diffs);

    let h = (parse(&time) - diffs[idx1]).rem_euclid(24 * 60) / 60;

    println!("{}", names[h as usize]);
}
