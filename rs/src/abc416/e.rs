fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
        k: usize, t: usize,
        d: [Usize1; k],
        q: usize,
    };

    let mut dist = vec![vec![usize::MAX; n + 1]; n + 1];
    for (a, b, c) in abc {
        dist[a][b] = min(dist[a][b], 2 * c);
        dist[b][a] = min(dist[b][a], 2 * c);
    }
    for x in d {
        dist[n][x] = t;
        dist[x][n] = t;
    }
    for i in 0..=n {
        dist[i][i] = 0;
    }
    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                dist[y][z] = min(dist[y][z], dist[y][x].saturating_add(dist[x][z]));
            }
        }
    }
    let sum = iproduct!(0..n, 0..n)
        .map(|(i, j)| dist[i][j])
        .map(|d| if d == usize::MAX { 0 } else { d / 2 })
        .sum::<usize>();

    (0..q)
        .scan(sum, |sum, _| {
            input! {
                ty: usize,
            };

            let mut update = |x: usize, y: usize, w: usize, sum: &mut usize| {
                for i in 0..=n {
                    for j in 0..=n {
                        let d = min(
                            dist[i][x].saturating_add(w).saturating_add(dist[y][j]),
                            dist[i][y].saturating_add(w).saturating_add(dist[x][j]),
                        );
                        if d < dist[i][j] {
                            if i < n && j < n {
                                if dist[i][j] == usize::MAX {
                                    *sum += d / 2;
                                } else {
                                    *sum -= (dist[i][j] - d) / 2;
                                }
                            }
                            dist[i][j] = d;
                        }
                    }
                }
            };

            if ty == 1 {
                input! {
                    x: Usize1, y: Usize1, c: usize,
                };
                update(x, y, 2 * c, sum);

                Some(None)
            } else if ty == 2 {
                input! {
                    x: Usize1,
                };

                update(n, x, t, sum);

                Some(None)
            } else {
                Some(Some(*sum))
            }
        })
        .flatten()
        .for_each(|ans| {
            println!("{ans}");
        })
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
