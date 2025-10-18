use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        uv: [(usize, usize); m],
    };

    if k % 2 > 0 {
        println!("No");
        return;
    }

    if k == 0 {
        println!("Yes");
        println!("0");
        println!();
        return;
    }

    let g =
        Graph::from_edges1_undirected(n, uv.into_iter().enumerate().map(|(i, (u, v))| (u, v, i)));

    let mut depths = vec![0; n];
    let mut parents = vec![None; n];
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut sizes = vec![];
    for i in 0..n {
        let s = dfs(
            &g,
            i,
            n,
            m,
            0,
            &mut visited,
            &mut parents,
            &mut children,
            &mut depths,
        );

        if s > 0 {
            sizes.push(s);
        }
    }

    if k > sizes.iter().map(|&s| s / 2 * 2).sum::<usize>() {
        println!("No");
        return;
    }

    let mut k = k;
    let mut used = vec![false; n];
    let mut ans = vec![];
    for v in (0..n).sorted_by_key(|&v| Reverse(depths[v])) {
        if used[v] {
            continue;
        }

        // while matches!(children[v].last(), Some(&(_, u)) if used[u]) {
        //     children[v].pop();
        // }

        // assert!(children[v].is_empty());

        let Some((idx, p)) = parents[v] else { continue };

        while matches!(children[p].last(), Some(&(_, u)) if u == v || used[u]) {
            children[p].pop();
        }

        if let Some((idx2, u)) = children[p].pop() {
            used[v] = true;
            used[u] = true;

            ans.push(idx);
            ans.push(idx2);
        } else {
            used[v] = true;
            used[p] = true;

            ans.push(idx);
        }

        k -= 2;

        if k == 0 {
            break;
        }
    }

    assert!(k == 0);

    println!("Yes");

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&x| x + 1).join(" "));
}

type Graph = detail::WeightedGraph;

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

fn dfs(
    g: &Graph,
    v: usize,
    p: usize,
    idx: usize,
    d: usize,
    visited: &mut [bool],
    parents: &mut [Option<(usize, usize)>],
    children: &mut [Vec<(usize, usize)>],
    depths: &mut [usize],
) -> usize {
    if visited[v] {
        return 0;
    }

    visited[v] = true;
    depths[v] = d;

    if p < g.size() {
        parents[v] = Some((idx, p));
        children[p].push((idx, v));
    }

    1 + g.out_edges[v]
        .iter()
        .map(|e| {
            dfs(
                g,
                e.to,
                v,
                e.label,
                d + 1,
                visited,
                parents,
                children,
                depths,
            )
        })
        .sum::<usize>()
}
