fn main() {
    input! {
        t: usize, m: usize,
        cases: [[usize]; t],
    };

    const K: usize = 5000;
    let combi = iterate(vec![1usize], |prev| {
        izip!(
            prev.iter().copied().chain(once(0)),
            once(0).chain(prev.iter().copied())
        )
        .map(|(x, y)| (x + y) % m)
        .collect()
    })
    .take(K + 1)
    .collect::<Vec<_>>();

    cases
        .into_iter()
        .map(|case| {
            let s = case.iter().sum::<usize>();

            let mut ans = 1;
            let mut r = s;
            for &c in &case {
                r -= c;
                ans *= combi[r + c][c];
                ans %= m;
            }

            ans
        })
        .for_each(|ans| println!("{ans}"));
}

#[allow(unused_imports)]
use std::{
    cmp::{Reverse, max, min},
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

pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
}
