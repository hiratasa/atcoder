fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let set = ab
        .into_iter()
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<FxHashSet<_>>();

    let dp = (1usize..1 << n).fold(vec![0; 1 << n], |mut dp, s| {
        let idxs = (0..n).filter(|&i| (s & (1 << i)) != 0).collect::<Vec<_>>();
        let k = idxs.len();

        let c0 = (0..k)
            .permutations(k)
            .map(|perm| {
                perm.into_iter()
                    .cycle()
                    .tuple_windows()
                    .take(k)
                    .map(|(i, j)| (idxs[i], idxs[j]))
                    .map(|(a, b)| (min(a, b), max(a, b)))
                    .filter(|&(a, b)| set.contains(&(a, b)))
                    .count()
            })
            .max()
            .unwrap();

        dp[s] = successors(Some(s), |&t| t.checked_sub(1).map(|t| t & s))
            .filter(|&t| t.count_ones() >= 3 && (s ^ t).count_ones() >= 3)
            .map(|t| {
                let u = s ^ t;

                dp[u] + dp[t]
            })
            .chain(once(c0))
            .max()
            .unwrap();

        dp
    });

    // for i in 0..(1 << n) {
    //     eprintln!("{i:08b} => {}", dp[i]);
    // }

    let ans = n + m - 2 * dp[(1 << n) - 1];

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
