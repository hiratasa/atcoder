fn main() {
    input! {
        n: usize, t: usize, m: usize, k: usize,
    };

    let mut patterns = vec![];
    enumerate_patterns(m, 1, &mut vec![], &mut patterns);

    let dp = iproduct!(0..=t, 0..=k).fold(vec![vec![0.0; k + 1]; t + 1], |mut dp, (t, k)| {
        dp[t][k] = if k == 0 {
            1.0
        } else if k > t * m {
            0.0
        } else {
            patterns
                .iter()
                .filter(|pattern| pattern.len() <= n)
                .map(|pattern| {
                    pattern
                        .iter()
                        .map(|&x| dp[t - 1][k.saturating_sub(x)] / n as f64)
                        .chain(once((n - pattern.len()) as f64 / n as f64 * dp[t - 1][k]))
                        .sum::<f64>()
                })
                .max_by(f64::total_cmp)
                .unwrap()
        };

        dp
    });

    println!("{}", dp[t][k]);
}

fn enumerate_patterns(m: usize, l: usize, current: &mut Vec<usize>, table: &mut Vec<Vec<usize>>) {
    assert!(m > 0);

    if l > m {
        return;
    }

    {
        current.push(m);
        table.push(current.clone());
        current.pop();
    }

    if l <= m / 2 {
        for i in l..=m / 2 {
            current.push(i);
            enumerate_patterns(m - i, i, current, table);
            current.pop();
        }
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
