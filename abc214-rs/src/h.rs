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

mod detail {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug)]
    pub struct Edge<W = ()>
    where
        W: Copy,
    {
        pub from: usize,
        pub to: usize,
        pub label: W,
    }
    #[allow(dead_code)]
    impl<W> Edge<W>
    where
        W: Copy,
    {
        pub fn new(from: usize, to: usize) -> Self
        where
            W: Default,
        {
            Self {
                from,
                to,
                label: W::default(),
            }
        }
        pub fn new_with_label(from: usize, to: usize, label: W) -> Self {
            Self { from, to, label }
        }
        pub fn rev(&self) -> Self {
            Self {
                from: self.to,
                to: self.from,
                ..*self
            }
        }
        pub fn offset1(&self) -> Self {
            Self {
                from: self.from - 1,
                to: self.to - 1,
                ..*self
            }
        }
    }
    pub type UnweightedEdge = Edge<()>;
    pub type WeightedEdge = Edge<usize>;
    impl std::convert::From<(usize, usize)> for UnweightedEdge {
        fn from(t: (usize, usize)) -> Self {
            UnweightedEdge::new(t.0, t.1)
        }
    }
    impl std::convert::From<&(usize, usize)> for UnweightedEdge {
        fn from(t: &(usize, usize)) -> Self {
            Edge::from(*t)
        }
    }
    impl std::convert::From<(usize, usize, usize)> for WeightedEdge {
        fn from(t: (usize, usize, usize)) -> Self {
            Edge::new_with_label(t.0, t.1, t.2)
        }
    }
    impl std::convert::From<&(usize, usize, usize)> for WeightedEdge {
        fn from(t: &(usize, usize, usize)) -> Self {
            Edge::from(*t)
        }
    }
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct Graph<W = ()>
    where
        W: Copy,
    {
        pub out_edges: Vec<Vec<Edge<W>>>,
        pub in_edges: Vec<Vec<Edge<W>>>,
    }
    #[allow(dead_code)]
    pub type UnweightedGraph = Graph<()>;
    #[allow(dead_code)]
    pub type WeightedGraph = Graph<usize>;
    #[allow(dead_code)]
    impl<W: Copy> Graph<W> {
        pub fn new(n: usize) -> Self {
            Self {
                out_edges: vec![vec![]; n],
                in_edges: vec![vec![]; n],
            }
        }
        pub fn from_edges_directed<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            let mut g = Graph::new(n);
            for edge in edges {
                let e = edge.into();
                g.add_edge(e);
            }
            g
        }
        pub fn from_edges1_directed<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges_directed(n, edges.into_iter().map(|e| e.into()).map(|e| e.offset1()))
        }
        pub fn from_edges_undirected<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges_directed(
                n,
                edges
                    .into_iter()
                    .map(|e| e.into())
                    .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
            )
        }
        pub fn from_edges1_undirected<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges1_directed(
                n,
                edges
                    .into_iter()
                    .map(|e| e.into())
                    .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
            )
        }
        pub fn size(&self) -> usize {
            self.out_edges.len()
        }
        pub fn add_edge<T>(&mut self, e: T)
        where
            Edge<W>: std::convert::From<T>,
        {
            let edge = Edge::from(e);
            self.out_edges[edge.from].push(edge);
            self.in_edges[edge.to].push(edge);
        }
    }
}

use detail::{Edge, Graph};

fn scc(g: &Graph) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
        visited[v] = true;

        for edge in &g.out_edges[v] {
            if !visited[edge.to] {
                dfs(g, edge.to, visited, vs);
            }
        }

        vs.push(v);
    }

    fn rev_dfs(
        g: &Graph,
        v: usize,
        idx: usize,
        visited: &mut Vec<Option<usize>>,
        vs: &mut Vec<usize>,
        scc_edges: &mut Vec<(usize, usize)>,
    ) {
        visited[v] = Some(idx);
        vs.push(v);

        for edge in &g.in_edges[v] {
            if let Some(p_idx) = visited[edge.from] {
                if p_idx != idx {
                    scc_edges.push((p_idx, idx));
                }
            } else {
                rev_dfs(g, edge.from, idx, visited, vs, scc_edges);
            }
        }
    }

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
        let mut visited = vec![None; g.size()];
        for &v in vs.iter().rev() {
            if visited[v].is_none() {
                let mut component = vec![];
                rev_dfs(
                    g,
                    v,
                    ret.len(),
                    &mut visited,
                    &mut component,
                    &mut scc_edges,
                );
                ret.push(component);
            }
        }
    }

    scc_edges.sort();
    scc_edges.dedup();

    (ret, scc_edges)
}

struct MinCostFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
    weights: Vec<i64>,
}

#[allow(dead_code)]
impl MinCostFlowGraph {
    fn new(n: usize) -> MinCostFlowGraph {
        MinCostFlowGraph {
            g: Graph::new(n),
            caps: vec![],
            rev: vec![],
            weights: vec![],
        }
    }

    fn num_vertices(&self) -> usize {
        self.g.size()
    }

    fn num_edges(&self) -> usize {
        self.caps.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize, cost: i64) {
        let idx = self.num_edges();
        let rev_idx = self.num_edges() + 1;

        self.g.add_edge(Edge::new_with_label(from, to, idx));
        self.g.add_edge(Edge::new_with_label(to, from, rev_idx));

        // forward edge
        self.caps.push(cap);
        self.rev.push(rev_idx);
        self.weights.push(cost);

        // backward edge
        self.caps.push(0);
        self.rev.push(idx);
        self.weights.push(-cost);
    }

    fn dijkstra(&self, src: usize, h: &[i64]) -> (Vec<i64>, Vec<Option<(usize, usize)>>) {
        let n = self.num_vertices();

        let mut q = std::collections::BinaryHeap::new();
        let mut costs = vec![std::i64::MAX; n];
        let mut parents = vec![None; n];
        q.push(std::cmp::Reverse((0, src)));
        costs[src] = 0;

        while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
            if cost > costs[v] {
                continue;
            }

            for edge in &self.g.out_edges[v] {
                if self.caps[edge.label] == 0 {
                    continue;
                }

                let next_cost = cost + self.weights[edge.label] - h[edge.to] + h[edge.from];
                assert!(cost <= next_cost);
                if next_cost < costs[edge.to] {
                    q.push(std::cmp::Reverse((next_cost, edge.to)));
                    costs[edge.to] = next_cost;
                    parents[edge.to] = Some((edge.label, v));
                }
            }
        }

        (costs, parents)
    }

    fn min_cost_dp(&self, src: usize) -> Vec<i64> {
        let mut costs = vec![std::i64::MAX; self.num_vertices()];
        costs[src] = 0;

        for v in 0..self.num_vertices() {
            costs[v] = self.g.in_edges[v]
                .citer()
                .filter(|e| self.caps[e.label] > 0)
                .map(|e| costs[e.from] + self.weights[e.label])
                .min()
                .unwrap_or(std::i64::MAX);
        }

        costs
    }

    fn flow(&mut self, src: usize, dst: usize, limit: usize) -> Vec<(usize, i64)> {
        let mut f = 0;
        let mut c = 0;
        let mut v = vec![];
        // DAGなので初回のみdpでポテンシャル求める
        let mut h = self.min_cost_dp(src);

        while f < limit {
            let (costs, parents) = self.dijkstra(src, &h);

            for v in 0..self.num_vertices() {
                h[v] = h[v].saturating_add(costs[v]);
            }

            let f1 = if let Some(f1) = successors(parents[dst], |(_, p)| parents[*p])
                .map(|(id, _p)| self.caps[id])
                .chain(once(limit - f))
                .min()
            {
                f1
            } else {
                break;
            };
            assert!(f1 > 0);
            f += f1;

            successors(parents[dst], |(_, p)| parents[*p]).for_each(|(id, _p)| {
                c += f1 as i64 * self.weights[id];

                self.caps[id] -= f1;
                self.caps[self.rev[id]] += f1;
            });

            v.push((f, c));
        }

        v
    }
}

fn main() {
    let (n, m, k) = read_tuple!(usize, usize, usize);
    let ab = read_vec(m, || read_tuple!(usize, usize));
    let x = read_row::<usize>();

    let g = Graph::from_edges1_directed(n, ab);

    let (components, scc_edges) = scc(&g);

    let w = components
        .iter()
        .map(|vs| vs.citer().map(|v| x[v]).sum::<usize>())
        .collect::<Vec<_>>();

    let n1 = components.len();
    let mut mcg = MinCostFlowGraph::new(2 * n1 + 1);
    let src = 2 * components
        .iter()
        .position(|c| c.citer().any(|v| v == 0))
        .unwrap();
    let dst = 2 * n1;
    for i in 0..n1 {
        mcg.add_edge(2 * i, 2 * i + 1, 1, -(w[i] as i64));
        mcg.add_edge(2 * i, 2 * i + 1, usize::MAX, 0);
        mcg.add_edge(2 * i + 1, dst, usize::MAX, 0);
    }
    for (v, u) in scc_edges {
        assert!(v < u);
        mcg.add_edge(2 * v + 1, 2 * u, usize::MAX, 0);
    }

    let f = mcg.flow(src, dst, k);
    let ans = -f.last().copied().unwrap().1;
    println!("{}", ans);
}
