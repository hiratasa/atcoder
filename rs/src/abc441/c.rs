fn main() {
    input! {
        n: usize, k: usize, x: usize,
        mut a: [usize; n],
    };
    a.sort();

    if let Some(ans) = once(0)
        .chain(a[..k].iter().copied().rev())
        .cumsum::<usize>()
        .position(|y| y >= x)
        .map(|t| t + (n - k))
    {
        println!("{ans}");
    } else {
        println!("-1");
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
