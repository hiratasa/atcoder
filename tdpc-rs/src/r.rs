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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
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

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}
#[allow(dead_code)]
impl Edge {
    fn new(from: usize, to: usize, weight: usize) -> Self {
        Edge { from, to, weight }
    }
    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            ..*self
        }
    }
}
impl std::convert::From<(usize, usize)> for Edge {
    fn from(t: (usize, usize)) -> Self {
        Edge::new(t.0, t.1, 1)
    }
}
impl std::convert::From<&(usize, usize)> for Edge {
    fn from(t: &(usize, usize)) -> Self {
        Edge::from(*t)
    }
}
impl std::convert::From<(usize, usize, usize)> for Edge {
    fn from(t: (usize, usize, usize)) -> Self {
        Edge::new(t.0, t.1, t.2)
    }
}
impl std::convert::From<&(usize, usize, usize)> for Edge {
    fn from(t: &(usize, usize, usize)) -> Self {
        Edge::from(*t)
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Graph {
    out_edges: Vec<Vec<Edge>>,
    in_edges: Vec<Vec<Edge>>,
}
#[allow(dead_code)]
impl Graph {
    fn new(n: usize) -> Graph {
        Graph {
            out_edges: vec![vec![]; n],
            in_edges: vec![vec![]; n],
        }
    }
    fn from_edges_directed<T, I>(n: usize, edges: I) -> Graph
    where
        I: IntoIterator<Item = T>,
        T: std::convert::Into<Edge>,
    {
        let mut g = Graph::new(n);
        for edge in edges {
            let e = edge.into();
            g.add_edge(e);
        }
        g
    }
    fn from_edges1_directed<T, I>(n: usize, edges: I) -> Graph
    where
        I: IntoIterator<Item = T>,
        T: std::convert::Into<Edge>,
    {
        Graph::from_edges_directed(
            n,
            edges
                .into_iter()
                .map(|e| e.into())
                .map(|e| Edge::new(e.from - 1, e.to - 1, e.weight)),
        )
    }
    fn from_edges_undirected<T, I>(n: usize, edges: I) -> Graph
    where
        I: IntoIterator<Item = T>,
        T: std::convert::Into<Edge>,
    {
        Graph::from_edges_directed(
            n,
            edges
                .into_iter()
                .map(|e| e.into())
                .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
        )
    }
    fn from_edges1_undirected<T, I>(n: usize, edges: I) -> Graph
    where
        I: IntoIterator<Item = T>,
        T: std::convert::Into<Edge>,
    {
        Graph::from_edges1_directed(
            n,
            edges
                .into_iter()
                .map(|e| e.into())
                .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
        )
    }
    fn size(&self) -> usize {
        self.out_edges.len()
    }
    fn add_edge<E: std::convert::Into<Edge>>(&mut self, e: E) {
        let edge = e.into();
        self.out_edges[edge.from].push(edge);
        self.in_edges[edge.to].push(edge);
    }
}

#[allow(dead_code)]
fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;

    for edge in &g.out_edges[v] {
        if !visited[edge.to] {
            dfs(g, edge.to, visited, vs);
        }
    }

    vs.push(v);
}

#[allow(dead_code)]
fn rev_dfs(
    g: &Graph,
    v: usize,
    idx: usize,
    idxs: &mut Vec<Option<usize>>,
    vs: &mut Vec<usize>,
    parents: &mut Vec<usize>,
) {
    idxs[v] = Some(idx);
    vs.push(v);

    for edge in &g.in_edges[v] {
        if let Some(t) = idxs[edge.from] {
            if t < idx {
                parents.push(t);
            }
        } else {
            rev_dfs(g, edge.from, idx, idxs, vs, parents);
        }
    }
}

#[allow(dead_code)]
fn scc(g: &Graph) -> (Vec<Vec<usize>>, Graph) {
    let mut vs = vec![];
    {
        let mut visited = vec![false; g.size()];
        for v in 0..g.size() {
            if !visited[v] {
                dfs(g, v, &mut visited, &mut vs);
            }
        }
    }

    let mut ret = vec![];
    let mut scc_edges = vec![];
    {
        let mut idxs = vec![None; g.size()];
        for &v in vs.iter().rev() {
            if idxs[v].is_none() {
                let idx = ret.len();
                let mut component = vec![];
                let mut parents = vec![];
                rev_dfs(g, v, idx, &mut idxs, &mut component, &mut parents);
                ret.push(component);
                parents.sort();
                parents.dedup();
                scc_edges.extend(parents.iter().map(|&p| (p, idx)));
            }
        }
    }

    let scc_graph = Graph::from_edges_directed(ret.len(), scc_edges);

    (ret, scc_graph)
}
fn main() {
    let n: usize = read();

    let g = read_mat::<usize>(n);

    let graph = Graph::from_edges_directed(n, iproduct!(0..n, 0..n).filter(|&(i, j)| g[i][j] > 0));

    let (components, scc_graph) = scc(&graph);

    // eprintln!("{:?}, {:?}", components, scc_graph);

    let m = components.len();
    let w = components.iter().map(|c| c.len()).collect_vec();

    let dp0 = (0..m).rev().fold(vec![0; m], |mut dp, i| {
        dp[i] = w[i]
            + scc_graph.out_edges[i]
                .citer()
                .map(|e| dp[e.to])
                .max()
                .unwrap_or(0);
        dp
    });

    let ans = iproduct!((0..m).rev(), (0..m).rev())
        .fold(vec![vec![0; m]; m], |mut dp, (i, j)| {
            dp[i][j] = if i == j {
                w[i] + iproduct!(
                    scc_graph.out_edges[i].citer(),
                    scc_graph.out_edges[i].citer()
                )
                .map(|(e0, e1)| dp[e0.to][e1.to])
                .max()
                .unwrap_or(0)
            } else if j < i {
                w[j] + scc_graph.out_edges[j]
                    .citer()
                    .map(|e| dp[i][e.to])
                    .max()
                    .unwrap_or(dp0[i])
            } else
            /* if i < j */
            {
                dp[j][i]
            };
            // eprintln!("dp[{}][{}] = {}", i, j, dp[i][j]);

            dp
        })
        .into_iter()
        .map(|row| row.citer().max().unwrap())
        .max()
        .unwrap();
    println!("{}", ans);
}
