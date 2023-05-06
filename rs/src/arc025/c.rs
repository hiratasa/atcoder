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
        // calculate before move out c
        let values = once(($($x),*));
        let mut c = $c;
        c.extend(values);
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
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

#[allow(dead_code)]
impl Edge {
    fn from_stdin() -> Edge {
        let (from, to, weight) = read_tuple!(usize, usize, usize);
        Edge {
            from: from - 1,
            to: to - 1,
            weight,
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

fn dijkstra(g: &Graph, src: usize) -> Vec<usize> {
    let n = g.out_edges.len();

    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];

    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;

    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }

        for &edge in &g.out_edges[v] {
            let next_cost = cost + edge.weight;

            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }

    costs
}

fn main() {
    let (n, m, r, t) = read_tuple!(usize, usize, usize, usize);

    let g = Graph::from_stdin_undirected(n, m);

    let ans = (0..n)
        .map(|a| {
            let costs = dijkstra(&g, a);

            let rcosts = costs.citer().fold(BTreeMap::new(), |hs, c| {
                let rc = r * c;
                inserted!(hs, rc, *hs.get(&rc).unwrap_or(&0) + 1)
            });

            let rcosts_sum =
                izip!(rcosts.keys(), rcosts.values().cumsum::<usize>()).collect::<BTreeMap<_, _>>();
            let e = (r < t) as usize;

            costs
                .citer()
                .map(|c| t * c)
                // .inspect(|tc| eprintln!("{} {}", a, tc))
                .filter_map(|tc| {
                    rcosts_sum
                        .range(..tc)
                        .next_back()
                        .map(|(_, &m)| m - /* origin */1 - e)
                })
                // .inspect(|m| eprintln!("=> {} {}", a, m))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", ans);
}
