#[allow(unused_imports)]
use itertools::chain;
#[allow(unused_imports)]
use itertools::zip;
#[allow(unused_imports)]
use itertools::Itertools;
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
macro_rules! read_tuple {
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
    let (n, q) = read_tuple!(usize, usize);

    let mut uf = UnionFind::new(2 * n);
    for _ in 0..q {
        let (w, x, y, z) = read_tuple!(usize, usize, usize, usize);
        let x = x - 1;
        let y = y - 1;

        if w == 1 {
            let zz = z % 2;

            uf.unite(x, y + zz * n);
            uf.unite(x + n, y + (zz + 1) % 2 * n);
        } else {
            if uf.same(x, y) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
