fn main() {
    input! {
        t: usize,
        cases: [(usize, usize, usize); t],
    };

    cases
        .into_iter()
        .map(|(na, nb, nc)| {
            // (na - k) + nb + (nc - k) >= k
            // => na + nb + nc >= 3k

            min(min(na, nc), (na + nb + nc) / 3)
        })
        .for_each(|ans| {
            println!("{}", ans);
        });
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
