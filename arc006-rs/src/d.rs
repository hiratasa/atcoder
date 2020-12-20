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

fn dfs(c: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, v: (usize, usize), b: char) -> usize {
    let h = c.len();
    let w = c[0].len();
    let deltas = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

    let mut stack = vec![v];
    let mut ret = 0;
    while let Some((i, j)) = stack.pop() {
        if visited[i][j] {
            continue;
        }

        if c[i][j] != b {
            continue;
        }

        visited[i][j] = true;

        ret += 1;

        deltas
            .iter()
            .map(|&delta| (i.wrapping_add(delta.0), j.wrapping_add(delta.1)))
            .filter(|next| next.0 < h && next.1 < w)
            .for_each(|next| stack.push(next));
    }

    ret
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let c = read_vec(h, || read_str());

    // a + 2b = X
    let mut visited = vec![vec![false; w]; h];
    let x = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| dfs(&c, &mut visited, (i, j), '.'))
                .filter(|&m| m > 0)
                .count()
        })
        .sum::<usize>()
        - 1;

    // 4a + 3b + 5c = Y
    let mut visited = vec![vec![false; w]; h];
    let y = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| dfs(&c, &mut visited, (i, j), 'o'))
                .filter(|&m| m > 0)
                .count()
        })
        .sum::<usize>();

    // 4a + 2b + 2c = Z
    let mut visited = vec![vec![false; w]; h];
    let z = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| dfs(&c, &mut visited, (i, j), 'o'))
                .filter(|&m| m > 0)
                .filter(|&m| {
                    let sqrt = (m as f64).sqrt().floor() as usize;
                    sqrt * sqrt == m
                })
                .count()
        })
        .sum::<usize>();

    eprintln!("{} {} {}", x, y, z);
    let a = (10 * z - 4 * x - 4 * y) / 20;
    let b = (12 * x + 2 * y - 5 * z) / 20;
    let c = (6 * y - 4 * x - 5 * z) / 20;

    println!("{} {} {}", a, b, c);
}
