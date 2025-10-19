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

#[allow(dead_code)]
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
        pub from: u16,
        pub to: u16,
        pub label: W,
    }
    #[allow(dead_code)]
    impl<W> Edge<W>
    where
        W: Copy,
    {
        pub fn new(from: u16, to: u16) -> Self
        where
            W: Default,
        {
            Self {
                from,
                to,
                label: W::default(),
            }
        }
        pub fn new_with_label(from: u16, to: u16, label: W) -> Self {
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
    impl std::convert::From<(u16, u16)> for UnweightedEdge {
        fn from(t: (u16, u16)) -> Self {
            UnweightedEdge::new(t.0, t.1)
        }
    }
    impl std::convert::From<&(u16, u16)> for UnweightedEdge {
        fn from(t: &(u16, u16)) -> Self {
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
            self.out_edges[edge.from as usize].push(edge);
            self.in_edges[edge.to as usize].push(edge);
        }
    }
}

use detail::Graph;

#[allow(dead_code)]
fn dfs(g: &Graph, v: u16, visited: &mut Vec<bool>, vs: &mut Vec<u16>) {
    visited[v as usize] = true;

    for edge in &g.out_edges[v as usize] {
        if !visited[edge.to as usize] {
            dfs(g, edge.to, visited, vs);
        }
    }

    vs.push(v);
}

#[allow(dead_code)]
fn rev_dfs(
    g: &Graph,
    v: u16,
    idx: usize,
    idxs: &mut Vec<Option<usize>>,
    vs: &mut Vec<u16>,
    parents: &mut Vec<usize>,
) {
    idxs[v as usize] = Some(idx);
    vs.push(v);

    for edge in &g.in_edges[v as usize] {
        if let Some(t) = idxs[edge.from as usize] {
            if t < idx {
                parents.push(t);
            }
        } else {
            rev_dfs(g, edge.from, idx, idxs, vs, parents);
        }
    }
}

#[allow(dead_code)]
fn scc_in_degrees(g: &Graph) -> Vec<usize> {
    let mut vs = vec![];
    {
        let mut visited = vec![false; g.size()];
        for v in 0..g.size() {
            if !visited[v] {
                dfs(g, v as u16, &mut visited, &mut vs);
            }
        }
    }

    let mut in_degrees = vec![];
    {
        let mut idxs = vec![None; g.size()];
        for &v in vs.iter().rev() {
            if idxs[v as usize].is_none() {
                let idx = in_degrees.len();
                let mut component = vec![];
                let mut parents = vec![];
                rev_dfs(g, v, idx, &mut idxs, &mut component, &mut parents);
                in_degrees.push(parents.citer().unique().count());
            }
        }
    }

    in_degrees
}

fn main() {
    let n: usize = read();
    let f = read_vec(n, || read_tuple!(i64, i64));

    let m: usize = read();
    let s = read_vec(m, || read_tuple!(i64, i64));

    if m == 0 {
        println!("1");
        return;
    }

    const B: i64 = 100_000_000;

    let map = s
        .citer()
        .map(|(x, y)| ((x / B, y / B), (x, y)))
        // .inspect(|t| eprintln!("{:?}", t))
        .into_group_map();

    let edges = f
        .citer()
        .enumerate()
        .flat_map(|(i, (x, y))| {
            let d2 = (1..)
                .find_map(|j| {
                    let xx = x / B;
                    let yy = y / B;

                    iproduct!(xx - j..=xx + j, yy - j..=yy + j)
                        // .inspect(|t| eprintln!("{:?}", t))
                        .filter_map(|(xx2, yy2)| map.get(&(xx2, yy2)))
                        .flatten()
                        .copied()
                        .map(|(x2, y2)| (x - x2) * (x - x2) + (y - y2) * (y - y2))
                        .min()
                        .filter(|&d2| d2 <= j * j * B * B)
                })
                .unwrap();

            f.citer()
                .enumerate()
                .filter(move |&(j, _)| j != i)
                .filter(move |&(_, (x2, y2))| (x - x2) * (x - x2) + (y - y2) * (y - y2) < d2)
                .map(move |(j, _)| (i as u16, j as u16))
        })
        .collect_vec();

    let g = Graph::from_edges_directed(n, &edges);

    let in_degrees = scc_in_degrees(&g);

    let ans = in_degrees.citer().filter(|deg| *deg == 0).count();
    println!("{}", ans);
}
