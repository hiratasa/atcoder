fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = (0..)
        .try_fold(a, |mut b, i| {
            b.sort();
            b.reverse();

            if b[1] == 0 {
                Err(i)
            } else {
                b[0] -= 1;
                b[1] -= 1;
                Ok(b)
            }
        })
        .unwrap_err();

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
