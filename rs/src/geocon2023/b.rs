#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
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
    type Weight = usize;
    pub type UnweightedEdge = Edge<()>;
    pub type WeightedEdge = Edge<Weight>;
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
    impl std::convert::From<(usize, usize, Weight)> for WeightedEdge {
        fn from(t: (usize, usize, Weight)) -> Self {
            Edge::new_with_label(t.0, t.1, t.2)
        }
    }
    impl std::convert::From<&(usize, usize, Weight)> for WeightedEdge {
        fn from(t: &(usize, usize, Weight)) -> Self {
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
    pub type WeightedGraph = Graph<Weight>;
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
        pub fn adjs<'a>(&'a self, v: usize) -> impl 'a + DoubleEndedIterator<Item = usize> {
            self.out_edges[v].iter().map(|e| e.to)
        }
        pub fn children<'a>(
            &'a self,
            v: usize,
            p: usize,
        ) -> impl 'a + DoubleEndedIterator<Item = usize> {
            self.adjs(v).filter(move |&u| u != p)
        }
        pub fn children_edge<'a>(
            &'a self,
            v: usize,
            p: usize,
        ) -> impl 'a + DoubleEndedIterator<Item = Edge<W>> {
            self.out_edges[v].iter().copied().filter(move |e| e.to != p)
        }
    }
}

struct MinCostFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
    weights: Vec<f64>,
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

    fn add_edge(&mut self, from: usize, to: usize, cap: usize, cost: f64) {
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

    fn dp(&self, src: usize) -> Vec<f64> {
        let n = self.num_vertices();

        let mut costs = vec![1e50; n];
        costs[src] = 0.0;

        for i in 0..n {
            for &e in &self.g.out_edges[i] {
                if self.caps[e.label] > 0 {
                    costs[e.to] = f64::min(costs[e.to], costs[i] + self.weights[e.label]);
                }
            }
        }

        costs
    }

    fn dijkstra(&self, src: usize, h: &[f64]) -> (Vec<f64>, Vec<Option<(usize, usize)>>) {
        use ordered_float::OrderedFloat;

        let n = self.num_vertices();

        let mut q = std::collections::BinaryHeap::new();
        let mut costs = vec![std::f64::MAX; n];
        let mut parents = vec![None; n];
        q.push(std::cmp::Reverse((OrderedFloat(0.0), src)));
        costs[src] = 0.0;

        while let Some(std::cmp::Reverse((OrderedFloat(cost), v))) = q.pop() {
            if cost > costs[v] {
                continue;
            }

            for edge in &self.g.out_edges[v] {
                if self.caps[edge.label] == 0 {
                    continue;
                }

                let next_cost = cost + self.weights[edge.label] - h[edge.to] + h[edge.from];
                // assert!(cost <= next_cost);
                if next_cost < costs[edge.to] - 1e-7 {
                    q.push(std::cmp::Reverse((OrderedFloat(next_cost), edge.to)));
                    costs[edge.to] = next_cost;
                    parents[edge.to] = Some((edge.label, v));
                }
            }
        }

        (costs, parents)
    }

    // (流量, コスト)のリストを返す
    fn flow(&mut self, src: usize, dst: usize, limit: usize) -> Vec<(usize, f64)> {
        let mut f = 0;
        let mut c = 0.0;
        let mut v = vec![];
        let mut h = self.dp(src);

        while f < limit {
            let (costs, parents) = self.dijkstra(src, &h);

            for v in 0..self.num_vertices() {
                h[v] = h[v] + costs[v];
            }

            let f1 = if let Some(f1) = successors(parents[dst], |(_, p)| parents[*p])
                .map(|(id, _p)| self.caps[id])
                .min()
            {
                std::cmp::min(limit - f, f1)
            } else {
                break;
            };
            assert!(f1 > 0);
            f += f1;

            successors(parents[dst], |(_, p)| parents[*p]).for_each(|(id, _p)| {
                c += f1 as f64 * self.weights[id];

                self.caps[id] -= f1;
                self.caps[self.rev[id]] += f1;
            });

            v.push((f, c));
        }

        v
    }
}

use detail::{Edge, Graph};

fn main() {
    input! {
        mut xy: [(f64, f64)]
    }

    xy.retain(|&(x, _y)| x != 0.0);

    let n = xy.len();

    let mut g = MinCostFlowGraph::new(n + 2);

    let src = 0;
    let dst = n + 1;

    let pos = xy.citer().filter(|&(x, _)| x > 0.0).collect::<Vec<_>>();
    let neg = xy.citer().filter(|&(x, _)| x < 0.0).collect::<Vec<_>>();

    pos.citer().enumerate().for_each(|(i, (_x, _y))| {
        g.add_edge(src, i + 1, 1, 0.0);
    });

    neg.citer().enumerate().for_each(|(i, (_x, _y))| {
        g.add_edge(pos.len() + 1 + i, dst, 1, 0.0);
    });

    iproduct!(pos.citer().enumerate(), neg.citer().enumerate()).for_each(
        |((i, (x0, y0)), (j, (x1, y1)))| {
            let dist = ((x0 + x1).powi(2) + (y0 - y1).powi(2)).sqrt();

            g.add_edge(i + 1, pos.len() + j + 1, 1, dist - x0.abs() - x1.abs());
        },
    );

    let flows = g.flow(src, dst, usize::MAX);

    let base = xy.citer().map(|(x, _)| x.abs()).sum::<f64>();

    let ans = base
        + flows
            .citer()
            .map(|(_, cost)| ordered_float::OrderedFloat(cost))
            .chain(once(ordered_float::OrderedFloat(0.0)))
            .min()
            .unwrap()
            .0;

    println!("{ans}");
}
