fn main() {
    input! {
        cases: [[i64]]
    };

    cases
        .into_iter()
        .map(|a| {
            if a.iter().map(|x| x.abs()).all_equal() {
                a.iter().map(|x| x.signum()).all_equal()
                    || a.iter()
                        .filter(|&&x| x > 0)
                        .count()
                        .abs_diff(a.iter().filter(|&&x| x < 0).count())
                        <= 1
            } else {
                let mut a = a;
                a.sort_by_key(|&x| x.abs());

                a.iter().tuple_windows().all(|(x, y, z)| y * y == x * z)
            }
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        });
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
