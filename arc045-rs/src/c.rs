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

#[allow(dead_code)]
struct UndirectedGraph {
    adjs: Vec<Vec<(usize, usize)>>,
}

#[allow(dead_code)]
impl UndirectedGraph {
    fn from_stdin(n: usize, m: usize) -> Self {
        let mut adjs = vec![vec![]; n];
        for _ in 0..m {
            let edge = read_cols!(usize, usize, usize);
            adjs[edge.0 - 1].push((edge.1 - 1, edge.2));
            adjs[edge.1 - 1].push((edge.0 - 1, edge.2));
        }
        UndirectedGraph { adjs }
    }
}

fn dfs(
    g: &UndirectedGraph,
    x: usize,
    values: &mut BTreeMap<usize, usize>,
    val: usize,
    v: usize,
    p: usize,
) -> usize {
    let ans0 = values.get(&(val ^ x)).copied().unwrap_or(0);
    *values.entry(val).or_insert(0) += 1;

    ans0 + g.adjs[v]
        .iter()
        .copied()
        .map(|(u, cost)| {
            if u == p {
                return 0;
            }

            let next_val = val ^ cost;
            dfs(g, x, values, next_val, u, v)
        })
        .sum::<usize>()
}

fn main() {
    let (n, x) = read_cols!(usize, usize);

    let g = UndirectedGraph::from_stdin(n, n - 1);

    let ans = dfs(&g, x, &mut BTreeMap::new(), 0, 0, n);
    println!("{}", ans);
}
