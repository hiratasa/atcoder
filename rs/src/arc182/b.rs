fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t],
    };

    for (n, k) in nk {
        let m = 1usize << (k - 1);

        let ans = (0..m)
            .rev()
            .cycle()
            .take(n)
            .map(|x| x.reverse_bits() >> (64 - (k - 1)))
            .map(|x| x + (1 << (k - 1)))
            .collect::<Vec<_>>();

        // eprintln!(
        //     "# {}",
        //     ans.iter()
        //         .flat_map(|&x| iterate(x, |&y| y / 2).take_while(|&y| y > 0))
        //         .chain(once(0))
        //         .unique()
        //         .count()
        // );

        println!("{}", ans.iter().join(" "));
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
