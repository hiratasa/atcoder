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

type Graph = detail::WeightedGraph;

fn range(r: impl std::ops::RangeBounds<usize>, n: usize) -> (usize, usize) {
    let start = match r.start_bound() {
        std::ops::Bound::Excluded(&i) => i + 1,
        std::ops::Bound::Included(&i) => i,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match r.end_bound() {
        std::ops::Bound::Excluded(&i) => i,
        std::ops::Bound::Included(&i) => i + 1,
        std::ops::Bound::Unbounded => n,
    };
    (start, end)
}
trait Monoid {
    fn id() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    cap: usize,
    values: Vec<M>,
}
#[allow(dead_code)]
impl<M> SegmentTree<M>
where
    M: Monoid + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTree {
            n,
            cap,
            values: vec![M::id(); 2 * cap - 1],
        }
    }
    fn with<T>(vals: &[T]) -> Self
    where
        T: Into<M> + Clone,
    {
        let n = vals.len();
        let cap = n.next_power_of_two();
        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned().map(|x| x.into()));
        values.resize(2 * cap - 1, M::id());
        let mut st = SegmentTree { n, cap, values };
        for idx in (0..cap - 1).rev() {
            st.fix(idx);
        }
        st
    }
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
        }
    }
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }
    fn get(&self, pos: usize) -> M {
        self.values[self.cap - 1 + pos].clone()
    }
    fn set<T>(&mut self, pos: usize, v: T)
    where
        T: Into<M>,
    {
        let idx = self.cap - 1 + pos;
        self.values[idx] = v.into();
        self.fix_all(idx);
    }
    fn query(&self, r: impl std::ops::RangeBounds<usize>) -> M {
        let (a, b) = range(r, self.n);
        let mut left = M::id();
        let mut right = M::id();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }
            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
        M::op(&left, &right)
    }
    fn right_partition_point<F>(&self, a: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M) -> bool,
    {
        assert!(a <= self.cap);
        if !f(&M::id()) {
            Some(a)
        } else if a == self.cap {
            None
        } else {
            let mut b = a;
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);
            while f(&val_next) {
                val = val_next;
                b += len;
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;
                if idx == 0 {
                    return None;
                }
                val_next = M::op(&val, &self.values[idx]);
            }
            idx = 2 * idx + 1;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&val, &self.values[idx]);
                if f(&val_next) {
                    val = val_next;
                    b += len;
                    idx += 1;
                }
                len >>= 1;
                idx = 2 * idx + 1;
            }
            Some(b + 1)
        }
    }
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            None
        } else if b == 0 {
            Some(0)
        } else {
            let mut a = b;
            let mut idx = (a + self.cap - 1) >> (!(a + self.cap - 1)).trailing_zeros();
            let mut len = 1 << (!(a + self.cap - 1)).trailing_zeros();
            if idx == 0 {
                len = self.cap;
            } else {
                idx -= 1;
            }
            let mut val = M::id();
            let mut val_next = M::op(&self.values[idx], &val);
            while f(&val_next) {
                val = val_next;
                a -= len;
                if idx == 0 || (idx + 1).is_power_of_two() {
                    return Some(0);
                }
                len <<= (!idx).trailing_zeros();
                idx >>= (!idx).trailing_zeros();
                idx -= 1;
                val_next = M::op(&self.values[idx], &val);
            }
            idx = 2 * idx + 2;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&self.values[idx], &val);
                if f(&val_next) {
                    val = val_next;
                    a -= len;
                    idx -= 1;
                }
                len >>= 1;
                idx = 2 * idx + 2;
            }
            Some(a)
        }
    }
}
macro_rules! define_monoid {
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
        #[derive(Clone, Copy, Debug)]
        struct $name($t);
        impl Monoid for $name {
            fn id() -> Self {
                Self($id)
            }
            fn op(&self, rhs: &Self) -> Self {
                Self(($op)(self.0, rhs.0))
            }
        }
        impl From<$t> for $name {
            fn from(x: $t) -> $name {
                Self(x)
            }
        }
    };
}

define_monoid!(Sum, usize, 0, std::ops::Add::add);

fn dfs(g: &Graph, v: usize, parents: &mut [usize], sizes: &mut [usize]) {
    sizes[v] = 1;

    g.children(v, parents[v]).for_each(|u| {
        parents[u] = v;
        dfs(g, u, parents, sizes);

        sizes[v] += sizes[u];
    });
}

// HL分解
fn hld(
    g: &Graph,
    v: usize,
    p: usize,
    sizes: &[usize],
    vert_idxs: &mut [usize],
    edge_idxs: &mut [usize],
    heads: &mut [usize],
    tour: &mut Vec<usize>,
) {
    vert_idxs[v] = tour.len();

    if g.children(v, p).next().is_none() {
        return;
    }

    let max_e = g.children_edge(v, p).max_by_key(|&e| sizes[e.to]).unwrap();
    edge_idxs[max_e.label] = tour.len();
    heads[max_e.to] = heads[v];
    tour.push(max_e.label);

    hld(g, max_e.to, v, sizes, vert_idxs, edge_idxs, heads, tour);

    g.children_edge(v, p).for_each(|e| {
        if e.label != max_e.label {
            edge_idxs[e.label] = tour.len();
            heads[e.to] = e.to;
            tour.push(e.label);
            hld(g, e.to, v, sizes, vert_idxs, edge_idxs, heads, tour);
        }
    });
}

fn decompose_path(
    parents: &[usize],
    heads: &[usize],
    vert_idxs: &[usize],
    v: usize,
    u: usize,
) -> Vec<(usize, usize)> {
    let mut paths = vec![];

    let mut v = v;
    let mut u = u;
    while heads[v] != heads[u] {
        if vert_idxs[v] < vert_idxs[u] {
            paths.push((vert_idxs[heads[u]] - 1, vert_idxs[u]));
            u = parents[heads[u]];
        } else {
            paths.push((vert_idxs[heads[v]] - 1, vert_idxs[v]));
            v = parents[heads[v]];
        }
    }

    paths.push((
        min(vert_idxs[v], vert_idxs[u]),
        max(vert_idxs[v], vert_idxs[u]),
    ));

    paths
}

fn main() {
    let n = read::<usize>();
    let uvw = read_vec(n - 1, || read_tuple!(usize, usize, usize));
    let q = read::<usize>();
    let query = read_vec(q, || read_tuple!(usize, usize, usize));

    let g =
        Graph::from_edges1_undirected(n, uvw.citer().enumerate().map(|(i, (u, v, _))| (u, v, i)));

    let mut parents = vec![0; n];
    let mut sizes = vec![0; n];
    dfs(&g, 0, &mut parents, &mut sizes);

    let mut vert_idxs = vec![0; n];
    let mut edge_idxs = vec![0; n];
    let mut heads = vec![0; n];
    let mut tour = vec![];
    hld(
        &g,
        0,
        n,
        &sizes,
        &mut vert_idxs,
        &mut edge_idxs,
        &mut heads,
        &mut tour,
    );

    let st =
        SegmentTree::<Sum>::with(&tour.citer().map(|i_edge| uvw[i_edge].2).collect::<Vec<_>>());

    query
        .citer()
        .scan(st, |st, (t, x, y)| {
            if t == 1 {
                let i_edge = x - 1;
                let w = y;
                st.set(edge_idxs[i_edge], w);

                Some(None)
            } else {
                let v = x - 1;
                let u = y - 1;

                let ans = decompose_path(&parents, &heads, &vert_idxs, v, u)
                    .into_iter()
                    .map(|(s, t)| st.query(s..t).0)
                    .sum::<usize>();

                Some(Some(ans))
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{}", ans);
        });
}
