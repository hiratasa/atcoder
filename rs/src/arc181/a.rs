fn main() {
    input! {
        t: usize,
        cases: [[usize]; t],
    };

    cases
        .into_iter()
        .map(|p| {
            let n = p.len();

            if p.iter().tuple_windows().all(|(&x, &y)| x < y) {
                // sorted
                0
            } else if p
                .iter()
                .enumerate()
                .scan(0, |ma, (i, &x)| {
                    let ok = *ma < x && i + 1 == x;
                    *ma = max(*ma, x);
                    Some(ok)
                })
                .any(|ok| ok)
            {
                1
            } else if p[0] == n && p[n - 1] == 1 {
                3
            } else {
                2
            }
        })
        .for_each(|ans| {
            println!("{ans}");
        });
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
