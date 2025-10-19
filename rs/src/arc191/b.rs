fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            mut n: usize, k: usize,
        };

        // let m = (n + 1).next_power_of_two();
        // let mask = (n + 1).next_power_of_two() - 1;
        // let l = (!n & mask).count_ones();
        // let t = 2usize << l;

        let mut j = 0;
        for i in 0.. {
            if 1 << i > n {
                break;
            }
            if n & (1 << i) == 0 {
                if (k - 1) & (1 << j) > 0 {
                    n |= 1 << i
                }
                j += 1;
            }
        }
        if 1 << j > k - 1 {
            println!("{n}");
        } else {
            println!("-1");
        }
    }
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
