fn main() {
    input! {
        x: usize, y: usize, z: usize,
    };

    println!(
        "{} {}",
        y,
        repeat_n(0, x).chain(repeat_n(1, y - x)).join(" ")
    );
    println!("{} {}", z, repeat_n(0, z).join(" "));
    println!(
        "{} {}",
        y + z - x,
        repeat_n(0, z).chain(repeat_n(1, y - x)).join(" ")
    );
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
