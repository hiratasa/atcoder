fn main() {
    input! {
        n: usize, m: usize,
        mut a: [usize; n],
        b: [usize; m],
    };

    a.sort();
    if let Some((_, ans)) = b
        .into_iter()
        .sorted()
        .try_fold((0, 0), |(mut next, sum), x| {
            while next < n && a[next] < x {
                next += 1;
            }

            if next >= n {
                return None;
            } else {
                Some((next + 1, sum + a[next]))
            }
        })
    {
        println!("{ans}");
    } else {
        println!("-1");
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
