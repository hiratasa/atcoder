fn main() {
    input! {
        n: usize,
    };

    let k = ((n as f64).sqrt().ceil()) as usize;

    let freq = (1..=k)
        .tuple_combinations()
        .map(|(x, y)| x * x + y * y)
        .fold(vec![0; n + 1], |mut freq, x| {
            if x <= n {
                freq[x] += 1;
            }
            freq
        });

    let ans = freq.iter().positions(|&x| x == 1).collect::<Vec<_>>();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
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
