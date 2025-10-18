use cargo_snippet::snippet;

#[allow(unused_imports)]
pub use detail::{Edge, Graph};

#[snippet("graph")]
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

    impl std::convert::From<(usize, usize)> for Edge<()> {
        fn from(t: (usize, usize)) -> Self {
            Edge::new(t.0, t.1)
        }
    }
    impl std::convert::From<&(usize, usize)> for Edge<()> {
        fn from(t: &(usize, usize)) -> Self {
            Edge::from(*t)
        }
    }

    impl<W> std::convert::From<(usize, usize, W)> for Edge<W>
    where
        W: Copy,
    {
        fn from(t: (usize, usize, W)) -> Self {
            Edge::new_with_label(t.0, t.1, t.2)
        }
    }
    impl<W> std::convert::From<&(usize, usize, W)> for Edge<W>
    where
        W: Copy,
    {
        fn from(t: &(usize, usize, W)) -> Self {
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

        // with offset 1
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

        // with offset 1
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

        // pub fn add_edge<T: std::convert::Into<Edge<W>>>(&mut self, e: T) {
        pub fn add_edge<T>(&mut self, e: T)
        where
            Edge<W>: std::convert::From<T>,
        {
            // let edge = e.into();
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

mod dfs_recursive {
    use super::*;
    use cargo_snippet::snippet;

    #[snippet("graph_dfs_recursive")]
    #[allow(dead_code)]
    fn dfs(g: &Graph, v: usize, visited: &mut [bool]) {
        if visited[v] {
            return;
        }

        visited[v] = true;

        for &edge in &g.out_edges[v] {
            dfs(g, edge.to, visited);
        }
    }
}

mod dfs {
    use super::*;
    use cargo_snippet::snippet;

    #[snippet("graph_dfs")]
    #[allow(dead_code)]
    fn dfs(g: &Graph, src: usize) {
        let mut visited = vec![false; g.size()];
        let mut stack = vec![src];

        visited[src] = true;
        while let Some(v) = stack.pop() {
            g.out_edges[v]
                .iter()
                .map(|e| e.to)
                .filter(|&u| !std::mem::replace(&mut visited[u], true))
                .for_each(|u| stack.push(u));
        }
    }
}

mod tree_dfs {
    use super::*;
    use cargo_snippet::snippet;

    #[snippet("tree_dfs")]
    #[allow(dead_code)]
    fn dfs(g: &Graph, v: usize, p: usize) {
        g.out_edges[v]
            .iter()
            .map(|e| e.to)
            .filter(|&u| u != p)
            .for_each(|u| dfs(g, u, v))
    }
}

#[allow(dead_code)]
#[snippet("dijkstra")]
fn dijkstra(g: &Graph<usize>, src: usize) -> Vec<usize> {
    let n = g.size();

    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];

    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;

    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }

        for &edge in &g.out_edges[v] {
            let next_cost = cost + edge.label;

            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }

    costs
}

#[allow(dead_code)]
#[snippet("dijkstra1")]
fn dijkstra1(g: &Graph<usize>, src: usize, dst: usize) -> Option<usize> {
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

        for &edge in &g.out_edges[v] {
            let next_cost = cost + edge.label;

            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }

    None
}

// 強連結成分分解
mod scc {
    use super::*;
    use itertools::Itertools;

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
    fn scc(g: &Graph) -> (Vec<Vec<usize>>, Vec<usize>, Vec<Vec<usize>>) {
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
