fn main() {
    input! {
        h: usize, w: usize,
    };

    if h == 1 {
        if w == 1 {
            println!("0");
        } else {
            println!(
                "{}",
                once(1).chain(repeat_n(2, w - 2)).chain(once(1)).join(" ")
            );
        }
    } else {
        if w == 1 {
            println!(
                "{}",
                once(1).chain(repeat_n(2, h - 2)).chain(once(1)).join("\n")
            );
        } else {
            println!(
                "{}",
                once(2).chain(repeat_n(3, w - 2)).chain(once(2)).join(" ")
            );
            for i in 1..h - 1 {
                println!(
                    "{}",
                    once(3).chain(repeat_n(4, w - 2)).chain(once(3)).join(" ")
                );
            }
            println!(
                "{}",
                once(2).chain(repeat_n(3, w - 2)).chain(once(2)).join(" ")
            );
        }
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
