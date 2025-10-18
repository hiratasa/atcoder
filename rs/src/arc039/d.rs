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
        pub adjs_list: Vec<u32>,
        pub adjs_idxs: Vec<u32>,
    }
    #[allow(dead_code)]
    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                adjs_list: vec![],
                adjs_idxs: vec![0; n + 1],
            }
        }
        pub fn from_edges1_undirected(n: usize, edges: &[(u32, u32)]) -> Self {
            let mut g = Graph::new(n);
            for &(from, to) in edges {
                g.adjs_idxs[from as usize - 1] += 1;
                g.adjs_idxs[to as usize - 1] += 1;
            }
            for i in 1..=n {
                g.adjs_idxs[i] += g.adjs_idxs[i - 1];
            }
            g.adjs_list.resize(2 * edges.len(), 0);
            for &(from, to) in edges {
                let from = from as usize - 1;
                let to = to as usize - 1;
                g.adjs_idxs[from] -= 1;
                g.adjs_list[g.adjs_idxs[from] as usize] = to as u32;
                g.adjs_idxs[to] -= 1;
                g.adjs_list[g.adjs_idxs[to] as usize] = from as u32;
            }

            g
        }
        pub fn size(&self) -> usize {
            self.adjs_idxs.len() - 1
        }
        pub fn adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = usize> {
            let s = self.adjs_idxs[v] as usize;
            let e = self.adjs_idxs[v + 1] as usize;
            self.adjs_list[s..e].iter().copied().map(|u| u as usize)
        }
        pub fn children<'a>(&'a self, v: usize, p: usize) -> impl 'a + Iterator<Item = usize> {
            self.adjs(v).filter(move |&u| u != p)
        }
    }
}

use detail::Graph;

fn calc_lowlink(g: &Graph, components: &mut [u32], bridges: &mut Vec<(usize, usize)>) -> u32 {
    let n = g.size();
    let mut ord = vec![0; n];
    let mut low = vec![0; n];
    let mut idx = 1;
    let mut cidx = 0;

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

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(u32, u32); m],
        q: usize,
        abc: [(u32, u32, u32); q]
    }

    let g = Graph::from_edges1_undirected(n, &xy);

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
                components[a as usize - 1] as usize,
                components[b as usize - 1] as usize,
                components[c as usize - 1] as usize,
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
                if is_ancestor(b, c) { true } else { false }
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
