fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let s = a.iter().map(|&x| x - 1).sum::<usize>();

    let first_win = match (n % 2, s % 2) {
        (0, 0) => false,
        (0, 1) => {
            let even = a.iter().map(|&x| x - 1).filter(|&x| x % 2 == 0).count();
            let odd = n - even;

            even >= 3 || odd >= 3
        }
        (1, 0) => true,
        (1, 1) => {
            let even = a.iter().map(|&x| x - 1).filter(|&x| x % 2 == 0).count();
            let odd = n - even;

            even <= 2 && odd == 1
        }
        _ => unreachable!(),
    };

    if first_win {
        println!("Fennec");
    } else {
        println!("Snuke");
    }
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
