fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
        uv: [(Usize1, Usize1); n - 1],
    };

    let g = Graph::from_edges_undirected(n, uv);
    let mut dp = vec![[[i64::MIN; 3]; 6]; n];
    dfs(&g, 0, n, &a, &mut dp);

    // eprintln!("{dp:?}");

    println!(
        "{}",
        iproduct!(0..=k, 0..3)
            .map(|(kk, i)| dp[0][kk][i])
            .max()
            .unwrap()
    );
}

fn dfs(g: &Graph, v: usize, p: usize, a: &[i64], dp: &mut [[[i64; 3]; 6]]) {
    g.children(v, p).for_each(|u| dfs(g, u, v, a, dp));

    dp[v][0][0] = 0;
    dp[v][1][1] = a[v];
    for u in g.children(v, p) {
        let mut t = dp[v];
        for k in 0..=5 {
            for k0 in 0..=k {
                let k1 = k - k0;

                t[k][0] = max(
                    t[k][0],
                    dp[v][k0][0].saturating_add(max(dp[u][k1][0], max(dp[u][k1][1], dp[u][k1][2]))),
                );
                t[k][1] = max(
                    t[k][1],
                    max(
                        dp[v][k0][0].saturating_add(dp[u][k1][1] + a[v]),
                        dp[v][k0][1]
                            .saturating_add(max(dp[u][k1][0], max(dp[u][k1][1], dp[u][k1][2]))),
                    ),
                );
                t[k][2] = max(
                    t[k][2],
                    max(
                        dp[v][k0][1].saturating_add(dp[u][k1][0] + a[u]),
                        dp[v][k0][2]
                            .saturating_add(max(dp[u][k1][0], max(dp[u][k1][1], dp[u][k1][2]))),
                    ),
                );

                if k1 > 0 {
                    t[k][1] = max(
                        t[k][1],
                        dp[v][k0][0].saturating_add(dp[u][k1 - 1][0] + a[v] + a[u]),
                    );
                }
                if k1 + 1 <= 5 {
                    t[k][2] = max(t[k][2], dp[v][k0][1].saturating_add(dp[u][k1 + 1][1]));
                }
            }
        }
        dp[v] = t;
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

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
    impl std::convert::From<(usize, usize)> for Edge<()> {
        fn from(t: (usize, usize)) -> Self {
            Edge::new(t.0, t.1)
        }
    }
    impl std::convert::From<&(usize, usize)> for Edge<()> {
        fn from(t: &(usize, usize)) -> Self {
            Edge::from(*t)
        }
    }
    impl<W> std::convert::From<(usize, usize, W)> for Edge<W>
    where
        W: Copy,
    {
        fn from(t: (usize, usize, W)) -> Self {
            Edge::new_with_label(t.0, t.1, t.2)
        }
    }
    impl<W> std::convert::From<&(usize, usize, W)> for Edge<W>
    where
        W: Copy,
    {
        fn from(t: &(usize, usize, W)) -> Self {
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
use detail::Graph;
