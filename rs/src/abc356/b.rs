fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    };

    let b = x.into_iter().fold(vec![0; m], |mut b, c| {
        izip!(b.iter_mut(), c).for_each(|(x, y)| *x += y);
        b
    });

    if izip!(a, b).all(|(x, y)| x <= y) {
        println!("Yes");
    } else {
        println!("No");
    }
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
