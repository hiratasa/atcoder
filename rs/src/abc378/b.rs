fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        queries: [(Usize1, usize)],
    };

    queries
        .into_iter()
        .map(|(t, d)| {
            let (q, r) = qr[t];

            (d + q - 1 - r) / q * q + r
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
