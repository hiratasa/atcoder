#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

#[derive(Clone, Copy, Debug)]
enum EdgeType {
    A,
    B,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    edge_type: EdgeType,
}

#[allow(dead_code)]
impl Edge {
    fn from_stdin() -> Edge {
        let (edge_type, from, to) = read_tuple!(usize, usize, usize);
        Edge {
            from: from,
            to: to,
            edge_type: if edge_type == 0 {
                EdgeType::A
            } else {
                EdgeType::B
            },
        }
    }
    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            ..*self
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Graph {
    out_edges: Vec<Vec<Edge>>,
}
#[allow(dead_code)]
impl Graph {
    fn from_stdin_undirected(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];
        for _ in 0..m {
            let e = Edge::from_stdin();
            out_edges[e.from].push(e);
            out_edges[e.to].push(e.rev());
        }
        Graph { out_edges }
    }
    fn from_stdin_directed(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];
        for _ in 0..m {
            let e = Edge::from_stdin();
            out_edges[e.from].push(e);
        }
        Graph { out_edges }
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let g = Graph::from_stdin_undirected(n, m);

    fn calc_next_b(b: usize, edge_type: EdgeType) -> usize {
        match edge_type {
            EdgeType::A => b,
            EdgeType::B => b + 1,
        }
    }

    fn calc_next_cost(cost: usize, b: usize, edge_type: EdgeType) -> usize {
        match edge_type {
            EdgeType::A => cost + 1,
            EdgeType::B => cost + b + 1,
        }
    }

    let init = once((usize::MAX, usize::MAX)).collect::<BTreeSet<_>>();

    let mut q = BinaryHeap::new();
    let mut costs = vec![init; n];

    q.push(Reverse((0, 0, 0)));
    costs[0] = once((0, 0)).collect::<BTreeSet<_>>();
    while let Some(Reverse((cost, b, v))) = q.pop() {
        if !costs[v].contains(&(cost, b)) {
            continue;
        }

        g.out_edges[v]
            .iter()
            .map(|e| {
                (
                    calc_next_cost(cost, b, e.edge_type),
                    calc_next_b(b, e.edge_type),
                    e.to,
                )
            })
            .for_each(|(next_cost, next_b, u)| {
                use std::ops::Bound::*;
                if let Some(&(_tmp_cost, tmp_b)) = costs[u]
                    .range((Unbounded, Included(&(next_cost, next_b))))
                    .next_back()
                {
                    if tmp_b <= next_b {
                        return;
                    }
                }

                let to_be_removed = costs[u]
                    .range((Excluded(&(next_cost, next_b)), Unbounded))
                    .take_while(|&&(_tmp_cost, tmp_b)| tmp_b >= next_b)
                    .copied()
                    .collect_vec();
                for e in to_be_removed {
                    costs[u].remove(&e);
                }

                costs[u].insert((next_cost, next_b));
                q.push(Reverse((next_cost, next_b, u)));
            });
    }

    for (cost, _) in costs.iter().map(|c| c.iter().next().unwrap()) {
        println!("{}", cost);
    }
}
