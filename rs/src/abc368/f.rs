fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    const M: usize = 100000;
    let grundy = (2..=M).fold(vec![0; M + 1], |mut grundy, i| {
        grundy[i] = (2..)
            .take_while(|&j| j * j <= i)
            .filter(|&j| i % j == 0)
            .flat_map(|j| [j, i / j])
            .chain(once(1))
            .map(|j| grundy[j])
            .sorted()
            .dedup()
            .chain(once(usize::MAX))
            .enumerate()
            .position(|(j, k)| j != k)
            .unwrap();
        grundy
    });

    let g = a.into_iter().map(|x| grundy[x]).fold(0, |x, y| x ^ y);

    if g == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
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
