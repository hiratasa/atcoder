#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct BipartiteMatching {
    n: usize,
    m: usize,
    adjs: Vec<Vec<usize>>,
    pair: Vec<Option<usize>>,
    pair2: Vec<Option<usize>>,
}

#[allow(dead_code)]
impl BipartiteMatching {
    fn new(n: usize, m: usize) -> BipartiteMatching {
        BipartiteMatching {
            n,
            m,
            adjs: vec![vec![]; n],
            pair: vec![None; n],
            pair2: vec![None; m],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adjs[from].push(to);
    }

    fn bfs(&self) -> Option<(Vec<usize>, Vec<usize>)> {
        let mut q = std::collections::VecDeque::new();
        let mut costs = vec![usize::MAX; self.n];
        let mut costs2 = vec![usize::MAX; self.m];

        for i in 0..self.n {
            if self.pair[i].is_none() {
                q.push_back(i);
                costs[i] = 0;
            }
        }

        let mut found = false;
        while let Some(v) = q.pop_front() {
            let c = costs[v];
            self.adjs[v].citer().for_each(|u| {
                if costs2[u] <= c + 1 {
                    return;
                }

                costs2[u] = c + 1;

                if let Some(w) = self.pair2[u] {
                    assert!(w != v);

                    if costs[w] > c + 2 {
                        costs[w] = c + 2;
                        q.push_back(w);
                    }
                } else {
                    found = true;
                }
            });
        }

        if found { Some((costs, costs2)) } else { None }
    }

    fn dfs(&mut self, v: usize, levels: &[usize], levels2: &[usize], itrs: &mut [usize]) -> bool {
        for i in itrs[v]..self.adjs[v].len() {
            let u = self.adjs[v][i];
            if levels[v] + 1 == levels2[u] {
                assert!(self.pair[v] != Some(u));
                let ok = if let Some(w) = self.pair2[u] {
                    if levels[w] == levels[v] + 2 {
                        self.dfs(w, levels, levels2, itrs)
                    } else {
                        false
                    }
                } else {
                    true
                };

                if ok {
                    self.pair[v] = Some(u);
                    self.pair2[u] = Some(v);
                    return true;
                }
            }
            itrs[v] += 1;
        }

        false
    }

    fn max_flow(&mut self) -> usize {
        let mut total_flow = 0;
        loop {
            if let Some((levels, levels2)) = self.bfs() {
                let mut itrs = vec![0; self.n];
                for v in 0..self.n {
                    if self.pair[v].is_none() && self.dfs(v, &levels, &levels2, &mut itrs) {
                        total_flow += 1;
                    }
                }
            } else {
                break;
            }
        }

        total_flow
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let mut a = read_mat::<usize>(n);

    for w in (2..=m).rev() {
        let mut matching = BipartiteMatching::new(n, n);

        for i in 0..n {
            a[i][..w]
                .citer()
                .scan(vec![false; n + 1], |seen, x| {
                    if seen[x] {
                        Some(None)
                    } else {
                        seen[x] = true;
                        Some(Some(x))
                    }
                })
                .flatten()
                .for_each(|x| {
                    matching.add_edge(i, x - 1);
                });
        }

        let f = matching.max_flow();

        if f < n {
            println!("No");
            return;
        }

        for i in 0..n {
            let x = matching.pair[i].unwrap() + 1;

            let j = a[i][..w].citer().position(|y| y == x).unwrap();

            a[i].swap(j, w - 1);
        }
    }

    println!("Yes");
    for row in a {
        println!("{}", row.citer().join(" "));
    }
}
