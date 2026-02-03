fn main() {
    input! {
        n: usize, l: usize,
        cp: [(i64, [Usize1; l]); n]
    };

    let calc_inversions = |a: &[usize], b: &[usize]| {
        let mut idxs = [0; 10];
        for i in 0..l {
            idxs[a[i]] = i;
        }

        (0..l)
            .tuple_combinations()
            .filter(|&(i, j)| idxs[b[i]] > idxs[b[j]])
            .count()
    };

    let rows = once((0..l).collect::<Vec<_>>())
        .chain(cp.iter().cloned().map(|(_, row)| row))
        .collect::<Vec<_>>();

    let k = l * (l - 1) / 2;
    let invs = (0..=n)
        .map(|i| {
            (1..k)
                .take_while(|&j| i + j < rows.len())
                .map(|j| calc_inversions(&rows[i], &rows[i + j]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dp = vec![0i64; n + 1];
    for i in 1..k {
        if i >= rows.len() {
            break;
        }
        if invs[0][i - 1] > i {
            dp[i] = i64::MIN;
        }
    }

    let (dp, _dp2) = (1..=n).fold((dp, vec![i64::MIN; n + 1]), |(mut dp, mut dp2), i| {
        let x0 = max(dp[i], dp2[i]);

        let x = x0 + cp[i - 1].0;
        dp[i] = x;
        (1..k)
            .take_while(|&j| i + j < rows.len())
            .filter(|&j| invs[i][j - 1] <= j)
            .for_each(|j| {
                dp[i + j] = max(dp[i + j], x);
            });
        if i + k < dp2.len() {
            dp2[i + k] = max(dp2[i + k], x);
        }
        if i + 1 < dp2.len() {
            dp2[i + 1] = max(dp2[i + 1], dp2[i]);
        }

        (dp, dp2)
    });

    let ans = dp.iter().max().unwrap();
    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
use rustc_hash::{FxHashMap, FxHashSet};
