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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    label: usize,
}

#[allow(dead_code)]
impl Edge {
    fn from_stdin() -> Edge {
        let (from, to, label) = read_cols!(usize, usize, usize);
        Edge {
            from: from - 1,
            to: to - 1,
            label,
        }
    }

    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            label: self.label,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Graph {
    out_edges: Vec<Vec<Edge>>,
}

#[allow(dead_code)]
impl Graph {
    fn from_stdin_undirected(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];
        for _ in 0..m {
            let e = Edge::from_stdin();
            out_edges[e.from].push(e);
            out_edges[e.to].push(e.rev());
        }
        Graph { out_edges }
    }

    fn from_stdin_directed(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];
        for _ in 0..m {
            let e = Edge::from_stdin();
            out_edges[e.from].push(e);
        }
        Graph { out_edges }
    }

    fn dfs(&self, visited: &mut Vec<bool>, ans: &mut Vec<usize>, v: usize) {
        let n = self.out_edges.len();

        visited[v] = true;

        for e in &self.out_edges[v] {
            if visited[e.to] {
                continue;
            }

            if e.label == ans[v] {
                ans[e.to] = e.label % n + 1;
            } else {
                ans[e.to] = e.label;
            }

            self.dfs(visited, ans, e.to);
        }
    }
}

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let g = Graph::from_stdin_undirected(n, m);

    let mut visited = vec![false; n];
    let mut ans = vec![1; n];
    g.dfs(&mut visited, &mut ans, 0);

    for a in ans {
        println!("{}", a);
    }
}
