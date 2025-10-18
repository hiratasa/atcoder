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
    }
}

// 強連結成分分解
mod scc {
    use super::detail::*;

    #[allow(dead_code)]
    fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
        visited[v] = true;

        for edge in &g.out_edges[v] {
            if !visited[edge.to] {
                dfs(g, edge.to, visited, vs);
            }
        }

        vs.push(v);
    }

    #[allow(dead_code)]
    fn rev_dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
        visited[v] = true;
        vs.push(v);

        for edge in &g.in_edges[v] {
            if !visited[edge.from] {
                rev_dfs(g, edge.from, visited, vs);
            }
        }
    }

    #[allow(dead_code)]
    fn scc(g: &Graph) -> Vec<Vec<usize>> {
        let mut vs = vec![];
        {
            let mut visited = vec![false; g.size()];
            for v in 0..g.size() {
                if !visited[v] {
                    dfs(g, v, &mut visited, &mut vs);
                }
            }
        }

        let mut ret = vec![];
        {
            let mut visited = vec![false; g.size()];
            for &v in vs.iter().rev() {
                if !visited[v] {
                    let mut component = vec![];
                    rev_dfs(g, v, &mut visited, &mut component);
                    ret.push(component);
                }
            }
        }

        ret
    }

    // 2-sat
    #[allow(dead_code)]
    pub struct TwoSat {
        g: Graph,
    }

    impl TwoSat {
        #[allow(dead_code)]
        pub fn new(n: usize) -> TwoSat {
            TwoSat {
                g: Graph::new(2 * n),
            }
        }

        // size()より小さいとfalse, size()以上だとtrue
        #[allow(dead_code)]
        pub fn to_v(&self, a: usize, f: bool) -> usize {
            if f { a + self.size() } else { a }
        }

        #[allow(dead_code)]
        pub fn add(&mut self, a: usize, fa: bool, b: usize, fb: bool) {
            self.g.add_edge((self.to_v(a, !fa), self.to_v(b, fb)));
            self.g.add_edge((self.to_v(b, !fb), self.to_v(a, fa)));
        }

        #[allow(dead_code)]
        pub fn size(&self) -> usize {
            self.g.size() / 2
        }

        #[allow(dead_code)]
        pub fn solve(&self) -> Option<Vec<bool>> {
            let components = scc(&self.g);

            let mut ret = vec![false; self.size()];
            let mut idx = vec![components.len(); self.size()];
            for i in 0..components.len() {
                for &v in &components[i] {
                    let t = v % self.size();

                    if idx[t] == i {
                        // negation is already appeared in same component.
                        return None;
                    }

                    if idx[t] == components.len() {
                        idx[t] = i;
                        // vの否定を立てる
                        ret[t] = v < self.size();
                    }
                }
            }

            Some(ret)
        }
    }
}

use scc::*;

fn main() {
    let n: usize = read();
    let ab = read_vec(n, || read_tuple!(usize, usize));

    const M: usize = 2000000;
    let mut is_prime = vec![true; M + 1];
    let nums = ab
        .citer()
        .enumerate()
        .flat_map(|(i, (a, b))| it!((a, i, true), (b, i, false)))
        .sorted()
        .group_by(|t| t.0)
        .into_iter()
        .map(|(x, it)| (x, it.map(|t| (t.1, t.2)).collect::<Vec<_>>()))
        .collect::<FxHashMap<_, _>>();
    let mut num_v = n;
    let mut edges = vec![];
    for i in 2..=M {
        if !is_prime[i] {
            continue;
        }

        let mut idxs = vec![];

        if let Some(v) = nums.get(&i) {
            idxs.extend(v.citer());
        }
        for j in (2..).map(|k| i * k).take_while(|&j| j <= M) {
            is_prime[j] = false;

            if let Some(v) = nums.get(&j) {
                idxs.extend(v.citer());
            }
        }

        if idxs.len() <= 1 {
            continue;
        }

        idxs.citer().skip(1).fold(idxs[0], |(ii, f), (jj, g)| {
            edges.push((ii, !f, jj, !g));
            let kk = num_v;
            num_v += 1;
            edges.push((ii, !f, kk, true));
            edges.push((jj, !g, kk, true));

            (kk, true)
        });
    }

    let mut s = TwoSat::new(num_v);

    for (ii, f, jj, g) in edges {
        s.add(ii, f, jj, g);
    }

    if let Some(_ans) = s.solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}
