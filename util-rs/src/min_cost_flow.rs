use std::iter::*;

use super::graph::{Edge, Graph};

struct MinCostFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
    weights: Vec<i64>,
}

#[allow(dead_code)]
impl MinCostFlowGraph {
    fn new(n: usize) -> MinCostFlowGraph {
        MinCostFlowGraph {
            g: Graph::new(n),
            caps: vec![],
            rev: vec![],
            weights: vec![],
        }
    }

    fn num_vertices(&self) -> usize {
        self.g.size()
    }

    fn num_edges(&self) -> usize {
        self.caps.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize, cost: i64) {
        let idx = self.num_edges();
        let rev_idx = self.num_edges() + 1;

        self.g.add_edge(Edge::new_with_label(from, to, idx));
        self.g.add_edge(Edge::new_with_label(to, from, rev_idx));

        // forward edge
        self.caps.push(cap);
        self.rev.push(rev_idx);
        self.weights.push(cost);

        // backward edge
        self.caps.push(0);
        self.rev.push(idx);
        self.weights.push(-cost);
    }

    fn dijkstra(&self, src: usize, h: &[i64]) -> (Vec<i64>, Vec<Option<(usize, usize)>>) {
        let n = self.num_vertices();

        let mut q = std::collections::BinaryHeap::new();
        let mut costs = vec![std::i64::MAX; n];
        let mut parents = vec![None; n];
        q.push(std::cmp::Reverse((0, src)));
        costs[src] = 0;

        while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
            if cost > costs[v] {
                continue;
            }

            for edge in &self.g.out_edges[v] {
                if self.caps[edge.label] == 0 {
                    continue;
                }

                let next_cost = cost + self.weights[edge.label] - h[edge.to] + h[edge.from];
                assert!(cost <= next_cost);
                if next_cost < costs[edge.to] {
                    q.push(std::cmp::Reverse((next_cost, edge.to)));
                    costs[edge.to] = next_cost;
                    parents[edge.to] = Some((edge.label, v));
                }
            }
        }

        (costs, parents)
    }

    // (流量, コスト)のリストを返す
    fn flow(&mut self, src: usize, dst: usize, limit: usize) -> Vec<(usize, i64)> {
        let mut f = 0;
        let mut v = vec![];
        // NOTE: 負辺がある場合は、hの初期値を各点への最小コストなどに変える
        // (weight(v=>u) >= h[u] - h[v] を満たせばなんでもよい)
        let mut h = vec![0; self.num_vertices()];

        while f < limit {
            let (costs, parents) = self.dijkstra(src, &h);

            for v in 0..self.num_vertices() {
                h[v] = h[v].saturating_add(costs[v]);
            }

            let f1 = if let Some(f1) = successors(parents[dst], |(_, p)| parents[*p])
                .map(|(id, _p)| self.caps[id])
                .chain(once(limit - f))
                .min()
            {
                f1
            } else {
                break;
            };
            assert!(f1 > 0);
            f += f1;

            let mut c = 0;
            successors(parents[dst], |(_, p)| parents[*p]).for_each(|(id, _p)| {
                c += f1 as i64 * self.weights[id];

                self.caps[id] -= f1;
                self.caps[self.rev[id]] += f1;
            });

            v.push((f, c));
        }

        v
    }
}
