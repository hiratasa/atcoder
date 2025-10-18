fn main() {
    input! {
        n: usize, m: usize,
        uvt: [(Usize1, Usize1, usize); m],
        q: usize,
        b: [[Usize1]; q],
    };

    let mut dists = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        dists[i][i] = 0;
    }
    for &(u, v, t) in &uvt {
        dists[u][v] = min(dists[u][v], t);
        dists[v][u] = min(dists[v][u], t);
    }
    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                dists[i][k] = min(dists[i][k], dists[i][j].saturating_add(dists[j][k]));
            }
        }
    }

    b.into_iter()
        .map(|b| {
            let k = b.len();
            let sum = b.iter().map(|&i| uvt[i].2).sum::<usize>();
            sum + iproduct!(0..1 << k, (0..k).permutations(k))
                .map(|(s, perm)| {
                    let start = |i: usize| {
                        if s & (1 << i) == 0 {
                            uvt[b[perm[i]]].0
                        } else {
                            uvt[b[perm[i]]].1
                        }
                    };
                    let end = |i: usize| {
                        if s & (1 << i) == 0 {
                            uvt[b[perm[i]]].1
                        } else {
                            uvt[b[perm[i]]].0
                        }
                    };

                    dists[0][start(0)]
                        + (0..k)
                            .tuple_windows()
                            .map(|(i, j)| dists[end(i)][start(j)])
                            .sum::<usize>()
                        + dists[end(k - 1)][n - 1]
                })
                .min()
                .unwrap()
        })
        .for_each(|ans| {
            println!("{ans}");
        })
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
