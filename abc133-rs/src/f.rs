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
    color: usize,
    length: usize,
}

#[allow(dead_code)]
impl Edge {
    fn from_stdin() -> Edge {
        let (from, to, color, length) = read_cols!(usize, usize, usize, usize);
        Edge {
            from: from - 1,
            to: to - 1,
            color,
            length,
        }
    }

    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            ..*self
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

#[allow(dead_code)]
fn dfs(
    g: &Graph,
    tour: &mut Vec<Vec<(usize, usize, usize)>>,
    in_idx: &mut Vec<usize>,
    out_idx: &mut Vec<usize>,
    idx: &mut usize,
    parents: &mut Vec<usize>,
    depth: &mut Vec<usize>,
    dist: &mut Vec<usize>,
    v: usize,
) {
    for &e in &g.out_edges[v] {
        if e.to == parents[v] {
            continue;
        }

        parents[e.to] = v;
        depth[e.to] = depth[v] + 1;
        dist[e.to] = dist[v] + e.length;

        let l = *tour[e.color].last().unwrap();
        tour[e.color].push((*idx, l.1 + 1, l.2 + e.length));
        in_idx[e.to] = *idx;
        *idx += 1;
        dfs(g, tour, in_idx, out_idx, idx, parents, depth, dist, e.to);
        let l2 = *tour[e.color].last().unwrap();
        tour[e.color].push((*idx, l2.1 - 1, l2.2 - e.length));
        out_idx[e.to] = *idx;
        *idx += 1;
    }
}

fn lca(parents: &Vec<Vec<usize>>, depth: &Vec<usize>, mut v: usize, mut u: usize) -> usize {
    if depth[v] > depth[u] {
        swap(&mut v, &mut u);
    }

    assert!(depth[v] <= depth[u]);

    for ps in parents.iter().rev() {
        if depth[v] <= depth[ps[u]] {
            u = ps[u];
        }
    }

    if v == u {
        return v;
    }

    for ps in parents.iter().rev() {
        if ps[v] != ps[u] {
            v = ps[v];
            u = ps[u];
        }
    }

    parents[0][v]
}

fn main() {
    let (n, q) = read_cols!(usize, usize);
    let g = Graph::from_stdin_undirected(n, n - 1);

    let mut tour = vec![vec![(0, 0, 0)]; n];
    let mut in_idx = vec![0; n];
    let mut out_idx = vec![0; n];
    let mut idx = 1;
    let mut parents = vec![vec![0; n]; 20];
    let mut depth = vec![0; n];
    let mut dist = vec![0; n];
    dfs(
        &g,
        &mut tour,
        &mut in_idx,
        &mut out_idx,
        &mut idx,
        &mut parents[0],
        &mut depth,
        &mut dist,
        0,
    );
    in_idx[0] = 0;
    out_idx[0] = 2 * n - 1;

    for i in 1..20 {
        for v in 0..n {
            parents[i][v] = parents[i - 1][parents[i - 1][v]];
        }
    }

    for _ in 0..q {
        let (x, y, mut u, mut v) = read_cols!(usize, usize, usize, usize);
        u -= 1;
        v -= 1;

        let c = lca(&parents, &depth, u, v);

        let ans = [u, v]
            .iter()
            .map(|&i| {
                let idx0 = tour[x]
                    .binary_search(&(in_idx[c], 2 * n, 2 * n))
                    .unwrap_err()
                    - 1;
                let idx1 = tour[x]
                    .binary_search(&(in_idx[i], 2 * n, 2 * n))
                    .unwrap_err()
                    - 1;

                let xnum = tour[x][idx1].1 - tour[x][idx0].1;
                let xdist = tour[x][idx1].2 - tour[x][idx0].2;

                dist[i] - dist[c] - xdist + xnum * y
            })
            .sum::<usize>();

        println!("{}", ans);
    }
}
