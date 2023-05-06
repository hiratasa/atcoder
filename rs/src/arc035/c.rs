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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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
        let mut c = $c;
        c.push($x);
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

#[allow(dead_code)]
struct UndirectedGraph {
    adjs: Vec<Vec<(usize, usize)>>,
}
#[allow(dead_code)]
impl UndirectedGraph {
    fn from_stdin(n: usize, m: usize) -> UndirectedGraph {
        let mut adjs = vec![vec![]; n];
        for _ in 0..m {
            let (u, v, c) = read_tuple!(usize, usize, usize);
            adjs[u - 1].push((v - 1, c));
            adjs[v - 1].push((u - 1, c));
        }
        UndirectedGraph { adjs }
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let g = UndirectedGraph::from_stdin(n, m);

    let k: usize = read();

    let xyz = read_vec(k, || read_tuple!(usize, usize, usize));

    let costs = (0..n)
        .map(|s| {
            let mut q = BinaryHeap::new();
            let mut costs = vec![usize::MAX; n];

            q.push(Reverse((0, s)));
            costs[s] = 0;

            while let Some(Reverse((cost, v))) = q.pop() {
                if cost > costs[v] {
                    continue;
                }

                g.adjs[v].citer().for_each(|(u, c)| {
                    if cost + c < costs[u] {
                        q.push(Reverse((cost + c, u)));
                        costs[u] = cost + c;
                    }
                });
            }

            costs
        })
        .collect_vec();

    xyz.citer()
        .scan(costs, |costs, (x, y, z)| {
            let x = x - 1;
            let y = y - 1;

            *costs = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| {
                            it!(
                                costs[i][j],
                                costs[i][x] + z + costs[y][j],
                                costs[i][y] + z + costs[x][j]
                            )
                            .min()
                            .unwrap()
                        })
                        .collect_vec()
                })
                .collect_vec();

            Some(
                (0..n)
                    .tuple_combinations()
                    .map(|(i, j)| costs[i][j])
                    .sum::<usize>(),
            )
        })
        .for_each(|ans| println!("{}", ans));
}
