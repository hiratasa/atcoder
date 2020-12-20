#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
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

struct UnionFind {
    g: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            g: (0..n).collect(),
        }
    }

    fn root(&mut self, v: usize) -> usize {
        if self.g[v] != v {
            self.g[v] = self.root(self.g[v]);
        }
        self.g[v]
    }

    fn unite(&mut self, v: usize, u: usize) {
        let rv = self.root(v);
        let ru = self.root(u);
        self.g[rv] = ru;
    }

    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
}

fn main() {
    let n: usize = read();
    let xy = (0..n).map(|_| read_cols!(i64, i64)).collect::<Vec<_>>();

    let edges = {
        let mut xs = xy
            .iter()
            .enumerate()
            .map(|(i, &(x, _))| (x, i))
            .collect::<Vec<_>>();
        xs.sort();

        let mut ys = xy
            .iter()
            .enumerate()
            .map(|(i, &(_, y))| (y, i))
            .collect::<Vec<_>>();
        ys.sort();

        let mut edges = vec![];

        xs.iter()
            .copied()
            .zip(xs.iter().copied().skip(1))
            .map(|((x1, i1), (x2, i2))| (x2 - x1, i1, i2))
            .for_each(|e| edges.push(e));

        ys.iter()
            .copied()
            .zip(ys.iter().copied().skip(1))
            .map(|((y1, i1), (y2, i2))| (y2 - y1, i1, i2))
            .for_each(|e| edges.push(e));

        edges.sort();

        edges
    };

    // eprintln!("{:?}", edges);

    let mut uf = UnionFind::new(n);

    let mut ans = 0;
    for &e in &edges {
        if !uf.same(e.1, e.2) {
            ans += e.0;
            uf.unite(e.1, e.2);

            // eprintln!("{:?}", e);
        }
    }

    println!("{}", ans);
}
