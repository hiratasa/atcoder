#[allow(unused_imports)]
use bitset_fixed::BitSet;
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

#[allow(dead_code)]
struct UndirectedGraph {
    adjs: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl UndirectedGraph {
    fn from_stdin(n: usize, m: usize) -> UndirectedGraph {
        let mut adjs = vec![vec![]; n];
        for _ in 0..m {
            let (u, v) = read_tuple!(usize, usize);
            adjs[u].push(v);
            adjs[v].push(u);
        }
        UndirectedGraph { adjs }
    }
}

fn dfs(g: &UndirectedGraph, v: usize, p: usize) -> usize {
    let t = g.adjs[v]
        .iter()
        .copied()
        .filter(|&u| u != p)
        .map(|u| dfs(g, u, v))
        .fold((0usize, 0usize), |(sum_nonzero, num_zero), r| {
            if r == 0 {
                (sum_nonzero, num_zero + 1)
            } else {
                (sum_nonzero + r, num_zero)
            }
        });

    if t.1 <= 1 {
        t.0
    } else {
        t.0 + t.1 - 1
    }
}

fn main() {
    let n: usize = read();

    let g = UndirectedGraph::from_stdin(n, n - 1);

    let v0 = match (0..n).find(|&i| g.adjs[i].len() >= 3) {
        None => {
            println!("1");
            return;
        }
        Some(v0) => v0,
    };

    let ans = dfs(&g, v0, n);

    println!("{}", ans);
}
