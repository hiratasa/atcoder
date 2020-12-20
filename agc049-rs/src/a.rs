#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
}

struct Graph {
    out_edges: Vec<Vec<Edge>>,
    in_edges: Vec<Vec<Edge>>,
}

impl Graph {
    #[allow(dead_code)]
    fn new(n: usize) -> Graph {
        Graph {
            out_edges: vec![vec![]; n],
            in_edges: vec![vec![]; n],
        }
    }

    #[allow(dead_code)]
    fn add_edge(&mut self, from: usize, to: usize) {
        self.out_edges[from].push(Edge { from, to });
        self.in_edges[to].push(Edge { from, to });
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.out_edges.len()
    }

    #[allow(dead_code)]
    fn get_out_edges(&self, from: usize) -> &Vec<Edge> {
        &self.out_edges[from]
    }

    #[allow(dead_code)]
    fn get_in_edges(&self, to: usize) -> &Vec<Edge> {
        &self.in_edges[to]
    }
}

#[allow(dead_code)]
fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;

    for edge in g.get_out_edges(v) {
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
    idxs: &mut Vec<Option<usize>>,
    vs: &mut Vec<usize>,
    idx: usize,
    parents: &mut BTreeSet<usize>,
) {
    idxs[v] = Some(idx);
    vs.push(v);

    for edge in g.get_in_edges(v) {
        if let Some(parent_idx) = idxs[edge.from] {
            if parent_idx != idx {
                parents.insert(parent_idx);
            }
        } else {
            rev_dfs(g, edge.from, idxs, vs, idx, parents);
        }
    }
}

// 強連結成分分解
#[allow(dead_code)]
fn scc(g: &Graph) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut vs = vec![];
    {
        let mut visited = vec![false; g.size()];
        for v in 0..g.size() {
            if !visited[v] {
                dfs(g, v, &mut visited, &mut vs);
            }
        }
    }

    let mut sizes = vec![];
    let mut parents = vec![];
    {
        let mut idxs = vec![None; g.size()];
        let mut idx = 0;
        for &v in vs.iter().rev() {
            if idxs[v].is_none() {
                let mut component = vec![];
                parents.push(BTreeSet::new());
                rev_dfs(g, v, &mut idxs, &mut component, idx, &mut parents[idx]);
                idx += 1;
                sizes.push(component.len());
            }
        }
    }

    let mut ret_g = vec![vec![]; parents.len()];

    for (i, ps) in parents.iter().enumerate() {
        for &p in ps {
            assert!(p < i);
            ret_g[i].push(p);
        }
    }

    (ret_g, sizes)
}

fn calc(radjs: &Vec<Vec<usize>>, sizes: &Vec<usize>, visited: &mut Vec<bool>, v: usize) -> usize {
    visited[v] = true;
    let mut ret = 0;
    for &u in &radjs[v] {
        if visited[u] {
            continue;
        }

        ret += sizes[u];
        ret += calc(radjs, sizes, visited, u);
    }

    ret
}

fn main() {
    let n: usize = read();

    let g = {
        let mut g = Graph::new(n);

        for i in 0..n {
            let s: String = read();

            for (j, _) in s.chars().enumerate().filter(|&(_, c)| c == '1') {
                g.add_edge(i, j);
            }
        }

        g
    };

    let (radjs, sizes) = scc(&g);

    let mut ans = 0.0;
    for i in 0..sizes.len() {
        let mut visited = vec![false; sizes.len()];
        let s = calc(&radjs, &sizes, &mut visited, i);

        ans += sizes[i] as f64 / (sizes[i] as f64 + s as f64);
    }

    println!("{:.20}", ans);
}
