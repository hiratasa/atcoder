use super::graph::{Edge, Graph};

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

    // dinic法
    // 計算量
    //  - O(EV^2) (一般の場合)
    //  - O(E^(3/2)) (全ての辺の容量が同一(容量ゼロの辺を除く))
    //  - O(EV^(2/3)) (全ての辺の容量が同一かつ多重辺がない(容量ゼロの辺を除く))
    //  - O(EV^(1/2)) (全ての辺の容量が同一かつ全ての頂点で入次数もしくは出次数が1(容量ゼロの辺を除く))
    //                (e.g. 二部マッチング)
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
