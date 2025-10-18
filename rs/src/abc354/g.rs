use itertools::{Itertools, izip};
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        a: [usize; n],
    };

    let (s, a): (Vec<_>, Vec<_>) = izip!(s.iter(), a)
        .sorted()
        .group_by(|(t, _)| t.as_str())
        .into_iter()
        .map(|(t, it)| (t, it.map(|(_, x)| x).max().unwrap()))
        .unzip();
    let n = s.len();

    let mut g = MaxFlowGraph::new(2 * n + 2);
    let src = 2 * n;
    let dst = 2 * n + 1;

    for i in 0..n {
        g.add_edge(src, i, a[i]);
        g.add_edge(n + i, dst, a[i]);

        for j in 0..n {
            if i == j {
                continue;
            }

            let t = s[i].chars().chain(s[j].chars()).collect::<Vec<_>>();

            let z = z_algorithm(&t);

            if z[s[i].len()..].iter().any(|&l| l >= s[i].len()) {
                g.add_edge(i, n + j, usize::MAX);
            }
        }
    }

    let f = g.max_flow(src, dst);
    let ans = a.iter().sum::<usize>() - f;

    println!("{ans}");
}

#[allow(dead_code)]
fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
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
}

// N個の頂点からいくつか選んでコストを最小にする
struct ProjectSelectionProblem {
    g: MaxFlowGraph,
    cost_offset: i64,
    src: usize,
    dst: usize,
}

#[allow(dead_code)]
impl ProjectSelectionProblem {
    fn new(n: usize) -> ProjectSelectionProblem {
        ProjectSelectionProblem {
            g: MaxFlowGraph::new(n + 2),
            cost_offset: 0,
            src: n,
            dst: n + 1,
        }
    }

    // 各頂点を選んだ時・選ばなかった時のコストを設定
    // 同一頂点に対しては高々1回だけ呼ぶこと
    // * `cost_a` - 選んだ時のコスト
    // * `cost_b` - 選ばなかった時のコスト
    fn set_cost<C: TryInto<i64>>(&mut self, idx: usize, cost_a: C, cost_b: C) {
        let cost_a = cost_a.try_into().ok().unwrap();
        let cost_b = cost_b.try_into().ok().unwrap();

        let offset = std::cmp::max(-cost_a, -cost_b).max(0);

        // コストをどちら側にのせるか注意
        self.g.add_edge(self.src, idx, (cost_b + offset) as usize);
        self.g.add_edge(idx, self.dst, (cost_a + offset) as usize);

        self.cost_offset += offset;
    }

    // idx番目の頂点は選ぶべき（選ばないべき）
    fn should(&mut self, idx: usize, select: bool) {
        if select {
            self.set_cost(idx, 0, 1i64 << 60);
        } else {
            self.set_cost(idx, 1i64 << 60, 0);
        }
    }

    // (idx0を選ぶ) ⇒ (idx1を選ぶ) の形の条件
    fn add_constraint(&mut self, idx0: usize, idx1: usize) {
        self.g.add_edge(idx0, idx1, usize::MAX);
    }

    // (idx0を選ぶ) ⇒ (idx1を選ぶ) の形の条件. 破るとコストがかかる
    fn add_constraint_cost(&mut self, idx0: usize, idx1: usize, cost: i64) {
        let cost = self.fix_cost(cost);
        self.g.add_edge(idx0, idx1, cost);
    }

    // idx0とidx1の選択が等しい の形の条件. 破るとコストがかかる
    fn add_equality_constraint(&mut self, idx0: usize, idx1: usize, cost: i64) {
        let cost = self.fix_cost(cost);

        // (idx0がA) => (idx1がA)
        self.g.add_edge(idx0, idx1, cost);
        // (idx1がA) => (idx0がA)
        self.g.add_edge(idx1, idx0, cost);
    }

    fn fix_cost(&mut self, cost: i64) -> usize {
        if cost > 0 {
            cost as usize
        } else {
            self.cost_offset += -cost;
            -cost as usize
        }
    }

    // 最小コストを求める
    fn calc_min_cost(&mut self) -> i64 {
        let f = self.g.max_flow(self.src, self.dst);

        f as i64 - self.cost_offset
    }
}
