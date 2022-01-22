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

use std::io::BufWriter;
use std::io::Write;

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
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct Graph {
        pub out_edges: Vec<Vec<u32>>,
    }
    #[allow(dead_code)]
    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                out_edges: vec![vec![]; n],
            }
        }
        pub fn from_edges_directed<I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = (usize, usize)>,
        {
            let mut g = Graph::new(n);
            for (from, to) in edges {
                g.add_edge(from, to);
            }
            g
        }
        pub fn from_edges1_directed<I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = (usize, usize)>,
        {
            Graph::from_edges_directed(n, edges.into_iter().map(|(from, to)| (from - 1, to - 1)))
        }
        pub fn from_edges_undirected<I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = (usize, usize)>,
        {
            Graph::from_edges_directed(
                n,
                edges.into_iter().flat_map(|(from, to)| {
                    std::iter::once((from, to)).chain(std::iter::once((to, from)))
                }),
            )
        }
        pub fn from_edges1_undirected<I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = (usize, usize)>,
        {
            Graph::from_edges1_directed(
                n,
                edges.into_iter().flat_map(|(from, to)| {
                    std::iter::once((from, to)).chain(std::iter::once((to, from)))
                }),
            )
        }
        pub fn size(&self) -> usize {
            self.out_edges.len()
        }
        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.out_edges[from].push(to as u32);
        }
        pub fn adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = usize> {
            self.out_edges[v].iter().copied().map(|u| u as usize)
        }
        pub fn children<'a>(&'a self, v: usize, p: usize) -> impl 'a + Iterator<Item = usize> {
            self.adjs(v).filter(move |&u| u != p)
        }
    }
}

use detail::Graph;

fn calc_lowlink(g: &Graph, components: &mut [u32], bridges: &mut Vec<(usize, usize)>) {
    let n = g.size();
    let mut ord = vec![0; n];
    let mut low = vec![0; n];
    let mut idx = 1;
    let mut cidx = 0;

    let mut stack = vec![];
    let mut stack2 = vec![];
    stack.push((0, n, true));

    while let Some((v, p, first)) = stack.pop() {
        if first {
            ord[v] = idx;
            low[v] = idx;
            idx += 1;

            stack.push((v, p, false));
            stack2.push(v);
            g.adjs(v).for_each(|u| {
                if u == p {
                    return;
                }
                if ord[u] > 0 {
                    low[v] = min(low[v], ord[u]);
                } else {
                    stack.push((u, v, true));
                }
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

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
        q: usize,
        abc: [(usize, usize, usize); q]
    }

    let g = Graph::from_edges1_undirected(n, xy);

    let mut components = vec![0; n];
    let mut bridges = vec![];
    calc_lowlink(&g, &mut components, &mut bridges);

    let n1 = components.citer().max().unwrap() as usize + 1;

    const B: usize = 17;
    let mut parents = vec![vec![n1 as u32 - 1; n1]; B];
    for (p, v) in bridges {
        parents[0][components[v] as usize] = components[p];
    }
    for i in 1..B {
        for v in 0..n1 {
            parents[i][v] = parents[i - 1][parents[i - 1][v] as usize];
        }
    }
    let mut depth = vec![0u32; n1];
    for i in (0..n1 - 1).rev() {
        depth[i] = depth[parents[0][i] as usize] + 1;
    }

    let is_ancestor = |x: usize, y: usize| {
        if depth[x] > depth[y] {
            return false;
        }

        let y = (0..B).rev().fold(y, |yy, i| {
            if depth[parents[i][yy] as usize] < depth[x] {
                yy
            } else {
                parents[i][yy] as usize
            }
        });

        x == y
    };

    let lca = |x: usize, y: usize| {
        let (x, y) = if depth[x] < depth[y] { (x, y) } else { (y, x) };

        let y = (0..B).rev().fold(y, |yy, i| {
            if depth[parents[i][yy] as usize] < depth[x] {
                yy
            } else {
                parents[i][yy] as usize
            }
        });

        if x == y {
            return x;
        }

        let (x1, _y1) = (0..B).rev().fold((x, y), |(xx, yy), i| {
            if parents[i][xx] == parents[i][yy] {
                (xx, yy)
            } else {
                (parents[i][xx] as usize, parents[i][yy] as usize)
            }
        });

        parents[0][x1] as usize
    };

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    abc.citer()
        .map(|(a, b, c)| {
            (
                components[a - 1] as usize,
                components[b - 1] as usize,
                components[c - 1] as usize,
            )
        })
        .map(|(a, b, c)| {
            if is_ancestor(b, a) {
                if is_ancestor(b, c) {
                    lca(a, c) == b
                } else {
                    true
                }
            } else {
                if is_ancestor(b, c) {
                    true
                } else {
                    false
                }
            }
        })
        .for_each(|ans| {
            if ans {
                writeln!(stdout, "OK").unwrap();
            } else {
                writeln!(stdout, "NG").unwrap();
            }
        });
}
