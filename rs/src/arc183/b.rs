fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        if solve() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve() -> bool {
    input! {
        n: usize, k: usize,
        a: [usize; n],
        b: [usize; n],
    };

    if a == b {
        return true;
    }

    let counts_a = a.iter().fold(FxHashMap::default(), |mut counts, x| {
        *counts.entry(x).or_insert(0) += 1;
        counts
    });

    if b.iter().any(|&x| !counts_a.contains_key(&x)) {
        return false;
    }

    if k == 1 {
        b.into_iter()
            .scan(0, |i, x| {
                while *i < n && a[*i] != x {
                    *i += 1;
                }

                if *i == n {
                    Some(false)
                } else {
                    Some(true)
                }
            })
            .all(|ok| ok)
    } else {
        let set_b = b.iter().collect::<FxHashSet<_>>();

        if !counts_a.iter().any(|(&x, &l)| l > 1 || !set_b.contains(&x)) {
            return false;
        }

        b.iter()
            .enumerate()
            .scan(FxHashMap::default(), |map, (i, x)| {
                if let Some(&j) = map.get(&x) {
                    if i - j <= k {
                        Some(true)
                    } else {
                        map.insert(x, i);
                        Some(false)
                    }
                } else {
                    map.insert(x, i);
                    Some(false)
                }
            })
            .any(|ok| ok)
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
