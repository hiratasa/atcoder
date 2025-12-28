fn main() {
    input! {
        n: usize, m: usize,
        rc: [(usize, usize); m],
    };

    let ans = rc
        .into_iter()
        .scan(FxHashSet::default(), |set, (r, c)| {
            if set.contains(&(r, c))
                || set.contains(&(r, c + 1))
                || set.contains(&(r + 1, c))
                || set.contains(&(r + 1, c + 1))
            {
                Some(false)
            } else {
                set.insert((r, c));
                set.insert((r, c + 1));
                set.insert((r + 1, c));
                set.insert((r + 1, c + 1));
                Some(true)
            }
        })
        .filter(|&inserted| inserted)
        .count();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
