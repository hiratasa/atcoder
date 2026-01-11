fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let ans = [3, 5, 7]
        .into_iter()
        .permutations(3)
        .filter(|v| v[0] == 5 || v[2] == 5)
        .map(|v| {
            a.iter()
                .copied()
                .scan(FxHashMap::default(), |dp, x| {
                    if x % v[0] == 0 {
                        dp.entry(x / v[0]).or_insert([0usize; 2])[0] += 1;
                    }
                    if x % v[1] == 0 {
                        if let Some(u) = dp.get_mut(&(x / v[1])) {
                            u[1] += u[0];
                        }
                    }
                    if x % v[2] == 0 {
                        if let Some(u) = dp.get(&(x / v[2])) {
                            return Some(u[1]);
                        }
                    }
                    Some(0)
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{ans}");
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
