fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let t = ab.iter().copied().fold(vec![n - 1; n], |mut t, (a, b)| {
        t[a] -= 1;
        t[b] -= 1;
        t
    });

    println!(
        "{}",
        t.iter()
            .copied()
            .map(|d| d * d.saturating_sub(1) * d.saturating_sub(2) / 6)
            .join(" ")
    );
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
