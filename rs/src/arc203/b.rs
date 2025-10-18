fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
            b: [usize; n],
        };

        if solve(&a, &b) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve(a: &[usize], b: &[usize]) -> bool {
    let n = a.len();

    let na = a.iter().sum::<usize>();
    let nb = b.iter().sum::<usize>();
    if na != nb {
        return false;
    }

    if na == 1 {
        if (a[0] == 1) != (b[0] == 1) {
            return false;
        }

        if (a[n - 1] == 1) != (b[n - 1] == 1) {
            return false;
        }

        true
    } else {
        true
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
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
