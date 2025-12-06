fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let g = Graph::from_edges_undirected(n, uv);

    let mut cidxs = vec![0; n];
    let mut bridges = vec![];
    let k = calc_lowlink(&g, &mut cidxs, &mut bridges);

    let g2 = Graph::<(usize, usize)>::from_edges_directed(
        k,
        bridges
            .iter()
            .copied()
            .flat_map(|(u, v)| [(cidxs[u], cidxs[v], (u, v)), (cidxs[v], cidxs[u], (v, u))]),
    );

    let c0 = cidxs[0];
    let c1 = cidxs[1];
    let path = successors(Some((k, c0, (n, n))), |&(p, v, _)| {
        g2.children_edge(v, p).next().map(|e| (v, e.to, e.label))
    })
    .skip(1)
    .collect::<Vec<_>>();
    let is_ok = path.len() == k - 1 && (path.is_empty() || path[path.len() - 1].1 == c1);
    if !is_ok {
        println!("No");
        return;
    }

    let pairs: Vec<(usize, usize)> = once(0)
        .chain(path.iter().flat_map(|&(_, _, (v, u))| [v, u]))
        .chain(once(1))
        .tuples()
        .collect::<Vec<_>>();

    let mut edges0 = FxHashSet::default();
    let mut edges1 = FxHashSet::default();

    let mut visited = vec![false; n];
    for &(v, _) in &pairs {
        dfs(&g, v, &mut visited, &cidxs, &mut edges0, &edges1);
    }

    let mut visited = vec![false; n];
    for &(_, v) in &pairs {
        dfs(&g, v, &mut visited, &cidxs, &mut edges1, &edges0);
    }

    for &(_, _, (v, u)) in &path {
        edges0.insert((u, v));
        edges1.insert((v, u));
    }

    assert_eq!(edges0.len(), n - 1);
    assert_eq!(edges1.len(), n - 1);

    let ans0 = edges0.iter().fold(vec![n; n], |mut ans0, &(u, v)| {
        ans0[u] = v;
        ans0
    });

    let ans1 = edges1.iter().fold(vec![n; n], |mut ans1, &(u, v)| {
        ans1[u] = v;
        ans1
    });

    println!("Yes");
    println!("{}", ans1[0] + 1);
    println!("{}", ans0[1] + 1);
    for i in 2..n {
        println!("{} {}", ans0[i] + 1, ans1[i] + 1);
    }
}

fn dfs(
    g: &Graph,
    v: usize,
    visited: &mut [bool],
    cidxs: &[usize],
    used: &mut FxHashSet<(usize, usize)>,
    used2: &FxHashSet<(usize, usize)>,
) -> bool {
    if visited[v] {
        return false;
    }
    visited[v] = true;

    for u in g.adjs(v) {
        if cidxs[u] != cidxs[v] {
            continue;
        }
        if used.contains(&(u, v)) || used2.contains(&(u, v)) {
            continue;
        }
        if dfs(g, u, visited, cidxs, used, used2) {
            used.insert((u, v));
        }
    }

    true
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
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

fn calc_lowlink(g: &Graph, components: &mut [usize], bridges: &mut Vec<(usize, usize)>) -> usize {
    let n = g.size();
    let mut ord = vec![0; n];
    let mut low = vec![0; n];
    let mut idx = 1usize;
    let mut cidx = 0usize;

    let mut stack = vec![];
    let mut stack2 = vec![];

    for v0 in 0..n {
        if ord[v0] > 0 {
            continue;
        }

        stack.push((v0, n, true));

        while let Some((v, p, first)) = stack.pop() {
            if first {
                if ord[v] > 0 {
                    low[p] = min(low[p], ord[v]);
                    continue;
                }

                ord[v] = idx;
                low[v] = idx;
                idx += 1;

                stack.push((v, p, false));
                stack2.push(v);
                let mut p_first = true;
                g.adjs(v).for_each(|u| {
                    // 多重辺があるときのために、pは初回だけ無視
                    if u == p && p_first {
                        p_first = false;
                        return;
                    }
                    stack.push((u, v, true));
                });
            } else {
                if p < n {
                    low[p] = min(low[p], low[v]);
                }

                if p >= n || ord[p] < low[v] {
                    // bridge
                    while let Some(x) = stack2.pop() {
                        components[x] = cidx;
                        if x == v {
                            break;
                        }
                    }
                    cidx += 1;
                    if p < n {
                        bridges.push((p, v));
                    }
                }
            }
        }
    }

    cidx
}
