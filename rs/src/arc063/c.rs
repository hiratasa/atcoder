use std::cmp::{max, min};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        k: usize,
        vp: [(Usize1, i64); k],
    };

    let g = Graph::from_edges_undirected(n, ab);

    let mut ranges = vp.into_iter().fold(vec![None; n], |mut ranges, (v, p)| {
        ranges[v] = Some((p, p));
        ranges
    });

    if !check(&g, 0, n, &mut ranges) {
        println!("No");
        return;
    }

    println!("Yes");
    let mut ans = vec![0; n];
    ans[0] = ranges[0].unwrap().0;
    fill(&g, 0, n, &ranges, &mut ans);

    for x in ans {
        println!("{x}");
    }
}

fn check(g: &Graph, v: usize, p: usize, ranges: &mut [Option<(i64, i64)>]) -> bool {
    let merge = |u: usize, ranges: &mut [Option<(i64, i64)>]| {
        ranges[v] = match (ranges[v], ranges[u]) {
            (None, None) => None,
            (None, Some((l1, u1))) => Some((l1 - 1, u1 + 1)),
            (Some((l0, u0)), None) => Some((l0, u0)),
            (Some((l0, u0)), Some((l1, u1))) => {
                if l0 % 2 == l1 % 2 {
                    return false;
                }

                let lower = max(l0, l1 - 1);
                let upper = min(u0, u1 + 1);

                if lower > upper {
                    return false;
                }

                Some((lower, upper))
            }
        };

        true
    };

    for u in g.children(v, p) {
        if !check(g, u, v, ranges) {
            return false;
        }

        if !merge(u, ranges) {
            return false;
        }
    }

    true
}

fn fill(g: &Graph, v: usize, p: usize, ranges: &[Option<(i64, i64)>], ans: &mut [i64]) {
    if p < g.size() {
        let x = ans[p];

        ans[v] = match ranges[v] {
            Some((l, _)) => {
                if x - 1 >= l {
                    x - 1
                } else {
                    x + 1
                }
            }
            None => x - 1,
        };
    }

    for u in g.children(v, p) {
        fill(g, u, v, ranges, ans);
    }
}

type Graph = detail::UnweightedGraph;

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
