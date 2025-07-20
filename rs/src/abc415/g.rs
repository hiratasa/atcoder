fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    };

    const K: usize = 100000;
    let ab = ab
        .into_iter()
        .sorted_by_key(|&(a, b)| (b, a))
        .dedup_by(|&(a1, b1), &(a2, b2)| b1 == b2)
        .collect::<Vec<_>>();
    let (a0, b0) = ab
        .iter()
        .copied()
        .max_by(|(a1, b1), (a2, b2)| ((a2 - b2) * b1).cmp(&((a1 - b1) * b2)))
        .unwrap();

    let dp = ab.iter().fold(vec![0; K], |mut dp, &(a, b)| {
        let c = a - b;

        for i in b..K - c {
            dp[i + c] = max(dp[i + c], dp[i] + b);
        }

        dp
    });

    let ans = (0..min(K, n + 1))
        .map(|i| {
            if i < a0 {
                dp[i] + n
            } else {
                dp[i] + (n - i) / (a0 - b0) * b0 + n
            }
        })
        .max()
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
