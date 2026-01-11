fn main() {
    input! {
        n: usize,
    };

    if iterate(n, |&m| {
        iterate(m, |&k| k / 10)
            .take_while(|&k| k > 0)
            .map(|k| k % 10)
            .map(|k| k * k)
            .sum::<usize>()
    })
    .scan(FxHashSet::default(), |seen, x| {
        if seen.insert(x) { Some(x) } else { None }
    })
    .any(|x| x == 1)
    {
        println!("Yes");
    } else {
        println!("No");
    }
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
