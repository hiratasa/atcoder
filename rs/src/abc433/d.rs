fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    let p = (0..12)
        .scan(1, |k, _| Some(replace(k, *k * 10 % m)))
        .collect::<Vec<_>>();
    let t = p
        .iter()
        .map(|&k| a.iter().map(|&x| x * k % m).counts())
        .collect::<Vec<_>>();

    let ans = a
        .iter()
        .map(|&x| {
            let d = iterate(x, |&y| y / 10)
                .take_while(|&y| y > 0)
                .map(|y| y % 10)
                .count();

            t[d].get(&((m - x % m) % m)).copied().unwrap_or(0)
        })
        .sum::<usize>();

    println!("{ans}");
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
