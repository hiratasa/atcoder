fn main() {
    input! {
        t: usize,
        a: [[usize; 9]; t],
    };

    a.into_iter()
        .map(|a| {
            let n = a.iter().sum::<usize>();
            let n5 = a[4];

            if n5 == 0 {
                let m = (0..9).filter(|&i| a[i] > 0).count();

                if m != 2 {
                    0
                } else {
                    let x = (1..10).find(|&i| a[i - 1] > 0).unwrap();
                    let y = (1..10).rev().find(|&i| a[i - 1] > 0).unwrap();

                    if x + y == 10 { 1 } else { 0 }
                }
            } else if n5 <= (n + 1) / 2 {
                0
            } else {
                let m = n - n5;

                n5 - 1 - m
            }
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
