fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            x: [i64; n],
            mut y: [i64; n - 1],
        };
        y.insert(0, 0);

        let dp = (0..n).fold([0, -(1 << 50)], |prev, i| {
            if s[i] == 'S' {
                [
                    max(prev[0], prev[1] + y[i]),
                    max(prev[0] - x[i], prev[1] - x[i]),
                ]
            } else {
                [
                    max(prev[0] - x[i], prev[1] + y[i] - x[i]),
                    max(prev[0], prev[1]),
                ]
            }
        });

        let ans = max(dp[0], dp[1]);
        println!("{ans}");
    }
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
