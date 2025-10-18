fn main() {
    input! {
        s: [char; 3],
    };

    let cmp = |c: char| {
        if c == '<' {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    let ans = (0..3)
        .sorted_by(|&i, &j| match (i, j) {
            (0, 1) => cmp(s[0]),
            (1, 0) => cmp(s[0]).reverse(),
            (0, 2) => cmp(s[1]),
            (2, 0) => cmp(s[1]).reverse(),
            (1, 2) => cmp(s[2]),
            (2, 1) => cmp(s[2]).reverse(),
            _ => unreachable!(),
        })
        .nth(1)
        .unwrap();

    println!("{}", (b'A' + ans as u8) as char);
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
