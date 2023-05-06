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
use itertools::{chain, iproduct, izip, Itertools};
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

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let st = read_vec(m, || read_tuple!(usize, usize));

    let ins = st
        .iter()
        .copied()
        .enumerate()
        .fold(vec![vec![]; n + 1], |mut ins, (i, (s, _))| {
            ins[s - 1].push(i);
            ins
        });

    let outs = st
        .iter()
        .copied()
        .enumerate()
        .fold(vec![vec![]; n + 1], |mut outs, (i, (_, t))| {
            outs[t].push(i);
            outs
        });

    let all = (0..m).collect::<FxHashSet<_>>();
    let ans = (0..n)
        .fold(
            (FxHashSet::default(), all),
            |(mut current, mut ok): (FxHashSet<usize>, FxHashSet<usize>), i| {
                current.extend(ins[i].iter());

                for &o in &outs[i] {
                    current.remove(&o);
                }

                if current.len() == 1 {
                    ok.remove(current.iter().next().unwrap());
                }

                (current, ok)
            },
        )
        .1
        .iter()
        .copied()
        .sorted()
        .collect_vec();

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a + 1);
    }
}
