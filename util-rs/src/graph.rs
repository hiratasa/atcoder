#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

use cargo_snippet::snippet;

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

#[snippet("graph")]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[snippet("graph")]
#[allow(dead_code)]
impl Edge {
    fn from_stdin() -> Edge {
        let (from, to) = read_tuple!(usize, usize);
        Edge {
            from: from - 1,
            to: to - 1,
        }
    }

    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
        }
    }
}

#[snippet("graph")]
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Graph {
    out_edges: Vec<Vec<Edge>>,
}

#[snippet("graph")]
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

    fn dfs(&self, visited: &mut Vec<bool>, v: usize) {
        visited[v] = true;

        for e in &self.out_edges[v] {
            if visited[e.to] {
                continue;
            }

            self.dfs(visited, e.to);
        }
    }
}

#[snippet("undirected_graph")]
#[allow(dead_code)]
struct UndirectedGraph {
    adjs: Vec<Vec<usize>>,
}

#[snippet("undirected_graph")]
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

#[allow(dead_code)]
fn tree_dfs(g: &UndirectedGraph, p: usize, v: usize) {
    for &u in &g.adjs[v] {
        if u == p {
            continue;
        }

        tree_dfs(g, v, u);
    }
}
