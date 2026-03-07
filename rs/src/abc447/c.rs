fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut i = 0;
    let mut j = 0;
    let mut ans = 0;
    while i < s.len() || j < t.len() {
        if i < s.len() && j < t.len() && s[i] == t[j] {
            i += 1;
            j += 1;
        } else if i < s.len() && s[i] == 'A' {
            i += 1;
            ans += 1;
        } else if j < t.len() && t[j] == 'A' {
            j += 1;
            ans += 1;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{ans}");
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
