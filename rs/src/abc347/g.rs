use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    let mut g = MaxFlowGraph::new(6 * n * n + 2);
    let src = 6 * n * n;
    let dst = 6 * n * n + 1;

    let idx = |i: usize, j: usize, l: usize| {
        assert!(l < 6);
        let v = i * n + j;
        v * 6 + l
    };

    let mut offset = 0i64;
    for i in 0..n {
        for j in 0..n {
            let even = (i + j) % 2 == 0;
            let num_neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(ni, nj)| Some((i.checked_add_signed(ni)?, j.checked_add_signed(nj)?)))
                .filter(|&(ni, nj)| ni < n && nj < n)
                .count();

            g.add_edge(src, idx(i, j, a[i][j]), usize::MAX);
            for l in 1..=5 {
                g.add_edge(idx(i, j, l), idx(i, j, l - 1), usize::MAX);
            }

            if even {
                let calc = |l: usize| {
                    let l = l as i64;
                    (l * l - 10 * l) * num_neighbors as i64
                };

                if a[i][j] == 0 {
                    offset += calc(5);
                    for l in 1..=5 {
                        g.add_edge(src, idx(i, j, l), (calc(l - 1) - calc(l)) as usize);
                    }
                } else {
                    let l = a[i][j];
                    offset += calc(l);
                    for l in l + 1..6 {
                        g.add_edge(idx(i, j, l), dst, usize::MAX);
                    }
                }
            } else {
                let calc = |l: usize| l * l * num_neighbors;
                if a[i][j] == 0 {
                    for l in 1..=5 {
                        g.add_edge(idx(i, j, l), dst, calc(l) - calc(l - 1));
                    }
                } else {
                    let l = a[i][j];
                    offset += calc(l) as i64;
                    for l in l + 1..6 {
                        g.add_edge(idx(i, j, l), dst, usize::MAX);
                    }
                }
            }

            if even {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(|(ni, nj)| {
                        Some((i.checked_add_signed(ni)?, j.checked_add_signed(nj)?))
                    })
                    .filter(|&(ni, nj)| ni < n && nj < n)
                    .for_each(|(ni, nj)| {
                        for l0 in 1..=5 {
                            for l1 in 1..=5 {
                                g.add_edge(idx(i, j, l0), idx(ni, nj, l1), 2);
                            }
                        }
                    });
            }
        }
    }

    let flow = offset + g.max_flow(src, dst) as i64;
    #[allow(non_snake_case)]
    let in_S = g.min_cut(src, dst);

    eprintln!("flow={flow}");

    for i in 0..n {
        println!(
            "{}",
            (0..n)
                .map(|j| { (0..=5).rev().find(|&l| in_S[idx(i, j, l)]).unwrap() })
                .join(" ")
        );
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

use detail::{Edge, Graph};

#[allow(dead_code)]
struct MaxFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
}

#[allow(dead_code)]
impl MaxFlowGraph {
    fn new(n: usize) -> MaxFlowGraph {
        MaxFlowGraph {
            g: Graph::new(n),
            caps: vec![],
            rev: vec![],
        }
    }

    fn num_vertices(&self) -> usize {
        self.g.size()
    }

    fn num_edges(&self) -> usize {
        self.caps.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let idx = self.num_edges();
        let rev_idx = self.num_edges() + 1;

        self.g.add_edge(Edge::new_with_label(from, to, idx));
        self.g.add_edge(Edge::new_with_label(to, from, rev_idx));

        // forward edge
        self.caps.push(cap);
        self.rev.push(rev_idx);

        // backward edge
        self.caps.push(0);
        self.rev.push(idx);
    }

    fn bfs(&self, src: usize, dst: usize) -> Option<Vec<usize>> {
        fn chmin(a: &mut usize, b: usize) -> bool {
            if *a > b {
                *a = b;
                true
            } else {
                false
            }
        }

        let mut q = std::collections::VecDeque::new();
        let mut costs = vec![std::usize::MAX; self.num_vertices()];

        q.push_back(src);
        costs[src] = 0;

        while let Some(v) = q.pop_front() {
            if v == dst {
                return Some(costs);
            }

            let c = costs[v];
            self.g.out_edges[v]
                .iter()
                .filter(|e| self.caps[e.label] > 0)
                .filter(|e| chmin(&mut costs[e.to], c + 1))
                .for_each(|e| q.push_back(e.to));
        }

        None
    }

    fn dfs(
        &mut self,
        src: usize,
        dst: usize,
        upper: usize,
        levels: &Vec<usize>,
        itrs: &mut Vec<usize>,
    ) -> usize {
        if src == dst {
            return upper;
        }

        let mut total_flow = 0;
        for i in itrs[src]..self.g.out_edges[src].len() {
            let e = self.g.out_edges[src][i];
            if levels[src] + 1 == levels[e.to] && self.caps[e.label] > 0 {
                let flow = self.dfs(
                    e.to,
                    dst,
                    (upper - total_flow).min(self.caps[e.label]),
                    levels,
                    itrs,
                );

                self.caps[e.label] -= flow;
                self.caps[self.rev[e.label]] += flow;

                total_flow += flow;
                if upper == total_flow {
                    // NOTE:
                    //  この場合はitrs[src]はインクリメントしないこと！
                    //  (この辺に沿ってまだ流せるかもしれない)
                    return total_flow;
                }
            }
            itrs[src] += 1;
        }

        total_flow
    }

    fn max_flow(&mut self, src: usize, dst: usize) -> usize {
        let mut total_flow = 0;
        loop {
            if let Some(levels) = self.bfs(src, dst) {
                // ここでは一回のdfsで流せるだけ流しきる方式を採用しているので、
                // 複数回呼ぶ必要はない
                total_flow += self.dfs(
                    src,
                    dst,
                    std::usize::MAX,
                    &levels,
                    &mut vec![0; self.num_vertices()],
                );
            } else {
                break;
            }
        }

        total_flow
    }

    // max_flow後に呼ぶこと
    fn min_cut(&self, src: usize, _dst: usize) -> Vec<bool> {
        let mut stack = vec![src];
        let mut visited = vec![false; self.g.size()];

        while let Some(v) = stack.pop() {
            if visited[v] {
                continue;
            }

            visited[v] = true;

            stack.extend(
                self.g.out_edges[v]
                    .iter()
                    .filter(|&&e| self.caps[e.label] > 0)
                    .map(|e| e.to),
            );
        }

        visited
    }
}
