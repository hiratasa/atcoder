use cargo_snippet::snippet;

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

#[allow(dead_code)]
#[snippet("dijkstra")]
fn dijkstra(edges: &Vec<Vec<Edge>>, src: usize) -> Vec<usize> {
    let n = edges.len();

    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];

    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;

    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }

        for &edge in &edges[v] {
            let next_cost = cost + edge.weight;

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
fn dijkstra1(edges: &Vec<Vec<Edge>>, src: usize, dst: usize) -> Option<usize> {
    let n = edges.len();

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

        for &edge in &edges[v] {
            let next_cost = cost + edge.weight;

            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }

    None
}
