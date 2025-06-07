fn main() {
    input! {
        n: usize, l: usize,
        d: [usize; n - 1],
    };

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let s = once(0).chain(d).cumsum::<usize>().collect::<Vec<_>>();
    let map = s.iter().copied().fold(vec![0; l], |mut map, x| {
        map[x % l] += 1;
        map
    });

    let ans = (0..l / 3)
        .map(|i| map[i] * map[i + l / 3] * map[i + 2 * l / 3])
        .sum::<usize>();

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
