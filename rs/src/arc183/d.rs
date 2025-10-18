fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let g = Graph::from_edges_undirected(n, ab);

    let mut sizes = vec![0; n];
    calc_sizes(&g, 0, n, &mut sizes);
    let c = find_centroid(&g, 0, n, &sizes).unwrap();

    // cを根として再計算
    let mut sizes = vec![0; n];
    calc_sizes(&g, c, n, &mut sizes);

    let subs = g
        .adjs(c)
        .map(|v| {
            let mut idxs = vec![];
            dfs(&g, v, c, &sizes, &mut idxs);
            idxs
        })
        .collect::<Vec<_>>();

    #[derive(Debug, Clone)]
    struct BySize(Vec<usize>);
    impl PartialOrd for BySize {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.len().partial_cmp(&other.0.len())
        }
    }
    impl Ord for BySize {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    impl PartialEq for BySize {
        fn eq(&self, other: &Self) -> bool {
            self.0.len() == other.0.len()
        }
    }
    impl Eq for BySize {}

    let mut odd = subs.iter().find(|sub| sub.len() % 2 > 0).unwrap().clone();
    let mut evens = subs
        .into_iter()
        .filter(|sub| sub.len() % 2 == 0)
        .map(|sub| BySize(sub))
        .collect::<BinaryHeap<_>>();

    let mut ans = vec![];
    while ans.len() < n / 2 {
        let x = odd.pop().unwrap();

        if let Some(BySize(mut sub)) = evens.pop() {
            let y = sub.pop().unwrap();

            ans.push((x, y));

            if !odd.is_empty() {
                evens.push(BySize(odd));
            }
            odd = sub;
        } else {
            ans.push((c, x));

            assert!(odd.is_empty());
            assert!(ans.len() == n / 2);
        }
    }

    for (x, y) in ans {
        println!("{} {}", x + 1, y + 1);
    }
}

fn calc_sizes(g: &Graph, v: usize, p: usize, sizes: &mut [usize]) -> usize {
    sizes[v] = 1 + g
        .children(v, p)
        .map(|u| calc_sizes(g, u, v, sizes))
        .sum::<usize>();

    sizes[v]
}

fn find_centroid(g: &Graph, v: usize, p: usize, sizes: &[usize]) -> Option<usize> {
    let n = g.size();

    g.children(v, p)
        .find_map(|u| {
            let c = find_centroid(g, u, v, sizes);
            c
        })
        .or_else(|| {
            if 2 * (n - sizes[v]) <= n {
                Some(v)
            } else {
                None
            }
        })
}

fn dfs(g: &Graph, v: usize, p: usize, sizes: &[usize], idxs: &mut Vec<usize>) {
    idxs.push(v);

    g.children(v, p)
        .sorted_by_key(|&u| sizes[u] % 2)
        .for_each(|u| dfs(g, u, v, sizes, idxs));
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
