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

type Graph = detail::UnweightedGraph;

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    cap: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> SegmentTree<M>
where
    M: Monoid,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTree {
            cap,
            values: vec![M::id(); 2 * cap - 1],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let n = vals.len();
        let cap = n.next_power_of_two();

        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned());
        values.resize(2 * cap - 1, M::id());

        let mut st = SegmentTree { cap, values };
        for idx in (0..cap - 1).rev() {
            st.fix_value(idx);
        }
        st
    }

    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
    }

    fn get(&self, pos: usize) -> M::Item {
        self.values[self.cap - 1 + pos].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        let mut idx = self.cap - 1 + pos;

        self.values[idx] = v;

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn query(&self, a: usize, b: usize) -> M::Item {
        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        let c0 = std::cmp::min(
            // trailing_ones
            (!left_idx).trailing_zeros(),
            (right_idx + 1).trailing_zeros(),
        );
        left_idx = left_idx >> c0;
        right_idx = ((right_idx + 1) >> c0) - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }

            let c = std::cmp::min(
                // trailing_ones
                (!left_idx).trailing_zeros(),
                (right_idx + 1).trailing_zeros(),
            );
            left_idx = left_idx >> c;
            right_idx = ((right_idx + 1) >> c) - 1;
        }

        M::op(&left, &right)
    }

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
    // (存在しないときにnを返してしまうとquery(a,n)がfalseのときと区別がつかないのでNoneを返す)
    fn right_partition_point<F>(&self, a: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(a <= self.cap);
        if !f(&M::id()) {
            Some(a)
        } else if a == self.cap {
            None
        } else {
            let mut b = a;
            // [b, b+2^k) が保持されている最初の箇所に移動
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);

            // チェックする範囲を拡大しながらf()がtrueになる限りbを右に伸ばしていく
            while f(&val_next) {
                val = val_next;
                b += len;

                // [b, b+2^k) が保持されている最初の箇所に移動
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;

                // 最後に計算したidxが右端だった場合
                if idx == 0 {
                    return None;
                }
                val_next = M::op(&val, &self.values[idx]);
            }

            // 範囲を縮小しながらbを右に伸ばしていく
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

            // [a, b)区間でfがtrue => 求めるbはその次
            Some(b + 1)
        }
    }

    // f(query(a, b)) == false となるaが存在すればその最大のもの+1を返す
    // 存在しない場合は0を返す
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> usize
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            b
        } else if b == 0 {
            0
        } else {
            let mut a = b;
            // [a-2^k, a) が保持されている最初の箇所に移動
            let mut idx = (a + self.cap - 1) >> (!(a + self.cap - 1)).trailing_zeros();
            let mut len = 1 << (!(a + self.cap - 1)).trailing_zeros();
            if idx == 0 {
                // このケースになるのはb=self.capのときだけ
                len = self.cap;
            } else {
                idx -= 1;
            }

            let mut val = M::id();
            let mut val_next = M::op(&self.values[idx], &val);

            // チェックする範囲を拡大しながらf()がtrueになる限りaを左に伸ばしていく
            while f(&val_next) {
                val = val_next;
                a -= len;

                // 最後に計算したidxが左端だった場合
                if idx == 0 || (idx + 1).is_power_of_two() {
                    return 0;
                }

                // [a-2^k, a) が保持されている最初の箇所に移動
                len <<= (!idx).trailing_zeros();
                idx >>= (!idx).trailing_zeros();
                idx -= 1;

                val_next = M::op(&self.values[idx], &val);
            }

            // 範囲を縮小しながらaを左に伸ばしていく
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

            a
        }
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone, Debug)]
        struct $name;

        impl Monoid for $name {
            type Item = $t;

            fn id() -> Self::Item {
                $id
            }

            fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
                ($op)(*lhs, *rhs)
            }
        }
    };
}

define_monoid!(Sum, usize, 0, std::ops::Add::add);

type ST = SegmentTree<Sum>;

fn calc_median(
    g: &Graph,
    v: usize,
    p: usize,
    st: &mut ST,
    a: &[usize],
    a_to_idx: &FxHashMap<usize, usize>,
    idx_to_a: &[usize],
    medians: &mut [usize],
) {
    let aa = a[v];
    let idx = a_to_idx[&aa];
    st.set(idx, st.get(idx) + 1);

    // leaf
    if p < g.size() && g.out_edges[v].len() == 1 {
        let l = idx_to_a.len();
        let d = st.query(0, l);
        if d % 2 == 0 {
            let idx0 = st.right_partition_point(0, |&k| k < d / 2).unwrap();
            let idx1 = st.right_partition_point(0, |&k| k <= d / 2).unwrap();
            medians[v] = (idx_to_a[idx0 - 1] + idx_to_a[idx1 - 1]) / 2;
        } else {
            let idx0 = st.right_partition_point(0, |&k| k <= d / 2).unwrap();
            medians[v] = idx_to_a[idx0 - 1];
        }
    }

    for &e in &g.out_edges[v] {
        if e.to == p {
            continue;
        }

        calc_median(&g, e.to, v, st, a, a_to_idx, idx_to_a, medians);
    }

    st.set(idx, st.get(idx) - 1);
}

fn dfs(g: &Graph, v: usize, p: usize, depth: usize, medians: &[usize]) -> usize {
    // leaf
    if p < g.size() && g.out_edges[v].len() == 1 {
        return medians[v];
    }

    if depth % 2 == 0 {
        g.out_edges[v]
            .citer()
            .filter(|&e| e.to != p)
            .map(|e| dfs(&g, e.to, v, depth + 1, medians))
            .max()
            .unwrap()
    } else {
        g.out_edges[v]
            .citer()
            .filter(|&e| e.to != p)
            .map(|e| dfs(&g, e.to, v, depth + 1, medians))
            .min()
            .unwrap()
    }
}

fn main() {
    let n: usize = read();
    let a = read_row::<usize>();
    let uv = read_vec(n - 1, || read_tuple!(usize, usize));

    let g = Graph::from_edges1_undirected(n, uv);

    let idx_to_a = a.citer().sorted().dedup().collect::<Vec<_>>();
    let a_to_idx = idx_to_a
        .citer()
        .enumerate()
        .map(|(i, aa)| (aa, i))
        .collect::<FxHashMap<_, _>>();

    let mut medians = vec![0; n];
    calc_median(
        &g,
        0,
        n,
        &mut ST::new(idx_to_a.len()),
        &a,
        &a_to_idx,
        &idx_to_a,
        &mut medians,
    );

    let ans = dfs(&g, 0, n, 0, &medians);
    println!("{}", ans);
}
