fn main() {
    // input! {
    //     n: usize,
    //     ka: [(usize, usize); n],
    // };

    let mut rng = SmallRng::seed_from_u64(42);

    for _ in 0.. {
        let n = rng.random_range(6..=8);
        let ka = repeat_with(|| (rng.random_range(1..=6), rng.random_range(1..10)))
            .take(n)
            .collect::<Vec<_>>();

        let ans = solve(&ka);
        let ans0 = solve0(&ka);

        assert_eq!(
            ans,
            ans0,
            "{n}\n{}",
            ka.iter().map(|(k, a)| format!("{k} {a}")).join("\n")
        );

        if let Some(ans) = ans {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }
}

fn solve(ka: &[(usize, usize)]) -> Option<usize> {
    let dp = ka.iter().copied().fold(
        vec![vec![]; 6],
        |mut dp: Vec<Vec<(usize, [usize; 3])>>, (k, a)| {
            for i in (1..6).rev() {
                let r = [0, 1, 2, 3, 2, 3][i];
                let mut t = take(&mut dp[i]);
                t.extend(
                    dp[i - 1]
                        .iter()
                        .copied()
                        .filter(|&(_, xyz)| xyz[3 - r..].iter().copied().all(|t| t != k))
                        .map(|(b, [_x, y, z])| (a + b, [y, z, k])),
                );
                dp[i] = t;
                if !dp[i].is_empty() {
                    let idx = dp[i]
                        .iter()
                        .copied()
                        .position_max_by_key(|&(b, _)| b)
                        .unwrap();
                    dp[i].swap(0, idx);
                }
                if dp[i].len() > 13 {
                    let xyz = dp[i][0].1;
                    for j in 0..3 {
                        for l in 0..4 {
                            let idx = dp[i][j * 4 + l + 1..]
                                .iter()
                                .copied()
                                .position_max_by_key(
                                    |&(b, t)| if t.contains(&xyz[j]) { 0 } else { b },
                                )
                                .unwrap();
                            dp[i][j * 4 + l + 1..].swap(0, idx);
                        }
                    }
                }
                dp[i].truncate(13);
            }

            {
                dp[0].push((a, [0, 0, k]));
                dp[0].sort_by_key(|&(b, _)| Reverse(b));
                dp[0].truncate(4);
            }

            // eprintln!("{k} {a}: {dp:?}");

            dp
        },
    );

    dp[5].get(0).copied().map(|t| t.0)
}

fn solve0(ka: &[(usize, usize)]) -> Option<usize> {
    let n = ka.len();

    ka.iter()
        .copied()
        .tuple_combinations()
        .filter(|&(a, b, c, d, e, f)| {
            let t = [a.0, b.0, c.0, d.0, e.0, f.0];
            let t0 = &t[0..4];
            let t1 = &t[2..6];
            t0.iter().copied().tuple_combinations().all(|(x, y)| x != y)
                && t1.iter().copied().tuple_combinations().all(|(x, y)| x != y)
        })
        .map(|(a, b, c, d, e, f)| a.1 + b.1 + c.1 + d.1 + e.1 + f.1)
        .max()
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
