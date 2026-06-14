fn main() {
    input! {
        t: usize,
        cases: [(Chars, usize); t],
    };

    cases
        .into_iter()
        .map(|(mut s, k)| {
            let c = s
                .iter()
                .copied()
                .tuple_windows::<(_, _, _)>()
                .filter(|&t| t == ('A', 'B', 'C'))
                .count();

            if c + k > s.len() / 3 {
                return None;
            }

            for i in 0..s.len() - 2 {
                if &s[i..i + 3] == &['A', 'B', 'C'] {
                    s[i] = 'a';
                    s[i + 1] = 'b';
                    s[i + 2] = 'c';
                }
            }

            let mut init = vec![vec![[1usize << 40; 3]; k + 1]; s.len() + 1];
            init[0][0][0] = 0;
            let dp = s.iter().copied().enumerate().fold(init, |mut dp, (i, c)| {
                if i + 1 < dp.len() {
                    for j in 0..=k {
                        dp[i + 1][j][0] = min(dp[i + 1][j][0], dp[i][j][0]);
                        dp[i + 1][j][2] = min(dp[i + 1][j][2], dp[i][j][1]);
                        dp[i + 1][j][0] = min(dp[i + 1][j][0], dp[i][j][2]);
                    }
                }

                if i + 2 < s.len() {
                    let cost = izip!(&['A', 'B', 'C'], &s[i..i + 3])
                        .filter(|&(c, d)| c != d)
                        .count();

                    for j in 0..=k {
                        if c == 'a' {
                            // no
                        } else if c == 'b' {
                            if i + 1 < s.len() && s[i + 1] == 'a' {
                                dp[i + 3][j][2] = dp[i][j][1] + cost;
                            } else if i + 2 < s.len() && s[i + 2] == 'a' {
                                dp[i + 3][j][1] = dp[i][j][1] + cost;
                            } else if j + 1 <= k {
                                dp[i + 3][j + 1][0] = dp[i][j][1] + cost;
                            }
                        } else if c == 'c' {
                            if i + 1 < s.len() && s[i + 1] == 'a' {
                                dp[i + 3][j][2] = dp[i][j][2] + cost;
                            } else if i + 2 < s.len() && s[i + 2] == 'a' {
                                dp[i + 3][j][1] = dp[i][j][2] + cost;
                            } else if j + 1 <= k {
                                dp[i + 3][j + 1][0] = dp[i][j][2] + cost;
                            }
                        } else {
                            if i + 1 < s.len() && s[i + 1] == 'a' {
                                dp[i + 3][j][2] = dp[i][j][0] + cost;
                            } else if i + 2 < s.len() && s[i + 2] == 'a' {
                                dp[i + 3][j][1] = dp[i][j][0] + cost;
                            } else if j + 1 <= k {
                                dp[i + 3][j + 1][0] = dp[i][j][0] + cost;
                            }
                        }
                    }
                }

                dp
            });

            Some(dp[s.len()][k][0])
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{ans}");
            } else {
                println!("-1");
            }
        });
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
