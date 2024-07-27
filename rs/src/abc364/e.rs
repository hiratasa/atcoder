fn main() {
    input! {
        n: usize, x: usize, y: usize,
        ab: [(usize, usize); n],
    };

    let mut init = vec![vec![usize::MAX; n + 1]; x + 1];
    init[0][0] = 0;
    let dp = ab.iter().copied().fold(init, |prev, (a, b)| {
        let mut next = prev.clone();

        for i in 0..=x {
            for j in 0..=n {
                if i + a <= x && prev[i][j].saturating_add(b) <= y {
                    next[i + a][j + 1] = min(next[i + a][j + 1], prev[i][j] + b);
                }
            }
        }

        next
    });

    let ans = (0..=x)
        .filter_map(|i| dp[i].iter().rposition(|&b| b <= y))
        .max()
        .unwrap();
    let ans = n.min(ans + 1);

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
