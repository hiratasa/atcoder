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
}

#[allow(dead_code)]
impl FromStr for Edge {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut it = s.split_whitespace();

        let from: usize = it.next().unwrap().parse()?;
        let to: usize = it.next().unwrap().parse()?;

        Ok(Edge {
            from: from - 1,
            to: to - 1,
        })
    }
}

#[allow(dead_code)]
impl Edge {
    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
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
    fn from_stdin(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];

        for _ in 0..m {
            let e: Edge = read();

            out_edges[e.from].push(e);
            out_edges[e.to].push(e.rev());
        }

        Graph { out_edges }
    }

    fn dfs(&self, visited: &mut Vec<bool>, colors: &mut Vec<bool>, v: usize) -> bool {
        visited[v] = true;

        let mut bipartite = true;
        for e in &self.out_edges[v] {
            if visited[e.to] {
                if colors[e.to] == colors[v] {
                    bipartite = false;
                }
                continue;
            }

            colors[e.to] = !colors[v];
            bipartite &= self.dfs(visited, colors, e.to);
        }

        bipartite
    }
}

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let g = Graph::from_stdin(n, m);

    let mut num_one = 0;
    let mut num_bipartite = 0;
    let mut num_unbipartite = 0;
    let mut visited = vec![false; n];
    let mut colors = vec![false; n];
    for v in 0..n {
        if visited[v] {
            continue;
        }

        if g.out_edges[v].is_empty() {
            num_one += 1;
            continue;
        }

        let bipartite = g.dfs(&mut visited, &mut colors, v);
        if bipartite {
            num_bipartite += 1;
        } else {
            num_unbipartite += 1;
        }
    }

    let ans = 2 * num_bipartite * num_bipartite
        + num_unbipartite * num_unbipartite
        + 2 * num_bipartite * num_unbipartite
        + num_one * num_one
        + 2 * num_one * (n - num_one);
    println!("{}", ans);
}
