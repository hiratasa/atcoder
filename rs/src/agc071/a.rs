fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let dp = (1..=n)
        .flat_map(|len| (0..=n - len).map(move |i| (i, i + len)))
        .fold(vec![vec![0; n + 1]; n], |mut dp, (i, j)| {
            let len = j - i;

            let x = a[i..j].iter().fold(0, |x, &y| x ^ y);

            dp[i][j] = if len == 1 {
                0
            } else {
                (i + 1..j)
                    .map(|k| match (len % 2, (k - i) % 2) {
                        (0, 1) => 2 * x + dp[i][k] + dp[k][j],
                        _ => dp[i][k] + dp[k][j],
                    })
                    .min()
                    .unwrap()
            };

            dp
        });

    let ans = if n % 2 == 0 {
        dp[0][n]
    } else {
        let x = a.iter().fold(0, |x, &y| x ^ y);

        x + dp[0][n]
    };

    println!("{ans}");
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
