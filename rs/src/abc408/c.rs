fn main() {
    input! {
        n: usize, m: usize,
        lr: [(Usize1, usize); m],
    };

    let mut t = lr.into_iter().fold(vec![0; n + 1], |mut t, (l, r)| {
        t[l] += 1;
        t[r] -= 1;
        t
    });
    for i in 1..=n {
        t[i] += t[i - 1];
    }

    let ans = *t[..n].iter().min().unwrap();

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
