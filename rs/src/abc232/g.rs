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
    #[derive(Clone, Debug)]
    pub struct Graph {
        pub out_edges: Vec<(u32, u32)>,
        pub out_edges_idx: Vec<u32>,
    }
    #[allow(dead_code)]
    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                out_edges: vec![],
                out_edges_idx: vec![0; n + 1],
            }
        }
        pub fn from_edges_directed(n: usize, edges: &[(usize, usize, usize)]) -> Self {
            let mut g = Graph::new(n);
            for &(v, _u, _w) in edges {
                g.out_edges_idx[v] += 1;
            }
            for i in 0..n {
                g.out_edges_idx[i + 1] += g.out_edges_idx[i];
            }
            g.out_edges
                .resize_with(edges.len(), std::default::Default::default);
            for &(v, u, w) in edges {
                g.out_edges_idx[v] -= 1;
                g.out_edges[g.out_edges_idx[v] as usize] = (u as u32, w as u32);
            }
            g
        }
        pub fn size(&self) -> usize {
            self.out_edges_idx.len() - 1
        }
        pub fn out_adjs<'a>(&'a self, v: usize) -> impl 'a + Iterator<Item = (u32, u32)> {
            self.out_edges[self.out_edges_idx[v] as usize..self.out_edges_idx[v + 1] as usize]
                .iter()
                .copied()
        }
    }
}

use detail::Graph;

#[allow(dead_code)]
fn dijkstra1(g: &Graph, src: usize, dst: usize) -> Option<usize> {
    let n = g.size();
    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];
    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;
    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }
        if v == dst {
            return Some(cost);
        }
        for (u, w) in g.out_adjs(v) {
            let u = u as usize;
            let w = w as usize;
            let next_cost = cost + w;
            if next_cost < costs[u] {
                q.push(std::cmp::Reverse((next_cost, u)));
                costs[u] = next_cost;
            }
        }
    }
    None
}

fn main() {
    let (n, m) = read_tuple!(usize, u32);
    let a = read_row::<u32>();
    let b = read_row::<u32>();

    let mut a_idxs = (0..n as u32).collect::<Vec<_>>();
    a_idxs.sort_by_cached_key(|&i| a[i as usize]);
    let mut b_idxs = (0..n as u32).collect::<Vec<_>>();
    b_idxs.sort_by_cached_key(|&i| b[i as usize]);

    let mut adjs = vec![[0u32; 2]; n];

    a_idxs
        .citer()
        .rev()
        .scan(0, |j, i| {
            let i = i as usize;
            while *j < n && b[b_idxs[*j] as usize] < m - a[i] {
                *j += 1;
            }

            if *j == n {
                Some((i, b_idxs[0]))
            } else {
                Some((i, b_idxs[*j]))
            }
        })
        .for_each(|(i, j)| {
            adjs[i][0] = j;
        });
    b_idxs
        .citer()
        .chain(once(b_idxs[0]))
        .tuple_windows()
        .map(|(i0, i1)| (i0, i1))
        .for_each(|(i, j)| {
            adjs[i as usize][1] = j;
        });

    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::u32::MAX; n];
    let src = adjs[0][0];
    costs[src as usize] = (a[0] + b[src as usize]) % m;
    q.push(std::cmp::Reverse((costs[src as usize], src)));
    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v as usize] {
            continue;
        }
        if v as usize == n - 1 {
            println!("{}", cost);
            break;
        }

        {
            let u = adjs[v as usize][0];
            let next_cost = cost + (a[v as usize] + b[u as usize]) % m;
            if next_cost < costs[u as usize] && next_cost < costs[n - 1] {
                q.push(std::cmp::Reverse((next_cost, u)));
                costs[u as usize] = next_cost;
            }
        }
        {
            let u = adjs[v as usize][1];
            let next_cost = cost + (m + b[u as usize] - b[v as usize]) % m;
            if next_cost < costs[u as usize] && next_cost < costs[n - 1] {
                q.push(std::cmp::Reverse((next_cost, u)));
                costs[u as usize] = next_cost;
            }
        }
    }
}
