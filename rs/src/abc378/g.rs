fn main() {
    input! {
        a: usize, b: usize, m: usize,
    };

    let (a, b) = (min(a, b), max(a, b));

    let dp = (0..=b)
        .combinations_with_replacement(a)
        .skip(1)
        .filter(|t| t[a - 1] < b || t[a - 2] == b)
        .fold(
            once((vec![0; a], 1)).collect::<FxHashMap<_, _>>(),
            |mut dp, mut t| {
                if t[a - 1] == b {
                    t[a - 1] -= 1;
                }

                let z = (0..a)
                    .map(|i| {
                        if t[i] == 0 {
                            0
                        } else {
                            t[i] -= 1;
                            let y = dp.get(&t).copied().unwrap_or(0);
                            t[i] += 1;
                            y
                        }
                    })
                    .sum::<usize>();

                dp.insert(t, z % m);

                dp
            },
        );

    let dp2 = (0..=b).combinations_with_replacement(a).skip(1).fold(
        once((vec![0; a], 1)).collect::<FxHashMap<_, _>>(),
        |mut dp, mut t| {
            let z = (0..a)
                .map(|i| {
                    if t[i] == 0 {
                        0
                    } else {
                        t[i] -= 1;
                        let y = dp.get(&t).copied().unwrap_or(0);
                        t[i] += 1;
                        y
                    }
                })
                .sum::<usize>();

            dp.insert(t, z % m);

            dp
        },
    );

    let mut key = vec![b; a];
    key[a - 1] -= 1;
    let mut key2 = vec![b; a];
    key2[0] -= 1;
    let ans = dp[&key] * dp2[&key2] % m;

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
