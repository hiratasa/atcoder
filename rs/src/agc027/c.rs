#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::iproduct;
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
            adjs[u - 1].push(v - 1);
            adjs[v - 1].push(u - 1);
        }
        UndirectedGraph { adjs }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Visit {
    NonVisited,
    Visiting,
    Visited,
}

use Visit::*;

fn dfs(
    g: &UndirectedGraph,
    s: &Vec<char>,
    visited: &mut Vec<Vec<Visit>>,
    c: usize,
    v: usize,
) -> bool {
    if visited[c][v] == Visiting {
        return true;
    }

    if visited[c][v] == Visited {
        return false;
    }

    visited[c][v] = Visiting;

    let ok = g.adjs[v]
        .iter()
        .copied()
        .filter(|&u| (s[v] == s[u]) == (c == 0))
        .any(|u| dfs(g, s, visited, (c + 1) % 2, u));
    visited[c][v] = Visited;

    ok
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let s = read_str();

    let g = UndirectedGraph::from_stdin(n, m);

    let ans = iproduct!(0..2, 0..n)
        .scan(vec![vec![NonVisited; n]; 2], |visited, (i, v)| {
            Some(dfs(&g, &s, visited, i, v))
        })
        // .inspect(|b| eprintln!("{:?}", b))
        .any(|b| b);

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
