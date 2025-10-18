fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let g = Graph::<usize>::from_edges_undirected(n, &abc);

    let from_src = dijkstra(&g, 0);
    let from_dst = dijkstra(&g, n - 1);

    let d = from_src[n - 1];

    let g1 = Graph::<usize>::from_edges_undirected(
        n,
        abc.iter()
            .enumerate()
            .filter(|&(i, &(a, b, c))| {
                if from_src[a] + c + from_dst[b] == d {
                    true
                } else if from_src[b] + c + from_dst[a] == d {
                    true
                } else {
                    false
                }
            })
            .map(|(i, &(a, b, _))| (a, b, i)),
    );

    let bridges = calc_lowlink(&g1);

    let ans = bridges.into_iter().fold(vec![false; m], |mut ans, i| {
        ans[i] = true;
        ans
    });

    for x in ans {
        if x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dijkstra(g: &Graph<usize>, src: usize) -> Vec<usize> {
    let n = g.size();
    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];
    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;
    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }
        for &edge in &g.out_edges[v] {
            let next_cost = cost + edge.label;
            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }
    costs
}

fn calc_lowlink(g: &Graph<usize>) -> Vec<usize> {
    let n = g.size();
    let mut ord = vec![0; n];
    let mut low = vec![0; n];
    let mut idx = 1;

    let mut stack = vec![];
    let mut bridges = vec![];

    for v0 in 0..n {
        if ord[v0] > 0 {
            continue;
        }

        stack.push((v0, n, usize::MAX, true));

        while let Some((v, p, edge_id, first)) = stack.pop() {
            if first {
                if ord[v] > 0 {
                    low[p] = min(low[p], ord[v]);
                    continue;
                }

                ord[v] = idx;
                low[v] = idx;
                idx += 1;

                stack.push((v, p, edge_id, false));
                g.out_edges[v].iter().for_each(|&e| {
                    if e.to == p {
                        return;
                    }
                    stack.push((e.to, v, e.label, true));
                });
            } else {
                if p < n {
                    low[p] = min(low[p], low[v]);
                }

                if p < n && ord[p] < low[v] {
                    bridges.push(edge_id);
                }
            }
        }
    }

    bridges
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
use detail::Graph;
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
