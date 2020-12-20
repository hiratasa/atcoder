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

const K: i64 = 10000000;

fn main() {
    let n: usize = read();
    let xy: Vec<_> = (0..n).map(|_| read_cols!(f64, f64)).collect();

    let range = (0..=K)
        .map(|i| 200.0 / K as f64 * i as f64)
        .collect::<Vec<_>>();

    let idx = range
        .binary_search_by(|&r| {
            let mut uf = UnionFind::new(n + 2);

            for (i, (x, y)) in xy.iter().enumerate() {
                if 2.0 * r > 100.0 - y {
                    uf.unite(i, n);
                }

                if 2.0 * r > y + 100.0 {
                    uf.unite(i, n + 1);
                }

                for (j, (x2, y2)) in xy.iter().enumerate().skip(i + 1) {
                    let dist = ((x - x2) * (x - x2) + (y - y2) * (y - y2)).sqrt();

                    if 2.0 * r > dist {
                        uf.unite(i, j);
                    }
                }
            }

            if uf.same(n, n + 1) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap_err();

    let r = range[idx - 1];

    println!("{:.10}", r);
}
