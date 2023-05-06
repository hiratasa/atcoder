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
    let (n, q) = read_cols!(usize, usize);

    let c = read_vec::<usize>();

    let mut nums = c
        .iter()
        .map(|&cc| {
            let mut map = FxHashMap::default();
            map.insert(cc - 1, 1);
            map
        })
        .collect::<Vec<_>>();
    let mut uf = UnionFind::new(n);
    (0..q).for_each(|_| {
        let (t, a, b) = read_cols!(usize, usize, usize);
        let a = a - 1;
        let b = b - 1;

        if t == 1 {
            let pa = uf.root(a);
            let pb = uf.root(b);

            if pa != pb {
                uf.unite(a, b);
                assert!(pb == uf.root(a));
                assert!(pb == uf.root(b));

                if nums[pa].len() > nums[pb].len() {
                    nums.swap(pa, pb);
                }

                let tmp = std::mem::replace(&mut nums[pa], FxHashMap::default());
                for (cc, mm) in tmp {
                    *nums[pb].entry(cc).or_insert(0) += mm;
                }
            }
        } else {
            let pa = uf.root(a);

            println!("{}", nums[pa].get(&b).copied().unwrap_or(0));
        }
    })
}
