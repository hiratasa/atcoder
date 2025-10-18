fn main() {
    input! {
        n: usize, h: usize, m: usize,
        ab: [(usize, usize); n],
    };

    let mut init = vec![None; h + 1];
    init[h] = Some(m);
    let ans = ab
        .iter()
        .enumerate()
        .try_fold(init, |dp, (i, &(a, b))| {
            let next = (0..=h)
                .map(|j| {
                    if j + a <= h {
                        max(dp[j + a], dp[j].and_then(|x| x.checked_sub(b)))
                    } else {
                        dp[j].and_then(|x| x.checked_sub(b))
                    }
                })
                .collect::<Vec<_>>();

            if next.iter().any(Option::is_some) {
                Ok(next)
            } else {
                Err(i)
            }
        })
        .map_or_else(|i| i, |_| n);

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
