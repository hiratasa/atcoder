fn main() {
    input! {
        q: usize,
        a: [usize; q],
    };

    a.iter()
        .copied()
        .scan((0usize, false), |(x, playing), c| {
            if c == 1 {
                *x += 1;
            } else if c == 2 {
                *x = x.saturating_sub(1);
            } else {
                *playing = !*playing;
            }

            Some(*x >= 3 && *playing)
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        });
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
