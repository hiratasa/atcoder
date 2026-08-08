fn main() {
    input! {
        n: usize, l: usize,
        a: [usize; n],
    };

    let mut init = vec![vec![vec![vec![0.0; l + 1]; n + 1]; 2]; n + 1];
    init[0][0][0][0] = 1.0;
    let dp = iproduct!(0..n, 0..2, 0..=n, 0..l).fold(
        init,
        |mut dp, (removed, has_pair, memory, damage)| {
            let remain = 2 * n - 2 * removed;
            let p = dp[removed][has_pair][memory][damage];

            if p == 0.0 {
                return dp;
            }

            let unknown_cards = remain - memory;

            if has_pair == 1 {
                if memory >= 2 {
                    dp[removed + 1][0][memory - 2][damage] += p;
                }
            } else if memory <= unknown_cards {
                if unknown_cards > 0 && memory > 0 {
                    dp[removed + 1][has_pair][memory - 1][damage] +=
                        p * memory as f64 / unknown_cards as f64;
                }
                if unknown_cards > 1 && unknown_cards > memory {
                    dp[removed + 1][has_pair][memory][damage] += p
                        * (unknown_cards - memory) as f64
                        / unknown_cards as f64
                        / (unknown_cards - 1) as f64;
                    if memory + 2 <= n {
                        dp[removed][1][memory + 2][damage + 1] +=
                            p * (unknown_cards - memory) as f64 / unknown_cards as f64
                                * memory as f64
                                / (unknown_cards - 1) as f64;
                        dp[removed][has_pair][memory + 2][damage + 1] +=
                            p * (unknown_cards - memory) as f64 / unknown_cards as f64
                                * (unknown_cards - memory - 2) as f64
                                / (unknown_cards - 1) as f64;
                    }
                }
            }

            dp
        },
    );

    let mut init = vec![(0.0, 0.0); n + 1];
    init[0] = (1.0, 0.0);
    let e = a.iter().copied().fold(init, |mut e, x| {
        for i in (0..n).rev() {
            e[i + 1].0 += e[i].0;
            e[i + 1].1 += e[i].1 + e[i].0 * x as f64
        }
        e
    });

    let ans0 = dp[n][0][0].iter().copied().sum::<f64>() * (e[n].1 / e[n].0);
    let ans1 = iproduct!(0..n, 0..2, 0..=n)
        .map(|(i, j, k)| dp[i][j][k][l] * e[i].1 / e[i].0)
        .sum::<f64>();

    let ans = ans0 + ans1;

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
