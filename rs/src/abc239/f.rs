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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
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
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
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

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
struct UnionFind {
    g: Vec<UnionFindNode>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;
        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
        }
    }
    fn root(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let p = match self.g[v] {
            Root { size: _ } => return v,
            Child { parent: p } => p,
        };
        let r = self.root(p);
        self.g[v] = Child { parent: r };
        r
    }
    fn unite(&mut self, v: usize, u: usize) -> bool {
        use UnionFindNode::*;
        let rv = self.root(v);
        let ru = self.root(u);
        if rv == ru {
            return false;
        }
        let size_rv = self.size(rv);
        let size_ru = self.size(ru);
        let (rsmall, rlarge) = if size_rv < size_ru {
            (rv, ru)
        } else {
            (ru, rv)
        };
        self.g[rsmall] = Child { parent: rlarge };
        self.g[rlarge] = Root {
            size: size_rv + size_ru,
        };
        true
    }
    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let rv = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _ } => unreachable!(),
        }
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let d = read_row::<usize>();
    let ab = read_vec(m, || read_tuple!(usize, usize));

    let mut uf =
        ab.citer()
            .map(|(a, b)| (a - 1, b - 1))
            .fold(UnionFind::new(n), |mut uf, (a, b)| {
                uf.unite(a, b);
                uf
            });

    let d = if let Some(d) = ab
        .citer()
        .map(|(a, b)| (a - 1, b - 1))
        .try_fold(d, |mut d, (a, b)| {
            if d[a] == 0 {
                None
            } else if d[b] == 0 {
                None
            } else {
                d[a] -= 1;
                d[b] -= 1;
                Some(d)
            }
        }) {
        d
    } else {
        println!("-1");
        return;
    };

    if d.citer().sum::<usize>() != 2 * (n - m - 1) {
        println!("-1");
        return;
    }
    let (t, mut vs) = (0..n).fold(
        (FxHashMap::default(), FxHashMap::default()),
        |(mut map, mut map2), i| {
            *map.entry(uf.root(i)).or_insert(0) += d[i];
            map2.entry(uf.root(i)).or_insert(vec![]).push(i);
            (map, map2)
        },
    );

    if t.len() != n - m {
        println!("-1");
        return;
    }

    if t.values().any(|&x| x == 0) {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut q = BinaryHeap::from_iter(t.iter().map(|(&x, &y)| (y, x)));
    let mut d = d;
    while q.len() > 1 {
        let (y0, x0) = q.pop().unwrap();
        let (y1, x1) = q.pop().unwrap();

        assert!(y0 > 0);
        assert!(y1 > 0);

        let mut v0 = vs.remove(&x0).unwrap();
        let mut v1 = vs.remove(&x1).unwrap();

        while matches!(v0.last(), Some(&x) if d[x] == 0) {
            v0.pop();
        }

        while matches!(v1.last(), Some(&x) if d[x] == 0) {
            v1.pop();
        }

        assert!(!v0.is_empty());
        assert!(!v1.is_empty());

        ans.push((*v0.last().unwrap(), *v1.last().unwrap()));
        d[ans[ans.len() - 1].0] -= 1;
        d[ans[ans.len() - 1].1] -= 1;

        let v = if v0.len() < v1.len() {
            v1.extend(v0);
            v1
        } else {
            v0.extend(v1);
            v0
        };

        q.push((y0 + y1 - 2, x0));
        vs.insert(x0, v);
    }

    assert!(ans.len() == n - m - 1, "{:?}", ans);

    for (x, y) in ans {
        println!("{} {}", x + 1, y + 1);
    }
}
