fn main() {
    input! {
        s: String,
        t: String,
    };

    println!(
        "{}",
        s.chars()
            .zip_longest(t.chars())
            .position(|x| {
                match x {
                    EitherOrBoth::Both(a, b) => a != b,
                    EitherOrBoth::Left(_) => true,
                    EitherOrBoth::Right(_) => true,
                }
            })
            .map_or(0, |i| i + 1)
    );
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
