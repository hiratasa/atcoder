fn main() {
    input! {
        n: usize, m: usize,
        c: [usize; n],
        a: [[Usize1]; m],
    };

    let ans = (0..n)
        .map(|_| (0..=2))
        .multi_cartesian_product()
        .filter(|v| {
            (0..m)
                .map(|i| a[i].iter().map(|&j| v[j]).sum::<usize>())
                .all(|x| x >= 2)
        })
        .map(|v| izip!(v, &c).map(|(x, &y)| x * y).sum::<usize>())
        .min()
        .unwrap();

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
