fn main() {
    input! {
        mut s: Chars,
    };

    let n = s.len();
    let l = s.iter().filter(|&&c| c.is_ascii_lowercase()).count();
    let u = n - l;

    if l < u {
        s.iter_mut().for_each(|c| c.make_ascii_uppercase());
    } else {
        s.iter_mut().for_each(|c| c.make_ascii_lowercase());
    }

    println!("{}", s.iter().join(""));
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
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
