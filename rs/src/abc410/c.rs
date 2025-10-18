fn main() {
    input! {
        n: usize, q: usize,
    };

    (0..q)
        .scan(((1..=n).collect::<Vec<_>>(), 0), |(a, st), _| {
            input! {
                ty: usize,
            };

            if ty == 1 {
                input! { p: Usize1, x: usize };
                a[(p + *st) % n] = x;
                Some(None)
            } else if ty == 2 {
                input! { p: Usize1 };
                Some(Some(a[(p + *st) % n]))
            } else {
                input! { k: usize };
                *st = (*st + k) % n;
                Some(None)
            }
        })
        .flatten()
        .for_each(|ans| println!("{ans}"))
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
