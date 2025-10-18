fn main() {
    input! {
        mut s: [i64; 2],
        mut t: [i64; 2],
    }

    if (s[0] + s[1]) % 2 > 0 {
        s[0] -= 1;
    }

    if (t[0] + t[1]) % 2 > 0 {
        t[0] -= 1;
    }

    let dx = (t[0] - s[0]).abs();
    let dy = (t[1] - s[1]).abs();

    let ans = if dx <= dy { dy } else { dy + (dx - dy) / 2 };

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
