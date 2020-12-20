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
use itertools::{chain, iproduct, izip, unfold, Itertools};
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

fn process(
    adjs: &Vec<Vec<usize>>,
    radjs: &Vec<Vec<usize>>,
    adjs2: &Vec<Vec<usize>>,
    radjs2: &Vec<Vec<usize>>,
    unavailable: &Vec<bool>,
) -> usize {
    let n = adjs.len();
    let mut nums = vec![0; n];

    let zeros = (0..n)
        .filter(|&i| radjs[i].is_empty() && radjs2[i].is_empty())
        .collect_vec();

    unfold(zeros, |zeros| {
        if let Some(z) = zeros.pop() {
            let aa = chain(&adjs[z], &adjs2[z]);
            aa.clone().copied().for_each(|u| {
                nums[u] += 1;
                if nums[u] == radjs[u].len() + radjs2[u].len() && !unavailable[u] {
                    zeros.push(u);
                }
            });
            Some(())
        } else {
            None
        }
    })
    .count()
}

fn main() {
    let n: usize = read();

    let a: usize = read();

    let (adjs, radjs) = (0..a).fold(
        (vec![vec![]; n], vec![vec![]; n]),
        |(mut adjs, mut radjs), _| {
            let (x, y) = read_tuple!(usize, usize);
            adjs[y - 1].push(x - 1);
            radjs[x - 1].push(y - 1);
            (adjs, radjs)
        },
    );

    let b: usize = read();
    let uv = read_vec(b, || read_tuple!(usize, usize))
        .into_iter()
        .map(|(u, v)| (u - 1, v - 1))
        .collect_vec();

    let ans = (0..1 << b)
        .map(|bs| {
            let available_edges = uv
                .iter()
                .enumerate()
                .filter(|(i, _)| (bs & (1 << i)) > 0)
                .map(|(_, uv)| *uv)
                .collect_vec();
            let unavailable_u = uv
                .iter()
                .copied()
                .enumerate()
                .filter(|(i, _)| (bs & (1 << i)) == 0)
                .fold(vec![false; n], |mut unavailable_u, (_i, (u, _v))| {
                    unavailable_u[u] = true;
                    unavailable_u
                });

            let (adjs2, radjs2) = available_edges.iter().copied().fold(
                (vec![vec![]; n], vec![vec![]; n]),
                |(mut adjs, mut radjs), (u, v)| {
                    adjs[u].push(v);
                    radjs[v].push(u);
                    (adjs, radjs)
                },
            );
            process(&adjs, &radjs, &adjs2, &radjs2, &unavailable_u)
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
