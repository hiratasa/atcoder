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
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
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

fn dfs(g: &Vec<Vec<(usize, f64)>>, visited: &mut Vec<bool>, table: &mut Vec<f64>, v: usize) {
    if visited[v] {
        return;
    }

    visited[v] = true;

    for &(u, coeff) in &g[v] {
        table[u] = 1.0 / coeff * table[v];
        dfs(g, visited, table, u);
    }
}

fn main() {
    let n: usize = read();

    let convs = (0..n)
        .map(|_| read_cols!(String, f64, String))
        .collect::<Vec<_>>();

    let (name_to_idx, idx_to_name) = convs.iter().fold(
        (BTreeMap::new(), vec![]),
        |(mut name_to_idx, mut idx_to_name), (large, _, small)| {
            if !name_to_idx.contains_key(large) {
                name_to_idx.insert(large, name_to_idx.len());
                idx_to_name.push(large);
            }

            if !name_to_idx.contains_key(small) {
                name_to_idx.insert(small, name_to_idx.len());
                idx_to_name.push(small);
            }

            (name_to_idx, idx_to_name)
        },
    );

    let m = name_to_idx.len();

    let mut g = vec![vec![]; m];
    for (large, coeff, small) in &convs {
        let i = *name_to_idx.get(large).unwrap();
        let j = *name_to_idx.get(small).unwrap();

        g[i].push((j, *coeff));
        g[j].push((i, 1.0 / *coeff))
    }

    let mut visited = vec![false; m];
    let mut table = vec![1.0; m];
    dfs(&g, &mut visited, &mut table, 0);

    let largest_idx = table
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .unwrap()
        .0;
    let smallest_idx = table
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .unwrap()
        .0;

    let coeff = (table[largest_idx] / table[smallest_idx]).round() as i64;

    let largest = idx_to_name[largest_idx];
    let smallest = idx_to_name[smallest_idx];

    println!("1{}={}{}", largest, coeff, smallest);
}
