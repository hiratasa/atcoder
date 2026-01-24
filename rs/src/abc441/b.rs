fn main() {
    input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        w: [Chars; q],
    };

    for w in w {
        let ok1 = w.iter().all(|&c| s.contains(&c));
        let ok2 = w.iter().all(|&c| t.contains(&c));

        let ans = match (ok1, ok2) {
            (true, true) => "Unknown",
            (true, false) => "Takahashi",
            (false, true) => "Aoki",
            (false, false) => unreachable!(),
        };

        println!("{ans}")
    }
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
