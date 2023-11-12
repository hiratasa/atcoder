#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{Readable, Source},
};

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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
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

fn calc_lowlink(g: &Graph, components: &mut [usize], bridges: &mut Vec<(usize, usize)>) -> usize {
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

// 子が1つ(以下)でleafまで一本道 ⇒ Err(len)
// それ以外 => Ok((長さ1の枝、長さ2以上の枝))
fn dfs(g: &Graph, v: usize, p: usize) -> Result<(usize, usize), usize> {
    let num_children = g.children(v, p).count();

    if num_children == 0 {
        return Err(0);
    }

    g.children(v, p)
        .map(|u| dfs(g, u, v))
        .try_fold((0, 0), |(num1, num2), res| match res {
            Ok((num1_1, num2_1)) => Ok((num1 + num1_1, num2 + num2_1)),
            Err(leaf_len) => {
                if num_children == 1 {
                    Err(leaf_len + 1)
                } else if leaf_len == 0 {
                    Ok((num1 + 1, num2))
                } else {
                    Ok((num1, num2 + 1))
                }
            }
        })
}

fn solve0(g: &Graph) -> Option<usize> {
    let n = g.size();

    let adjs =
        (0..n)
            .flat_map(|i| g.out_edges[i].citer())
            .fold(vec![vec![false; n]; n], |mut adjs, e| {
                adjs[e.from][e.to] = true;
                adjs
            });

    let mut components = vec![0; n];
    let mut bridges = vec![];
    calc_lowlink(&g, &mut components, &mut bridges);

    if bridges.len() == 1 {
        return Some(0);
    }

    if bridges.len() == 0 {
        return None;
    }

    let cand = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| !adjs[i][j])
        .collect::<Vec<_>>();
    let m = cand.len();

    (1usize..1 << m)
        .sorted_by_key(|&s| s.count_ones())
        .find(|&s| {
            let mut g = g.clone();

            (0..m)
                .filter(|&i| s & (1 << i) > 0)
                .map(|i| cand[i])
                .for_each(|(v, u)| {
                    g.add_edge((v, u));
                    g.add_edge((u, v));
                });

            let mut components = vec![0; n];
            let mut bridges = vec![];
            calc_lowlink(&g, &mut components, &mut bridges);

            bridges.len() == 1
        })
        .map(|s| s.count_ones() as usize)
}

fn solve(g: &Graph) -> Option<usize> {
    let mut components = vec![0; g.size()];
    let mut bridges = vec![];
    let n = calc_lowlink(&g, &mut components, &mut bridges);

    let bridges = bridges
        .citer()
        .map(|(i, j)| (components[i], components[j]))
        .collect::<Vec<_>>();

    let tree = Graph::from_edges_undirected(n, bridges);

    if n == 1 {
        return None;
    }

    if n == 2 {
        return Some(0);
    }

    let leaf = (0..n).find(|&i| tree.out_edges[i].len() == 1).unwrap();

    let (num1, num2) = match dfs(&tree, leaf, n) {
        Ok((num1, num2)) => {
            let l = successors(Some((leaf, n)), |&(v, p)| {
                if let Ok(u) = tree.children(v, p).exactly_one() {
                    Some((u, v))
                } else {
                    None
                }
            })
            .count()
                - 1;

            if l == 1 {
                (num1 + 1, num2)
            } else {
                (num1, num2 + 1)
            }
        }
        Err(leaf_len) => {
            assert!(leaf_len >= 2);
            if leaf_len == 2 {
                if g.size() == 3 {
                    return None;
                }
            }

            return Some(1);
        }
    };

    match (num1, num2) {
        (0, 0) | (1, 0) | (0, 1) | (2, 0) | (0, 2) | (1, 1) => unreachable!(),
        (0, a) => Some((a + 1) / 2),
        (z, a) => Some((z + a) / 2),
    }
}

fn main() {
    input! {
        v: usize, e: usize,
        ab: [(usize, usize); e],
    };

    let g = Graph::from_edges_undirected(v, &ab);

    let ans = solve(&g);

    if let Some(ans) = ans {
        println!("{ans}");
    } else {
        println!("IMPOSSIBLE");
    }
}
