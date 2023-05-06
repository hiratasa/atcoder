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

fn dfs_grid(c: &[Vec<char>], i: usize, j: usize, idx: u32, idxs: &mut [[u32; 16]]) {
    let h = c.len();
    let w = c[0].len();

    if idxs[i][j] != std::u32::MAX {
        return;
    }

    idxs[i][j] = idx;

    it![(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)]
        .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(|&(ni, nj)| ni < h && nj < w)
        .filter(|&(ni, nj)| c[i][j] == c[ni][nj])
        .for_each(|(ni, nj)| dfs_grid(c, ni, nj, idx, idxs))
}

mod detail {
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct Graph {
        pub out_edges: Vec<u32>,
        pub out_edges_idx: Vec<u32>,
        pub in_edges: Vec<u32>,
        pub in_edges_idx: Vec<u32>,
    }
    #[allow(dead_code)]
    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                out_edges: vec![],
                out_edges_idx: vec![0; n + 1],
                in_edges: vec![],
                in_edges_idx: vec![0; n + 1],
            }
        }
        pub fn from_edges_directed(n: usize, edges: Vec<(usize, usize)>) -> Self {
            let mut g = Graph::new(n);
            for &(v, u) in &edges {
                g.out_edges_idx[v] += 1;
                g.in_edges_idx[u] += 1;
            }
            for i in 0..n {
                g.out_edges_idx[i + 1] += g.out_edges_idx[i];
                g.in_edges_idx[i + 1] += g.in_edges_idx[i];
            }
            g.out_edges.resize(edges.len(), 0);
            g.in_edges.resize(edges.len(), 0);
            for &(v, u) in &edges {
                g.out_edges_idx[v] -= 1;
                g.out_edges[g.out_edges_idx[v] as usize] = u as u32;
                g.in_edges_idx[u] -= 1;
                g.in_edges[g.in_edges_idx[u] as usize] = v as u32;
            }
            g
        }
        pub fn size(&self) -> usize {
            self.out_edges_idx.len() - 1
        }
        pub fn out_adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = usize> {
            self.out_edges[self.out_edges_idx[v] as usize..self.out_edges_idx[v + 1] as usize]
                .iter()
                .map(|&u| u as usize)
        }
        pub fn in_adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = usize> {
            self.in_edges[self.in_edges_idx[v] as usize..self.in_edges_idx[v + 1] as usize]
                .iter()
                .map(|&u| u as usize)
        }
    }
}

use detail::*;

#[allow(dead_code)]
fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;

    for u in g.out_adjs(v) {
        if !visited[u] {
            dfs(g, u, visited, vs);
        }
    }

    vs.push(v);
}

#[allow(dead_code)]
fn rev_dfs(
    g: &Graph,
    v: usize,
    idx: u32,
    idxs: &mut Vec<u32>,
    vs: &mut Vec<usize>,
    edges: &mut Vec<(usize, usize)>,
) {
    idxs[v] = idx;
    vs.push(v);

    for u in g.in_adjs(v) {
        let idx2 = idxs[u];
        if idx2 == std::u32::MAX {
            rev_dfs(g, u, idx, idxs, vs, edges);
        } else if idx2 != idx {
            edges.push((idx2 as usize, idx as usize));
        }
    }
}

#[allow(dead_code)]
fn scc(g: &Graph) -> (Vec<Vec<usize>>, Vec<u32>, Graph) {
    let mut vs = vec![];
    {
        let mut visited = vec![false; g.size()];
        for v in 0..g.size() {
            if !visited[v] {
                dfs(g, v, &mut visited, &mut vs);
            }
        }
    }

    let mut components = vec![];
    let mut idxs = vec![std::u32::MAX; g.size()];
    let mut edges = vec![];
    {
        for &v in vs.iter().rev() {
            if idxs[v] == std::u32::MAX {
                let mut component = vec![];
                rev_dfs(
                    g,
                    v,
                    components.len() as u32,
                    &mut idxs,
                    &mut component,
                    &mut edges,
                );
                components.push(component);
            }
        }
    }

    let k = components.len();
    (
        components,
        idxs,
        Graph::from_edges_directed(k, edges.citer().sorted().dedup().collect()),
    )
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let c = read_vec(h, || read_str());
    let q = read::<usize>();
    let query = read_vec(q, || read_tuple!(usize, usize));

    let mut idx = 0;
    let mut idxs = vec![[std::u32::MAX; 16]; h];
    for i in 0..h {
        for j in 0..w {
            if idxs[i][j] == std::u32::MAX {
                dfs_grid(&c, i, j, idx, &mut idxs);
                idx += 1;
            }
        }
    }

    let g0 = Graph::from_edges_directed(
        idx as usize,
        iproduct!(0..h - 1, 0..w)
            .filter(|&(i, j)| c[i][j] != c[i + 1][j])
            .map(|(i, j)| (idxs[i][j] as usize, idxs[i + 1][j] as usize))
            .sorted()
            .dedup()
            .collect(),
    );

    let (components, cidxs, g1) = scc(&g0);

    let mut idxs = idxs;
    for i in 0..h {
        for j in 0..w {
            idxs[i][j] = cidxs[idxs[i][j] as usize];
        }
    }

    let k = components.len();
    let mut topmosts0 =
        iproduct!(0..h, 0..w).fold(vec![[std::u32::MAX; 16]; k], |mut topmosts, (i, j)| {
            let cidx = idxs[i][j] as usize;
            topmosts[cidx][j] = min(topmosts[cidx][j], i as u32);
            topmosts
        });

    let (_topmosts, sums) = (0..k).rev().fold(
        (vec![[0; 16]; k], vec![0; k]),
        |(mut topmosts, mut sums): (Vec<[u32; 16]>, Vec<u32>), cidx| {
            topmosts[cidx] = g1
                .out_adjs(cidx)
                .fold(take(&mut topmosts0[cidx]), |mut t, cidx2| {
                    izip!(t.iter_mut(), topmosts[cidx2].citer()).for_each(|(x, y)| *x = min(*x, y));
                    t
                });
            sums[cidx] = (0..w)
                .map(|jj| topmosts[cidx][jj])
                .filter(|&ii| ii != std::u32::MAX)
                .map(|ii| h as u32 - ii)
                .sum::<u32>();
            (topmosts, sums)
        },
    );

    let sums1 = iproduct!(0..h, 0..w).fold(vec![0; h * w], |mut sums1, (i, j)| {
        sums1[i * w + j] = sums[idxs[i][j] as usize];
        sums1
    });

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    query
        .citer()
        .map(|(j, i)| (j - 1, i - 1))
        .map(|(j, i)| sums1[i * w + j])
        .for_each(|ans| {
            writeln!(stdout, "{}", ans).unwrap();
        });
}
