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

#[allow(dead_code)]
struct MaxFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
}

#[allow(dead_code)]
impl MaxFlowGraph {
    fn new(n: usize) -> MaxFlowGraph {
        MaxFlowGraph {
            g: Graph::new(n),
            caps: vec![],
            rev: vec![],
        }
    }

    fn num_vertices(&self) -> usize {
        self.g.size()
    }

    fn num_edges(&self) -> usize {
        self.caps.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize) -> usize {
        let idx = self.num_edges();
        let rev_idx = self.num_edges() + 1;

        self.g.add_edge(Edge::new_with_label(from, to, idx));
        self.g.add_edge(Edge::new_with_label(to, from, rev_idx));

        // forward edge
        self.caps.push(cap);
        self.rev.push(rev_idx);

        // backward edge
        self.caps.push(0);
        self.rev.push(idx);

        idx
    }

    fn bfs(&self, src: usize, dst: usize) -> Option<Vec<usize>> {
        fn chmin(a: &mut usize, b: usize) -> bool {
            if *a > b {
                *a = b;
                true
            } else {
                false
            }
        }

        let mut q = std::collections::VecDeque::new();
        let mut costs = vec![std::usize::MAX; self.num_vertices()];

        q.push_back(src);
        costs[src] = 0;

        while let Some(v) = q.pop_front() {
            if v == dst {
                return Some(costs);
            }

            let c = costs[v];
            self.g.out_edges[v]
                .iter()
                .filter(|e| self.caps[e.label] > 0)
                .filter(|e| chmin(&mut costs[e.to], c + 1))
                .for_each(|e| q.push_back(e.to));
        }

        None
    }

    fn dfs(
        &mut self,
        src: usize,
        dst: usize,
        upper: usize,
        levels: &Vec<usize>,
        itrs: &mut Vec<usize>,
    ) -> usize {
        if src == dst {
            return upper;
        }

        let mut total_flow = 0;
        for i in itrs[src]..self.g.out_edges[src].len() {
            let e = self.g.out_edges[src][i];
            if levels[src] + 1 == levels[e.to] && self.caps[e.label] > 0 {
                let flow = self.dfs(
                    e.to,
                    dst,
                    (upper - total_flow).min(self.caps[e.label]),
                    levels,
                    itrs,
                );

                self.caps[e.label] -= flow;
                self.caps[self.rev[e.label]] += flow;

                total_flow += flow;
                if upper == total_flow {
                    // NOTE:
                    //  この場合はitrs[src]はインクリメントしないこと！
                    //  (この辺に沿ってまだ流せるかもしれない)
                    return total_flow;
                }
            }
            itrs[src] += 1;
        }

        total_flow
    }

    fn max_flow(&mut self, src: usize, dst: usize) -> usize {
        let mut total_flow = 0;
        loop {
            if let Some(levels) = self.bfs(src, dst) {
                // ここでは一回のdfsで流せるだけ流しきる方式を採用しているので、
                // 複数回呼ぶ必要はない
                total_flow += self.dfs(
                    src,
                    dst,
                    std::usize::MAX,
                    &levels,
                    &mut vec![0; self.num_vertices()],
                );
            } else {
                break;
            }
        }

        total_flow
    }

    // max_flow()の後に呼ぶ
    fn min_cut(&self, src: usize, _dst: usize) -> Vec<(usize, usize)> {
        let mut q = vec![];
        let mut visited = vec![false; self.num_vertices()];

        q.push(src);
        visited[src] = true;

        while let Some(v) = q.pop() {
            self.g.out_edges[v]
                .iter()
                .filter(|e| self.caps[e.label] > 0)
                .filter(|e| !replace(&mut visited[e.to], true))
                .for_each(|e| q.push(e.to));
        }

        (0..self.num_vertices())
            .flat_map(|i| {
                self.g.out_edges[i]
                    .citer()
                    .filter(|&e| e.label % 2 == 0)
                    .filter(|&e| visited[e.from] && !visited[e.to])
                    .map(|e| (e.from, e.to))
            })
            .collect()
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let ab = read_vec(m, || read_tuple!(usize, usize));
    let c = read_row::<usize>();

    let mut g = MaxFlowGraph::new(2 * n);
    for i in 0..n {
        if i == 0 || i == n - 1 {
            g.add_edge(2 * i, 2 * i + 1, usize::MAX);
        } else {
            g.add_edge(2 * i, 2 * i + 1, c[i]);
        }
    }

    for (a, b) in ab {
        let a = a - 1;
        let b = b - 1;
        g.add_edge(2 * a + 1, 2 * b, usize::MAX);
        g.add_edge(2 * b + 1, 2 * a, usize::MAX);
    }

    let ans = g.max_flow(0, 2 * n - 1);
    let cut = g.min_cut(0, 2 * n - 1);

    println!("{}", ans);
    println!("{}", cut.len());
    println!("{}", cut.citer().map(|(i, _)| i / 2 + 1).join(" "));
}
