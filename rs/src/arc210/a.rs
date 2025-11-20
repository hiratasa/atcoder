fn main() {
    input! {
        n: usize, q: usize,
        ix: [(Usize1, i64); q],
    };

    let mut b = vec![1; n - 1];
    let mut c = vec![0; n];

    for (i, x) in ix {
        c[i] += x;
        if i < n - 1 {
            b[i] = max(b[i], c[i] + 1 - c[i + 1]);
        }
    }

    // eprintln!("{b:?}");
    // eprintln!("{c:?}");
    let ans = once(1)
        .chain(b)
        .enumerate()
        .map(|(i, x)| x * (n - i) as i64)
        .sum::<i64>();

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
