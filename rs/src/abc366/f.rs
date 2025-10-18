fn main() {
    input! {
        n: usize, k: usize,
        mut ab: [(i64, i64); n],
    };

    ab.sort_by(|&(a0, b0), &(a1, b1)| (a0 * b1 + b0).cmp(&(a1 * b0 + b1)));

    let mut init = vec![i64::MIN; k + 1];
    init[0] = 1;
    let dp = ab.into_iter().fold(init, |mut dp, (a, b)| {
        for i in (0..k).rev() {
            dp[i + 1] = max(dp[i + 1], dp[i].saturating_mul(a).saturating_add(b));
        }

        dp
    });

    let ans = dp[k];

    println!("{ans}");
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
