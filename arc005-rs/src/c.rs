#[allow(unused_imports)]
use bitset_fixed::BitSet;
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

fn dfs(
    a: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    l: usize,
    v: (usize, usize),
    g: (usize, usize),
) -> bool {
    if v == g {
        return true;
    }

    if l >= visited.len() {
        return false;
    }

    if visited[l][v.0][v.1] {
        return false;
    }

    visited[l][v.0][v.1] = true;

    let h = a.len();
    let w = a[0].len();
    let deltas = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

    for &delta in &deltas {
        let (nx, ny) = (v.0.wrapping_add(delta.0), v.1.wrapping_add(delta.1));

        if nx < h && ny < w {
            let nl = if a[nx][ny] { l } else { l + 1 };

            if dfs(a, visited, nl, (nx, ny), g) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let c = read_vec(h, || read_str());

    let s = c
        .iter()
        .enumerate()
        .find_map(|(i, r)| {
            r.iter()
                .enumerate()
                .find(|&(_, &cc)| cc == 's')
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let g = c
        .iter()
        .enumerate()
        .find_map(|(i, r)| {
            r.iter()
                .enumerate()
                .find(|&(_, &cc)| cc == 'g')
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let a = c
        .iter()
        .map(|r| r.iter().copied().map(|cc| cc != '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![vec![false; w]; h]; 3];
    if dfs(&a, &mut visited, 0, s, g) {
        println!("YES");
    } else {
        println!("NO");
    }
}
