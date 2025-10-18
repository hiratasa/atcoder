fn main() {
    input! {
        n: usize,
        ab: [(Usize1, usize); n],
    };

    let s = ab.iter().map(|&(_, b)| b).sum::<usize>();

    if s % 3 != 0 {
        println!("-1");
        return;
    }

    let t = s / 3;

    let mut init = vec![vec![usize::MAX; t + 1]; t + 1];
    init[0][0] = 0;
    let dp = ab.iter().fold(init, |prev, &(a, b)| {
        let mut next = vec![vec![usize::MAX; t + 1]; t + 1];

        for i in 0..3 {
            let c = if i == a { 0 } else { 1 };

            for j in 0..=t {
                for k in 0..=t {
                    let next_j = if i == 0 { j + b } else { j };

                    let next_k = if i == 1 { k + b } else { k };

                    if next_j <= t && next_k <= t {
                        next[next_j][next_k] =
                            min(next[next_j][next_k], prev[j][k].saturating_add(c));
                    }
                }
            }
        }

        next
    });

    if dp[t][t] != usize::MAX {
        println!("{}", dp[t][t]);
    } else {
        println!("-1");
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
