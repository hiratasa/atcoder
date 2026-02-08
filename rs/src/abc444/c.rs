fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();

    let ans = [a[n - 1], a[0] + a[n - 1]]
        .into_iter()
        .unique()
        .filter(|&x| {
            let b = a.iter().copied().take_while(|&y| y < x).collect::<Vec<_>>();

            b.len() % 2 == 0 && izip!(b.iter(), b.iter().rev()).all(|(y, z)| y + z == x)
        })
        .sorted()
        .collect::<Vec<_>>();

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
