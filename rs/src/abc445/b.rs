fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let m = s.iter().map(|x| x.len()).max().unwrap();

    for i in 0..n {
        let k = s[i].len();
        println!(
            "{}{}{}",
            repeat_n('.', (m - k) / 2).join(""),
            s[i].iter().join(""),
            repeat_n('.', (m - k) / 2).join(""),
        );
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
