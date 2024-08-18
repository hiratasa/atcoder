fn main() {
    input! {
        x: f64,
    };

    let y = (x * 1000.0).round() as usize;

    let mut z = y % 1000;
    let mut d = 3;
    while z % 10 == 0 && z > 0 {
        z /= 10;
        d -= 1;
    }

    if z == 0 {
        println!("{}", y / 1000);
    } else {
        println!("{}.{:0width$}", y / 1000, z, width = d);
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
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
