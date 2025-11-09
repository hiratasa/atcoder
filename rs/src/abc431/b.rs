fn main() {
    input! {
        x: usize,
        n: usize,
        w: [usize; n],
        q: usize,
        p: [Usize1; q],
    };

    p.into_iter()
        .scan((x, vec![false; n]), |(x, attached), i| {
            if attached[i] {
                attached[i] = false;
                *x -= w[i];
            } else {
                attached[i] = true;
                *x += w[i];
            }
            Some(*x)
        })
        .for_each(|ans| {
            println!("{ans}");
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
