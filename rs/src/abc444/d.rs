fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();

    let ans = (0usize..)
        .scan((0, 0), |(carry, i), j| {
            while *i < n && a[*i] <= j {
                *i += 1;
            }

            *carry += n - *i;

            if *carry == 0 && *i == n {
                None
            } else {
                let x = *carry % 10;
                *carry /= 10;
                Some(x)
            }
        })
        .collect::<Vec<_>>();

    println!("{}", ans.iter().rev().join(""));
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
