fn main() {
    input! {
        n: usize, k: usize,
        ab: [(i64, i64); n],
    };

    let mut init = vec![[i64::MIN; 2]; k + 1];
    init[0][0] = 0;
    init[1][1] = 0;
    let dp = ab.iter().copied().fold(init, |prev, (a, b)| {
        let mut next = vec![[i64::MIN; 2]; k + 1];

        for i in 0..=k {
            next[i][0] = max(next[i][0], prev[i][0] + a);
            next[i][1] = max(next[i][1], prev[i][1] + b);

            if i < k {
                next[i + 1][1] = max(next[i + 1][1], prev[i][0] + b);
            }
            next[i][0] = max(next[i][0], prev[i][1] + a);
        }

        next
    });

    let ans = (0..=k).map(|i| max(dp[i][0], dp[i][1])).max().unwrap();
    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
