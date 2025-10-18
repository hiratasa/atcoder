fn main() {
    input! {
        n: usize, k: usize,
    };

    let ans = if n % 2 == 0 {
        once(n / 2)
            .chain((1..=n).rev().flat_map(|x| {
                if x == n / 2 {
                    repeat_n(x, k - 1)
                } else {
                    repeat_n(x, k)
                }
            }))
            .collect::<Vec<_>>()
    } else if n == 1 {
        vec![1; k]
    } else {
        repeat_n((n + 1) / 2, k)
            .chain(once(n / 2))
            .chain((1..=n).rev().filter(|&x| x != (n + 1) / 2).flat_map(|x| {
                if x == n / 2 {
                    repeat_n(x, k - 1)
                } else {
                    repeat_n(x, k)
                }
            }))
            .collect::<Vec<_>>()
    };

    println!("{}", ans.iter().join(" "));
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
