fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            h: usize, w: usize,
        };

        if h % 2 != 1 || w % 2 != 1 {
            println!("{}", (h / 2) * (w / 2));
            for i in (1..h).step_by(2) {
                for j in (1..w).step_by(2) {
                    println!("{i} {j} {1}");
                }
            }
        } else {
            let hh = if h % 4 == 3 { h - 2 } else { h };

            let ww = if w % 4 == 3 { w - 2 } else { w };

            let mut ans = vec![];
            for i in 1..=ww {
                let mut j = 1;
                let mut last = 0;
                while j <= hh {
                    if i == j || i % ww + 1 == j {
                        j + 1;
                    } else if last > 0 {
                    }
                }
            }
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
