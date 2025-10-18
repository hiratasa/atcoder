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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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
        let x = $x;
        let mut c = $c;
        c.push(x);
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
        pub fn adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = usize> {
            self.out_edges[v].iter().map(|e| e.to)
        }
        pub fn children<'a>(&'a self, v: usize, p: usize) -> impl 'a + Iterator<Item = usize> {
            self.adjs(v).filter(move |&u| u != p)
        }
        pub fn children_edge<'a>(
            &'a self,
            v: usize,
            p: usize,
        ) -> impl 'a + Iterator<Item = Edge<W>> {
            self.out_edges[v].iter().copied().filter(move |e| e.to != p)
        }
    }
}

use detail::{Edge, Graph};

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

    fn dp(&self, src: usize) -> Vec<i64> {
        let n = self.num_vertices();

        let mut costs = vec![std::i64::MAX; n];
        costs[src] = 0;

        for i in 0..n {
            for &e in &self.g.out_edges[i] {
                if self.caps[e.label] > 0 {
                    costs[e.to] = min(costs[e.to], costs[i].saturating_add(self.weights[e.label]));
                }
            }
        }

        costs
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

    // (流量, コスト)のリストを返す
    fn flow(&mut self, src: usize, dst: usize, limit: usize) -> Vec<(usize, i64)> {
        let mut f = 0;
        let mut c = 0;
        let mut v = vec![];
        let mut h = self.dp(src);

        while f < limit {
            let (costs, parents) = self.dijkstra(src, &h);

            for v in 0..self.num_vertices() {
                h[v] = h[v].saturating_add(costs[v]);
            }

            let f1 = if let Some(f1) = successors(parents[dst], |(_, p)| parents[*p])
                .map(|(id, _p)| self.caps[id])
                .min()
            {
                min(limit - f, f1)
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
    let n = read::<usize>();
    let abc = read_vec(n, || read_tuple!(usize, usize, i64));

    const K: usize = 150;

    let mut g = MinCostFlowGraph::new(2 + 2 * K);
    for (a, b, c) in abc {
        g.add_edge(1 + (a - 1), 1 + K + (b - 1), 1, -c);
    }
    for i in 0..K {
        g.add_edge(0, 1 + i, 1, 0);
        g.add_edge(1 + K + i, 1 + 2 * K, 1, 0);
    }

    let res = g.flow(0, 1 + 2 * K, usize::MAX);

    let k = res.last().unwrap().0;
    println!("{}", k);
    once((0, 0))
        .chain(res)
        .tuple_windows()
        .for_each(|((f0, c0), (f1, c1))| {
            for f in f0 + 1..=f1 {
                println!("{}", -c0 + (-c1 + c0) / (f1 - f0) as i64 * (f - f0) as i64);
            }
        });
}
