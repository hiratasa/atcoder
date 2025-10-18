fn main() {
    input! {
        n: usize,
        c: [usize; n - 1],
        a: [usize; n - 1],
    };

    let n = a.iter().rposition(|&x| x > 0).unwrap() + 1;

    let dp = (0..n).fold(vec![0usize], |mut dp, i| {
        let x = if let Some(j) = (i.saturating_sub(c[i])..i).rfind(|&j| a[j] > 0) {
            dp[j + 1]
        } else {
            dp[i + 1 - c[i]..].iter().copied().min().unwrap()
        };
        dp.push(x + 1);

        dp
    });

    println!("{}", dp[n]);
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
