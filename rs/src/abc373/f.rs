fn main() {
    input! {
        n: usize, w: usize,
        wv: [(usize, usize); n],
    };

    let mut items_by_w = wv
        .into_iter()
        .fold(vec![vec![]; w + 1], |mut items, (ww, v)| {
            // v - 2 * i + 1
            items[ww].extend((1..=w / ww).filter_map(|i| (v + 1).checked_sub(2 * i)));

            items
        });

    for ww in 1..=w {
        items_by_w[ww].sort();
        items_by_w[ww].reverse();
        items_by_w[ww].truncate(w / ww);
    }

    let dp = items_by_w
        .into_iter()
        .enumerate()
        .flat_map(|(ww, items)| items.into_iter().map(move |v| (ww, v)))
        .fold(vec![0; w + 1], |mut dp, (ww, v)| {
            for i in (0..=w - ww).rev() {
                dp[i + ww] = max(dp[i + ww], dp[i] + v);
            }

            dp
        });

    let ans = dp[w];

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
