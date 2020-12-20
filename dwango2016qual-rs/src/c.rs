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
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

use itertools_num::ItertoolsNum;

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

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
    terminal: usize,
    terminal_cost: usize,
}

#[allow(dead_code)]
fn dijkstra(edges: &Vec<Vec<Edge>>, src: usize) -> Vec<usize> {
    let n = edges.len();
    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];
    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;
    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }
        for &edge in &edges[v] {
            let next_cost = cost + edge.weight;
            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }
    costs
}

#[allow(dead_code)]
fn dijkstra1(
    edges: &Vec<Vec<Edge>>,
    src: usize,
    dst: usize,
    dst_costs: &Vec<usize>,
    upper_cost: usize,
) -> Option<usize> {
    let n = edges.len();
    let mut q = std::collections::BinaryHeap::new();
    let mut costs = vec![std::usize::MAX; n];
    q.push(std::cmp::Reverse((0, src)));
    costs[src] = 0;
    while let Some(std::cmp::Reverse((cost, v))) = q.pop() {
        if cost > costs[v] {
            continue;
        }
        if v == dst {
            return Some(cost);
        }
        for &edge in &edges[v] {
            if cost + edge.weight + edge.terminal_cost + dst_costs[edge.terminal] > upper_cost {
                continue;
            }

            let next_cost = cost + edge.weight;
            if next_cost < costs[edge.to] {
                q.push(std::cmp::Reverse((next_cost, edge.to)));
                costs[edge.to] = next_cost;
            }
        }
    }
    None
}

trait SliceCopiedExt<T> {
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>>;
}

impl<V, T> SliceCopiedExt<T> for V
where
    V: std::ops::Deref<Target = [T]>,
    T: Copy,
{
    fn citer(&self) -> std::iter::Copied<std::slice::Iter<T>> {
        self.iter().copied()
    }
}

fn main() {
    let (n, m, src, dst) = read_tuple!(usize, usize, usize, usize);

    let mut edges = vec![vec![]; n];
    for _ in 0..m {
        let _l: usize = read();

        let s = read_row::<usize>();
        let w = read_row::<usize>();
        let cumw = w.iter().cumsum::<usize>().collect_vec();
        let totalw = *cumw.last().unwrap();

        izip!(s.citer().tuple_windows(), w.citer(), cumw.citer())
            .map(|((s0, s1), weight, cumweight)| Edge {
                from: s0,
                to: s1,
                weight,
                terminal: *s.last().unwrap(),
                terminal_cost: totalw - cumweight,
            })
            .for_each(|e| edges[e.from].push(e));

        izip!(
            s.citer().rev().tuple_windows(),
            w.citer().rev(),
            cumw.citer().rev(),
        )
        .map(|((s0, s1), weight, cumweight)| Edge {
            from: s0,
            to: s1,
            weight,
            terminal: *s.first().unwrap(),
            terminal_cost: cumweight - weight,
        })
        .for_each(|e| edges[e.from].push(e));
    }

    let dst_costs = dijkstra(&edges, dst);

    let ans = lower_bound_int(0, 1000000000, |upper_cost| {
        if let Some(cost) = dijkstra1(&edges, src, dst, &dst_costs, upper_cost) {
            assert!(cost <= upper_cost);
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("{}", ans);
}
