fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
        p: [usize; h + w - 1],
    };

    let ans = lower_bound_int(0usize, 1 << 60, |x| {
        let mut dp = vec![vec![None; w]; h];
        dp[0][0] = Some(x);

        for i in 0..h {
            for j in 0..w {
                let Some(y) = dp[i][j] else {
                    continue;
                };

                let y = y + a[i][j];

                if y < p[i + j] {
                    continue;
                }

                let y = y - p[i + j];

                if i + 1 < h {
                    dp[i + 1][j] = max(dp[i + 1][j], Some(y));
                }
                if j + 1 < w {
                    dp[i][j + 1] = max(dp[i][j + 1], Some(y));
                }
            }
        }

        if matches!(dp[h - 1][w - 1], Some(y) if y + a[h - 1][w - 1] >= p[h + w - 2]) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("{ans}");
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
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
