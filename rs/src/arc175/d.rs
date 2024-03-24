use std::cmp;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let g = Graph::from_edges_undirected(n, uv);

    let mut sizes = vec![0; n];
    let mut total_sizes = vec![0; n];
    calc_sizes(&g, 0, n, &mut sizes, &mut total_sizes);

    if k < n || k > total_sizes[0] {
        println!("No");
        return;
    }

    println!("Yes");

    let mut marked = vec![false; n];
    calc_marked(&g, 0, n, &sizes, &total_sizes, k, &mut marked);

    let num_marked = marked.iter().filter(|&&x| x).count();
    let mut ans = vec![0; n];
    calc_labels(
        &g,
        0,
        n,
        &marked,
        &mut (1..=n - num_marked).collect(),
        &mut (n - num_marked + 1..=n).rev().collect::<Vec<_>>(),
        &mut ans,
    );

    println!("{}", ans.iter().join(" "));
}

fn calc_sizes(g: &Graph, v: usize, p: usize, sizes: &mut [usize], total_sizes: &mut [usize]) {
    g.children(v, p)
        .for_each(|u| calc_sizes(g, u, v, sizes, total_sizes));

    sizes[v] = 1 + g.children(v, p).map(|u| sizes[u]).sum::<usize>();
    total_sizes[v] = sizes[v] + g.children(v, p).map(|u| total_sizes[u]).sum::<usize>();
}

fn calc_marked(
    g: &Graph,
    v: usize,
    p: usize,
    sizes: &[usize],
    total_sizes: &[usize],
    k: usize,
    marked: &mut [bool],
) {
    let mut k = if sizes[v] <= k {
        marked[v] = true;
        k - sizes[v]
    } else {
        k
    };

    for u in g.children(v, p) {
        let l = cmp::min(k, total_sizes[u]);
        k -= l;
        calc_marked(g, u, v, sizes, total_sizes, l, marked);
    }
}

fn calc_labels(
    g: &Graph,
    v: usize,
    p: usize,
    marked: &[bool],
    idxs_non_marked: &mut Vec<usize>,
    idxs_marked: &mut Vec<usize>,
    labels: &mut [usize],
) {
    if marked[v] {
        labels[v] = idxs_marked.pop().unwrap();
    } else {
        labels[v] = idxs_non_marked.pop().unwrap();
    }

    g.children(v, p).for_each(|u| {
        calc_labels(g, u, v, marked, idxs_non_marked, idxs_marked, labels);
    });
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
