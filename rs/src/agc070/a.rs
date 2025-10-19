fn main() {
    let x = {
        let mut c = 1;
        let mut x = vec![];
        let z = 102;
        loop {
            // println!("#{c}");
            let r = c % 10;
            x.push(r);
            c /= 10;
            c += r * z;
            if c == 1 {
                break;
            }
        }
        x
    };

    let s = once(&x)
        .cycle()
        .flatten()
        .take(5000)
        .copied()
        .collect::<Vec<_>>();

    println!("{}", x.iter().rev().join(""));
    println!("{}", s.iter().rev().join(""));
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
