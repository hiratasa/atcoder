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

type Graph = detail::WeightedGraph;

// sizesは重心を根とした部分木だけ正確な値が入ってる
fn find_centroid(
    g: &Graph,
    v: usize,
    p: usize,
    used: &[bool],
    sizes: &mut [usize],
    n: usize,
) -> Option<usize> {
    sizes[v] = 1;
    g.children(v, p)
        .filter(|&u| !used[u])
        .filter_map(|u| {
            let c = find_centroid(g, u, v, used, sizes, n);
            sizes[v] += sizes[u];
            c
        })
        .next()
        .or_else(|| {
            if 2 * (n - sizes[v]) <= n {
                Some(v)
            } else {
                None
            }
        })
}

fn calc_distances(g: &Graph, v: usize, p: usize, used: &[bool], d: usize, dists: &mut Vec<usize>) {
    dists.push(d);

    g.children_edge(v, p)
        .filter(|&e| !used[e.to])
        .for_each(|e| calc_distances(g, e.to, v, used, d + e.label, dists))
}

// 和がx以下になる組み合わせの数, x以下の距離の和
fn solve(
    g: &Graph,
    v: usize,
    x: usize,
    n: usize,
    used: &mut [bool],
    sizes: &mut [usize],
) -> (usize, usize) {
    let c = find_centroid(g, v, usize::MAX, used, sizes, n).unwrap();

    used[c] = true;

    let distss = g.out_edges[c]
        .citer()
        .filter(|&e| !used[e.to])
        .map(|e| {
            let mut dists = vec![];
            calc_distances(g, e.to, c, used, e.label, &mut dists);
            dists.sort();
            dists
        })
        .collect::<Vec<_>>();

    let calcself = |dists: &[usize]| {
        dists
            .citer()
            .filter(|&d| d <= x)
            .fold((0, 0), |(k0, s0), d| (k0 + 1, s0 + d))
    };

    let calc = |dists: &[usize]| {
        let sums = once(0)
            .chain(dists.citer())
            .cumsum::<usize>()
            .collect::<Vec<_>>();

        dists
            .citer()
            .scan(dists.len(), |i, d| {
                while *i > 0 && dists[*i - 1] + d > x {
                    *i -= 1;
                }

                Some((*i, *i * d + sums[*i]))
            })
            .fold((0, 0), |(k0, s0), (k, s)| (k0 + k, s0 + s))
    };

    let (k, s) = distss
        .iter()
        .map(|dists| calc(dists))
        .fold((0, 0), |(k0, s0), (k, s)| (k0 + k, s0 + s));

    let all_dists = distss.iter().kmerge().copied().collect::<Vec<_>>();
    let (k0, s0) = calcself(&all_dists);
    let (k1, s1) = calc(&all_dists);

    g.adjs(c)
        .map(|u| {
            let nn = if sizes[u] > sizes[c] {
                n - sizes[c]
            } else {
                sizes[u]
            };
            if !used[u] {
                solve(g, u, x, nn, used, sizes)
            } else {
                (0, 0)
            }
        })
        .fold(
            (k0 + (k1 - k) / 2, s0 + (s1 - s) / 2),
            |(k0, s0), (k, s)| (k0 + k, s0 + s),
        )
}

fn main() {
    let (n, x) = read_tuple!(usize, usize);
    let abc = read_vec(n - 1, || read_tuple!(usize, usize, usize));

    let g = Graph::from_edges1_undirected(n, &abc);

    let (k, s) = solve(&g, 0, x, n, &mut vec![false; n], &mut vec![0; n]);

    let ans0 = (n * (n - 1) / 2 - k) * x + s;

    let nearest = abc
        .citer()
        .fold(vec![usize::MAX; n], |mut nearest, (a, b, c)| {
            nearest[a - 1] = min(nearest[a - 1], c);
            nearest[b - 1] = min(nearest[b - 1], c);
            nearest
        });
    let ans = ans0 - abc.citer().map(|(_, _, c)| min(c, x)).sum::<usize>()
        + abc
            .citer()
            .map(|(a, b, c)| {
                let y = match (
                    g.out_edges[a - 1].len() > 1,
                    g.out_edges[b - 1].len() > 1,
                    g.out_edges[a - 1].len() + g.out_edges[b - 1].len() < n,
                ) {
                    (_, _, true) => 2 * x,
                    (true, true, _) => 3 * x,
                    _ => usize::MAX,
                };
                min(y, min(c, x + min(nearest[a - 1], nearest[b - 1])))
            })
            .sum::<usize>();

    println!("{}", ans);
}
