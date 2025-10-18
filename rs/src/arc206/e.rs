fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            u: [usize; n - 2],
            d: [usize; n - 2],
            l: [usize; n - 2],
            r: [usize; n - 2],
        };

        let dp = [(&u, &d), (&l, &r)]
            .into_iter()
            .map(|(u, d)| {
                let mut init = vec![vec![vec![usize::MAX; 3]; 3]; 3];
                init[0][0][0] = 0;
                (0..n - 2).fold(init, |prev, i| {
                    let mut next = vec![vec![vec![usize::MAX; 3]; 3]; 3];
                    for s in 0..3 {
                        for t in 0..3 {
                            for k in 0..3 {
                                for s2 in 0..2 {
                                    for t2 in 0..2 {
                                        if s + s2 > 2 || t + t2 > 2 {
                                            continue;
                                        }

                                        let k2 = if k == 0 {
                                            if (s + s2 == 2 && t + t2 == 0)
                                                || (s + s2 == 0 && t + t2 == 2)
                                            {
                                                2
                                            } else if s + s2 > 0 && t + t2 > 0 {
                                                1
                                            } else {
                                                0
                                            }
                                        } else {
                                            k
                                        };

                                        next[s + s2][t + t2][k2] = next[s + s2][t + t2][k2].min(
                                            prev[s][t][k].saturating_add(
                                                (s2 > 0) as usize * u[i] + (t2 > 0) as usize * d[i],
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    next
                })
            })
            .collect::<Vec<_>>();

        let ans = min(
            min(
                dp[0][2][2][1].saturating_add(dp[1][2][2][2]),
                dp[0][2][2][2].saturating_add(dp[1][2][2][1]),
            ),
            dp[0][2][2][1].saturating_add(dp[1][2][2][1]),
        );

        println!("{ans}");
    }
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
