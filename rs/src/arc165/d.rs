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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
type Edge = detail::UnweightedEdge;

mod scc {
    use super::*;

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
    fn rev_dfs(
        g: &Graph,
        v: usize,
        idx: usize,
        idxs: &mut Vec<usize>,
        vs: &mut Vec<usize>,
        adjs: &mut Vec<usize>,
    ) {
        idxs[v] = idx;
        vs.push(v);

        for edge in &g.in_edges[v] {
            if idxs[edge.from] == std::usize::MAX {
                rev_dfs(g, edge.from, idx, idxs, vs, adjs);
            } else if idxs[edge.from] < idx {
                adjs.push(idxs[edge.from]);
            }
        }
    }

    #[allow(dead_code)]
    pub fn scc(g: &Graph) -> (Vec<Vec<usize>>, Vec<usize>, Vec<Vec<usize>>) {
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
        let mut idxs = vec![std::usize::MAX; g.size()];
        let mut scc_edges = vec![];
        {
            for &v in vs.iter().rev() {
                if idxs[v] == std::usize::MAX {
                    let mut component = vec![];
                    let mut adjs = vec![];
                    rev_dfs(g, v, ret.len(), &mut idxs, &mut component, &mut adjs);
                    ret.push(component);
                    scc_edges.push(vec![]);
                    for idx in adjs.iter().copied().sorted().dedup() {
                        scc_edges[idx].push(ret.len() - 1);
                    }
                }
            }
        }

        (ret, idxs, scc_edges)
    }

    // 2-sat
    #[allow(dead_code)]
    struct TwoSat {
        g: Graph,
    }

    impl TwoSat {
        #[allow(dead_code)]
        fn new(n: usize) -> TwoSat {
            TwoSat {
                g: Graph::new(2 * n),
            }
        }

        // size()より小さいとfalse, size()以上だとtrue
        #[allow(dead_code)]
        fn to_v(&self, a: usize, f: bool) -> usize {
            if f { a + self.size() } else { a }
        }

        #[allow(dead_code)]
        fn add(&mut self, a: usize, fa: bool, b: usize, fb: bool) {
            self.g.add_edge((self.to_v(a, !fa), self.to_v(b, fb)));
            self.g.add_edge((self.to_v(b, !fb), self.to_v(a, fa)));
        }

        #[allow(dead_code)]
        fn size(&self) -> usize {
            self.g.size() / 2
        }

        #[allow(dead_code)]
        fn solve(&self) -> Option<Vec<bool>> {
            let (components, _, _) = scc(&self.g);

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

    mod test {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn test_scc() {
            let mut g = Graph::new(6);
            g.add_edge((0, 1));
            g.add_edge((1, 0));
            g.add_edge((1, 2));
            g.add_edge((2, 3));
            g.add_edge((3, 4));
            g.add_edge((4, 5));
            g.add_edge((5, 3));

            let (mut components, _, _) = scc(&g);
            assert!(components.len() == 3);

            for component in components.iter_mut() {
                component.sort();
            }
            assert!(components[0] == vec![0, 1]);
            assert!(components[1] == vec![2]);
            assert!(components[2] == vec![3, 4, 5]);
        }

        #[test]
        fn test_two_sat() {
            let mut sat = TwoSat::new(3);
            sat.add(0, true, 1, true);
            sat.add(1, true, 2, true);
            sat.add(0, true, 2, false);

            assert!(sat.solve() == Some(vec![false, true, false]))
        }
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        abcd: [(Usize1, usize, Usize1, usize); m]
    }

    let mut component_idxs = (0..n).collect::<Vec<_>>();
    let mut num_components = n;
    let mut idxs = vec![0; m];
    let mut num_ok = 0;
    while num_ok < m {
        let mut g = Graph::new(num_components);
        for ((a, b, c, d), idx) in izip!(abcd.citer(), idxs.iter_mut()) {
            if *idx == usize::MAX {
                // completed
                continue;
            }

            while *idx < b - a
                && *idx < d - c
                && component_idxs[a + *idx] == component_idxs[c + *idx]
            {
                *idx += 1;
            }

            if *idx >= d - c {
                println!("No");
                return;
            }

            if *idx >= b - a {
                num_ok += 1;
                *idx = usize::MAX;
                continue;
            }

            let x = a + *idx;
            let y = c + *idx;

            g.add_edge(Edge::new(component_idxs[x], component_idxs[y]));
        }

        let (components, new_component_idxs, _) = scc::scc(&g);

        if components.len() == num_components {
            break;
        }

        component_idxs.iter_mut().for_each(|cidx| {
            *cidx = new_component_idxs[*cidx];
        });
        num_components = components.len();
    }

    println!("Yes");
}
