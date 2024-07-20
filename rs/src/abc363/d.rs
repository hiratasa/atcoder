fn main() {
    input! {
        n: Usize1,
    };

    if n == 0 {
        println!("0");
        return;
    }

    let (i, (s0, _)) = once(1)
        .chain((1..).map(|i| {
            let d = (i + 1) / 2;

            9usize.saturating_mul(10usize.saturating_pow(d - 1))
        }))
        .scan(0usize, |s, x| {
            let s0 = *s;
            *s = s.saturating_add(x);

            Some((s0, *s))
        })
        .enumerate()
        .find(|&(i, (s0, s))| n < s)
        .unwrap();

    let m = n - s0;
    let d = (i + 1) / 2;
    let x = 10usize.pow(d as u32 - 1) + m;

    let ans = if i % 2 == 0 {
        chain(x.to_string().chars(), x.to_string().chars().rev()).collect::<String>()
    } else {
        chain(x.to_string().chars(), x.to_string().chars().rev().skip(1)).collect::<String>()
    };

    println!("{ans}");
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
