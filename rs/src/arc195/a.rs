fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let left = b
        .iter()
        .copied()
        .scan(0, |i, x| {
            *i += a[*i..].iter().position(|&y| y == x)? + 1;

            Some(*i - 1)
        })
        .collect::<Vec<_>>();
    let right = b
        .iter()
        .copied()
        .rev()
        .scan(0, |i, x| {
            *i += a[..n - *i].iter().rev().position(|&y| y == x)? + 1;

            Some(n - *i)
        })
        .collect::<Vec<_>>();

    if left.len() != m {
        println!("No");
        return;
    }

    if izip!(left, right.into_iter().rev()).any(|(i, j)| i != j) {
        println!("Yes");
    } else {
        println!("No");
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
