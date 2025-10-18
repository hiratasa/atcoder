fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    let ans = a
        .into_iter()
        .sorted()
        .enumerate()
        .scan(0, |s, (i, x)| {
            *s += x;

            if *s + (n - i - 1) * x > m {
                None
            } else if i < n - 1 {
                Some((m - *s) / (n - i - 1))
            } else {
                Some(usize::MAX)
            }
        })
        .chain(once(m / n))
        .max()
        .unwrap();

    if ans == usize::MAX {
        println!("infinite");
    } else {
        println!("{ans}");
    }
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
