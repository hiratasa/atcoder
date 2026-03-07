fn main() {
    input! {
        s: Chars,
    };

    let dp = s.into_iter().fold([0, 0, 0], |mut dp, c| {
        if c == 'A' {
            dp[0] += 1;
        } else if dp[0] > 0 && c == 'B' {
            dp[0] -= 1;
            dp[1] += 1;
        } else if dp[1] > 0 && c == 'C' {
            dp[1] -= 1;
            dp[2] += 1;
        }
        dp
    });
    let ans = dp[2];

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
