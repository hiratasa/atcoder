#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn dfs(adjs: &Vec<Vec<usize>>, g: &mut Vec<bool>, v: usize) -> usize {
    let mut ret = 1;

    g[v] = true;

    for u in &adjs[v] {
        if g[*u] {
            continue
        }

        ret += dfs(adjs, g, *u);
    }

    return ret;
}

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let mut adjs = vec![Vec::new(); n];
    for _ in 0..m {
        let (a, b) = read_cols!(usize, usize);
        adjs[a - 1].push(b - 1);
        adjs[b - 1].push(a - 1);
    }

    let mut ans = 0;
    let mut g = vec![false; n];
    for i in 0..n {
        if !g[i] {
            ans = max(ans, dfs(&adjs, &mut g, i));
        }
    }

    println!("{}", ans);
}