fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by_key(|&(a, b)| (a, Reverse(b)));

    let t = ab
        .iter()
        .copied()
        .fold(vec![], |mut t: Vec<usize>, (_, b)| {
            let idx = t
                .binary_search_by(|&x| x.cmp(&b).then(Ordering::Greater))
                .unwrap_err();

            if idx == t.len() {
                t.push(b);
            } else {
                t[idx] = b;
            }

            t
        });

    let ans = t.len();

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
