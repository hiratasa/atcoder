fn main() {
    input! {
        n: usize,
        edges_g: [(Usize1, Usize1)],
        edges_h: [(Usize1, Usize1)],
        costs: [usize; n * (n - 1) / 2],
    };

    let adjs_g = edges_g
        .into_iter()
        .fold(vec![vec![false; n]; n], |mut adjs, (u, v)| {
            adjs[u][v] = true;
            adjs[v][u] = true;
            adjs
        });
    let adjs_h = edges_h
        .into_iter()
        .fold(vec![vec![false; n]; n], |mut adjs, (u, v)| {
            adjs[u][v] = true;
            adjs[v][u] = true;
            adjs
        });

    let ans = (0..n)
        .permutations(n)
        .map(|perm| {
            (0..n)
                .tuple_combinations()
                .filter(|&(u, v)| adjs_h[u][v] != adjs_g[perm[u]][perm[v]])
                .map(|(u, v)| costs[(2 * n - 1 - u) * u / 2 + v - (u + 1)])
                .sum::<usize>()
        })
        .min()
        .unwrap();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
