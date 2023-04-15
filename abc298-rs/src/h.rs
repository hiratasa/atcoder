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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

type Graph = detail::UnweightedGraph;

fn dfs(
    g: &Graph,
    v: usize,
    p: usize,
    parents: &mut [usize],
    depth: &mut [usize],
    sizes: &mut [usize],
    sums: &mut [usize],
) -> (usize, usize) {
    let (size, sum) = g
        .children_edge(v, p)
        .map(|e| {
            let u = e.to;
            parents[u] = v;
            depth[u] = depth[v] + 1;
            dfs(g, u, v, parents, depth, sizes, sums)
        })
        .fold((1, 0), |(size, sum), (size1, sum1)| {
            (size + size1, sum + sum1 + size1)
        });

    sizes[v] = size;
    sums[v] = sum;

    (size, sum)
}

fn dfs2(
    g: &Graph,
    v: usize,
    p: usize,
    sizes: &[usize],
    sums: &[usize],
    sums0: &mut [usize],
    carry_size: usize,
    carry_sum: usize,
) {
    sums0[v] = sums[v] + carry_sum;

    let (sz, sm) = g.children(v, p).map(|u| (sizes[u], sums[u])).fold(
        (carry_size + 1, carry_sum + carry_size + 1),
        |(sz, sm), (sz1, sm1)| (sz + sz1, sm + sm1 + 2 * sz1),
    );

    g.children(v, p).for_each(|u| {
        dfs2(
            g,
            u,
            v,
            sizes,
            sums,
            sums0,
            sz - sizes[u],
            sm - sums[u] - 2 * sizes[u],
        );
    });
}

fn lca(v: usize, u: usize, parents: &[Vec<usize>], depth: &[usize]) -> usize {
    let (mut v, mut u) = if depth[v] < depth[u] { (v, u) } else { (u, v) };

    for i in (0..20).rev() {
        if depth[parents[i][u]] >= depth[v] {
            u = parents[i][u];
        }
    }

    assert!(depth[v] == depth[u]);

    if v == u {
        return v;
    }

    for i in (0..20).rev() {
        if parents[i][v] != parents[i][u] {
            v = parents[i][v];
            u = parents[i][u];
        }
    }

    parents[0][v]
}

fn nth_parents(v: usize, parents: &[Vec<usize>], nth: usize) -> usize {
    (0..20)
        .filter(|&i| nth & (1 << i) > 0)
        .fold(v, |p, i| parents[i][p])
}

fn main() {
    let n = read::<usize>();
    let ab = read_vec(n - 1, || read_tuple!(usize, usize));
    let q = read::<usize>();
    let lr = read_vec(q, || read_tuple!(usize, usize));

    let g = Graph::from_edges1_undirected(n, ab);

    let mut parents = vec![vec![0; n]; 20];
    let mut depth = vec![0; n];
    let mut sizes = vec![0; n];
    let mut sums = vec![0; n];
    dfs(&g, 0, n, &mut parents[0], &mut depth, &mut sizes, &mut sums);

    for i in 1..20 {
        for v in 0..n {
            parents[i][v] = parents[i - 1][parents[i - 1][v]];
        }
    }

    let mut sums0 = vec![0; n];
    dfs2(&g, 0, n, &sizes, &sums, &mut sums0, 0, 0);

    lr.citer()
        .map(|(l, r)| {
            let l = l - 1;
            let r = r - 1;

            if l == r {
                return sums0[l];
            }

            let p = lca(l, r, &parents, &depth);

            let d0 = depth[l] - depth[p];
            let d1 = depth[r] - depth[p];
            let d = d0 + d1;

            let (l, r) = if d0 < d1 { (l, r) } else { (r, l) };

            let c0 = nth_parents(r, &parents, (d + 1) / 2);
            let c1 = nth_parents(r, &parents, (d - 1) / 2);
            assert!(c0 == parents[0][c1]);

            let d_rc0 = (d + 1) / 2;
            let d_lc1 = d - (d - 1) / 2;

            if l == p {
                let adj0 = nth_parents(r, &parents, d - 1);

                let sum_l = sums0[l] - (sums[adj0] + sizes[adj0]);
                let sum_r = sums[r];

                let sum_mid_l = sums[adj0] + sizes[adj0] - (sums[c1] + sizes[c1] * d_lc1);
                let sum_mid_r = sums0[r]
                    - sums[r]
                    - (sums0[c0] - (sums[c1] + sizes[c1]) + (n - sizes[c1]) * d_rc0);

                sum_l + sum_r + sum_mid_l + sum_mid_r
            } else {
                let sum_l = sums[l];
                let sum_r = sums[r];

                let sum_mid_l = sums0[l] - sums[l] - (sums[c1] + sizes[c1] * d_lc1);
                let sum_mid_r = sums0[r]
                    - sums[r]
                    - (sums0[c0] - (sums[c1] + sizes[c1]) + (n - sizes[c1]) * d_rc0);

                sum_l + sum_r + sum_mid_l + sum_mid_r
            }
        })
        .for_each(|ans| {
            println!("{}", ans);
        });
}
