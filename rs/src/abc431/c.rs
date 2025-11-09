fn main() {
    input! {
        n: usize, m: usize, k: usize,
        mut h: [usize; n],
        mut b: [usize; m],
    };

    h.sort();
    b.sort();

    let ans = h
        .into_iter()
        .scan(0, |i, x| {
            while *i < m && b[*i] < x {
                *i += 1;
            }

            if *i < m {
                *i += 1;
                Some(())
            } else {
                None
            }
        })
        .nth(k - 1)
        .is_some();

    if ans {
        println!("Yes");
    } else {
        println!("No");
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
