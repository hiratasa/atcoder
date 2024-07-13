fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    };

    let ls = once(0)
        .chain(lr.iter().map(|&(l, _)| l))
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let rs = once(0)
        .chain(lr.iter().map(|&(_, r)| r))
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    if ls[n] <= 0 && 0 <= rs[n] {
        println!("Yes");

        let i = (0..n)
            .find(|&i| {
                let s = ls[i] + rs[n] - rs[i + 1];

                s + lr[i].0 <= 0 && 0 <= s + lr[i].1
            })
            .unwrap();

        let s = ls[i] + rs[n] - rs[i + 1];

        println!(
            "{}",
            lr[..i]
                .iter()
                .copied()
                .map(|(l, _)| l)
                .chain(once(-s))
                .chain(lr[i + 1..].iter().copied().map(|(_, r)| r))
                .join(" ")
        );
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
