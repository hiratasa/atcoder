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

#[allow(dead_code)]
struct Graph {
    adjs: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Graph {
    fn from_stdin(n: usize, m: usize) -> Graph {
        let mut adjs = vec![vec![]; n];
        for _ in 0..m {
            let (u, v) = read_tuple!(usize, usize);
            adjs[u - 1].push(v - 1);
        }
        Graph { adjs }
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let g = Graph::from_stdin(n, m);

    let s = read::<String>();
    let idx = s.find("group").unwrap() + "group".len();
    let group: usize = s
        .chars()
        .skip(idx)
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    let group = group - 1;

    let a = s
        .chars()
        .skip(idx)
        .skip_while(|c| c.is_numeric())
        .fold(vec![false], |mut a, c| {
            if c == '"' {
                a.push(false);
            } else {
                *a.last_mut().unwrap() = true;
            }

            a
        });
    let mut init = vec![false; n];
    init[group] = true;
    let ans = a
        .iter()
        .copied()
        .fold(init, |prev, aa| {
            if aa {
                (0..n).map(|i| g.adjs[i].iter().any(|&j| prev[j])).collect()
            } else {
                let prev_count = prev.iter().filter(|&&b| b).count();
                (0..n)
                    .map(|i| g.adjs[i].iter().filter(|&&j| prev[j]).count() != prev_count)
                    .collect()
            }
        })
        .into_iter()
        .filter(|&b| b)
        .count();

    println!("{}", ans);
}
