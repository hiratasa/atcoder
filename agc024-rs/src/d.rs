#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[allow(dead_code)]
impl FromStr for Edge {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut it = s.split_whitespace();

        let from: usize = it.next().unwrap().parse()?;
        let to: usize = it.next().unwrap().parse()?;

        Ok(Edge {
            from: from - 1,
            to: to - 1,
        })
    }
}

#[allow(dead_code)]
impl Edge {
    fn rev(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
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
            let e: Edge = read();

            out_edges[e.from].push(e);
            out_edges[e.to].push(e.rev());
        }

        Graph { out_edges }
    }

    fn from_stdin_directed(n: usize, m: usize) -> Graph {
        let mut out_edges = vec![vec![]; n];

        for _ in 0..m {
            let e: Edge = read();

            out_edges[e.from].push(e.clone());
        }

        Graph { out_edges }
    }

    fn farthest_vertex(&self, p: usize, v: usize, depth: usize) -> (usize, usize) {
        let (mut v0, mut d0) = (v, depth);
        for e in &self.out_edges[v] {
            if e.to == p {
                continue;
            }

            let (v1, d1) = self.farthest_vertex(v, e.to, depth + 1);
            if d1 > d0 {
                v0 = v1;
                d0 = d1;
            }
        }

        (v0, d0)
    }

    fn get_parents(&self, parents: &mut Vec<usize>, v: usize) {
        for e in &self.out_edges[v] {
            if e.to == parents[v] {
                continue;
            }

            parents[e.to] = v;
            self.get_parents(parents, e.to);
        }
    }

    fn get_subsize(&self, subsize: &mut Vec<usize>, p: usize, v: usize, depth: usize) {
        subsize[depth] = max(subsize[depth], self.out_edges[v].len() - 1);

        for e in &self.out_edges[v] {
            if e.to == p {
                continue;
            }

            self.get_subsize(subsize, v, e.to, depth + 1);
        }
    }
}

fn main() {
    let n = read();

    let g = Graph::from_stdin_undirected(n, n - 1);

    let (v, _) = g.farthest_vertex(n, 0, 0);
    let (u, d) = g.farthest_vertex(n, v, 0);

    let mut parents = vec![v; n];
    g.get_parents(&mut parents, v);

    if d % 2 == 0 {
        let c = (0..(d / 2)).fold(u, |u1, _| parents[u1]);

        let ss: Vec<_> = g.out_edges[c]
            .iter()
            .map(|&e| {
                let mut subsize = vec![1; d / 2 + 1];
                g.get_subsize(&mut subsize, c, e.to, 0);
                assert!(*subsize.last().unwrap() == 1);
                subsize
            })
            .collect();

        let ans0 = ss.len()
            * ss.iter()
                .fold(vec![1; d / 2 + 1], |s0, s1| {
                    s0.iter()
                        .zip(s1.iter())
                        .map(|(t1, t2)| max(t1, t2))
                        .copied()
                        .collect()
                })
                .iter()
                .product::<usize>();

        let ans = min(
            ans0,
            (0..ss.len())
                .map(|i| {
                    2 * max(ss.len() - 1, *ss[i].first().unwrap())
                        * ss.iter()
                            .enumerate()
                            .fold(vec![1; d / 2 + 1], |s0, (j, s1)| {
                                s0.iter()
                                    .zip(s1.iter().skip(if j == i { 1 } else { 0 }))
                                    .map(|(t1, t2)| max(t1, t2))
                                    .copied()
                                    .collect()
                            })
                            .iter()
                            .product::<usize>()
                })
                .min()
                .unwrap(),
        );
        println!("{} {}", d / 2 + 1, ans);
    } else {
        let c0 = (0..(d / 2)).fold(u, |u1, _| parents[u1]);
        let c1 = parents[c0];

        let mut subsize = vec![1; d / 2 + 3];
        g.get_subsize(&mut subsize, c1, c0, 0);
        g.get_subsize(&mut subsize, c0, c1, 0);

        assert!(*subsize.last().unwrap() == 1);

        let ans: usize = 2 * subsize.iter().product::<usize>();
        println!("{} {}", d / 2 + 1, ans);
    }
}
