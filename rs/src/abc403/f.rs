fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("1");
        return;
    }

    let mut dp = vec![[usize::MAX; 4], [1; 4]];

    for i in 2..=n {
        dp.push([usize::MAX; 4]);

        dp[i][3] = if i % 10 == 1 {
            dp[i / 10][3].saturating_add(1)
        } else {
            usize::MAX
        };

        dp[i][2] = dp[i][3];

        dp[i][1] = dp[i][2];
        for j in 2..i {
            if i % j == 0 {
                dp[i][1] = min(
                    dp[i][1],
                    dp[j][1].saturating_add(dp[i / j][2]).saturating_add(1),
                );
            }
        }

        dp[i][0] = dp[i][1];
        for j in 1..i {
            dp[i][0] = min(
                dp[i][0],
                dp[j][0].saturating_add(dp[i - j][1]).saturating_add(1),
            );
        }

        dp[i][2] = min(dp[i][2], dp[i][0].saturating_add(2));
        dp[i][1] = min(dp[i][1], dp[i][2]);
    }

    eprintln!("{}", dp[n][0]);
    println!("{}", restore(n, 0, &dp));
}

fn restore(i: usize, idx: usize, dp: &[[usize; 4]]) -> String {
    if i == 1 {
        return "1".to_string();
    }

    match idx {
        0 => {
            if dp[i][0] == dp[i][1] {
                restore(i, 1, dp)
            } else {
                (1..i)
                    .find(|&j| dp[i][0] == dp[j][0].saturating_add(dp[i - j][1]).saturating_add(1))
                    .map(|j| format!("{}+{}", restore(j, 0, dp), restore(i - j, 1, dp)))
                    .unwrap()
            }
        }
        1 => {
            if dp[i][1] == dp[i][2] {
                restore(i, 2, dp)
            } else {
                (2..i)
                    .filter(|&j| i % j == 0)
                    .find(|&j| dp[i][1] == dp[j][1].saturating_add(dp[i / j][2]).saturating_add(1))
                    .map(|j| format!("{}*{}", restore(j, 1, dp), restore(i / j, 2, dp)))
                    .unwrap()
            }
        }
        2 => {
            if dp[i][2] == dp[i][3] {
                restore(i, 3, dp)
            } else if dp[i][2] == dp[i][0] + 2 {
                format!("({})", restore(i, 0, dp))
            } else {
                unreachable!()
            }
        }
        3 => {
            if i % 10 == 1 {
                format!("{}1", restore(i / 10, 3, dp))
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
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
