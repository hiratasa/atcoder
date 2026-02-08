fn main() {
    input! {
        n: usize, d: usize,
        a: [usize; n],
    };

    let ans = (0..n)
        .scan((BTreeSet::<(usize, usize)>::new(), 0), |(set, j), i| {
            while *j < n {
                let x = a[*j];
                let e = (x, *j);
                if let Some(&(y, _)) = set.range(..e).next_back()
                    && x - y < d
                {
                    break;
                }
                if let Some(&(y, _)) = set.range(e..).next()
                    && y - x < d
                {
                    break;
                }

                set.insert(e);
                *j += 1;
            }

            set.remove(&(a[i], i));

            Some(*j - i)
        })
        .sum::<usize>();

    println!("{ans}");
}

use std::collections::BTreeSet;
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
