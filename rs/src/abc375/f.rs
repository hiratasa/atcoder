fn main() {
    input! {
        n: usize, m: usize, q: usize,
        abc: [(Usize1, Usize1, usize); m],
        queries: [[Usize1]; q],
    };

    let enabled = queries.iter().fold(vec![true; m], |mut enabled, query| {
        if query.len() == 1 {
            enabled[query[0]] = false;
        }
        enabled
    });

    let mut costs = vec![vec![usize::MAX; n]; n];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        if enabled[i] {
            costs[a][b] = c;
            costs[b][a] = c;
        }
    }
    for i in 0..n {
        costs[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                costs[j][k] = min(costs[j][k], costs[j][i].saturating_add(costs[i][k]));
            }
        }
    }

    let mut ans = queries
        .iter()
        .rev()
        .scan(costs, |costs, query| {
            if query.len() == 1 {
                let (a, b, c) = abc[query[0]];

                for i in 0..n {
                    for j in 0..n {
                        costs[i][j] = min(
                            costs[i][j],
                            costs[i][a].saturating_add(c).saturating_add(costs[b][j]),
                        );
                        costs[i][j] = min(
                            costs[i][j],
                            costs[i][b].saturating_add(c).saturating_add(costs[a][j]),
                        );
                    }
                }

                Some(None)
            } else {
                Some(Some(costs[query[0]][query[1]]))
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    ans.reverse();

    for x in ans {
        if x == usize::MAX {
            println!("-1");
        } else {
            println!("{x}");
        }
    }
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
