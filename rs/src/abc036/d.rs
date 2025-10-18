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
            adjs[u - 1].push(v - 1);
            adjs[v - 1].push(u - 1);
        }
        UndirectedGraph { adjs }
    }
}

const M: usize = 1_000_000_007;

fn dfs(g: &UndirectedGraph, v: usize, p: usize) -> (usize, usize) {
    g.adjs[v]
        .iter()
        .copied()
        .filter(|&u| u != p)
        .map(|u| dfs(g, u, v))
        .fold(
            (1, 1),
            |(total_white, total_black), (nums_white, nums_black)| {
                (
                    total_white * (nums_white + nums_black) % M,
                    total_black * nums_white % M,
                )
            },
        )
}

fn main() {
    let n: usize = read();

    let g = UndirectedGraph::from_stdin(n, n - 1);

    let (total_white, total_black) = dfs(&g, 0, n);
    let ans = (total_white + total_black) % M;

    println!("{}", ans);
}
