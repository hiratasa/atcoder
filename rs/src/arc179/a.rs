fn main() {
    input! {
        n: usize, k: i64,
        mut a: [i64; n],
    };

    if k > 0 {
        a.sort();
        println!("Yes");
        println!("{}", a.iter().join(" "));
    } else {
        a.sort_by_key(|&x| Reverse(x));

        if a.iter().sum::<i64>() >= k {
            println!("Yes");
            println!("{}", a.iter().join(" "));
        } else {
            println!("No");
        }
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
