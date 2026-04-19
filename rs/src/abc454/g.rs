fn main() {
    input! {
        n: usize, seed: usize, m: usize, f: usize,
        q: [Usize1; m - 1],
        d: [usize; m],
    };

    let mut state = seed;
    let mut g = Graph::from_edges_directed(
        n,
        q.iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i + 1))
            .chain((m..n).map(|i| {
                let p = state % i;
                state = (state * 1103515245 + 12345) % (1 << 31);
                (p, i)
            })),
    );
    let c = d
        .iter()
        .copied()
        .chain((m..n).map(|i| {
            let c = state % f + 1;
            state = (state * 1103515245 + 12345) % (1 << 31);
            c
        }))
        .collect::<Vec<_>>();

    let mut sizes = vec![0; n];
    calc_sizes(&g, 0, &mut sizes);
    for v in 0..n {
        g.out_edges[v].sort_by_key(|e| Reverse(sizes[e.to]));
    }

    let mut ans = vec![(0, 0); n];
    dfs(&g, 0, &c, &mut vec![0; n + 1], &mut vec![], &mut ans);

    let a = ans
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (m, k))| (m ^ (i + 1)) * (k ^ (i + 1)))
        .fold(0, |x, y| (x + y) % 998244353);

    println!("{a}");
}

fn calc_sizes(g: &Graph, v: usize, sizes: &mut [usize]) -> usize {
    sizes[v] = 1 + g.out_edges[v]
        .iter()
        .copied()
        .map(|e| calc_sizes(g, e.to, sizes))
        .sum::<usize>();

    sizes[v]
}

fn dfs(
    g: &Graph,
    v: usize,
    c: &[usize],
    nums: &mut [usize],
    buf: &mut Vec<(usize, usize)>,
    ans: &mut [(usize, usize)],
) -> (usize, usize) {
    let l = g.out_edges[v].len();
    let b = buf.len();

    ans[v] = if l == 0 {
        nums[c[v]] += 1;
        (1, 1)
    } else {
        buf.push((c[v], nums[c[v]]));
        nums[c[v]] = 0;
        g.out_edges[v][1..]
            .iter()
            .for_each(|&e| dfs_pre(g, e.to, c, nums, buf));

        let (mut m, mut k) = dfs(g, g.out_edges[v][0].to, c, nums, buf, ans);
        for &(x, _) in &buf[b..] {
            nums[x] += 1;
            if nums[x] > m {
                m = nums[x];
                k = 1;
            } else if nums[x] == m {
                k += 1;
            }
        }
        for (x, p) in buf.drain(b..) {
            nums[x] += p;
            nums[x] -= 1;
        }

        nums[c[v]] += 1;
        g.out_edges[v][1..].iter().for_each(|e| {
            dfs(g, e.to, c, nums, buf, ans);
        });

        (m, k)
    };

    ans[v]
}

fn dfs_pre(g: &Graph, v: usize, c: &[usize], nums: &mut [usize], buf: &mut Vec<(usize, usize)>) {
    buf.push((c[v], nums[c[v]]));
    nums[c[v]] = 0;

    g.out_edges[v]
        .iter()
        .for_each(|&e| dfs_pre(g, e.to, c, nums, buf));
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
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
