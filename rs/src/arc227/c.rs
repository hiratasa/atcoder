fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let p = (1..=n)
        .filter(|p| n % p == 0)
        .find(|&p| (0..n).all(|i| s[i] == s[(i + p) % n]))
        .unwrap();

    println!("{}", n / p);
    println!("{}", n * n);
    println!("{}", s.iter().copied().cycle().take(n * n).join(""));
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
