fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n],
    };

    let mut ops = vec![];
    if solve(&mut a, k, 0, 0, &mut ops) {
        println!("Yes");
        println!("{}", ops.len());
        println!("{}", ops.iter().map(|&i| i + 1).join(" "));
    } else {
        println!("No");
    }
}

fn solve(a: &mut [usize], k: usize, x: usize, i0: usize, ops: &mut Vec<usize>) -> bool {
    let n = a.len();

    while x > a[0] {
        ops.push(i0);
        a.swap(0, 1);
        a[0] += k;
    }

    if n == 2 {
        while a[0] / k > a[1] / k {
            ops.push(i0);
            a.swap(0, 1);
            a[0] += k;

            if x > a[0] {
                ops.push(i0);
                a.swap(0, 1);
                a[0] += k;
            }
        }

        if a[0] <= a[1] {
            true
        } else if a[0] / k == a[1] / k {
            false
        } else {
            unreachable!();
        }
    } else if n == 3 {
        let a0 = a[0];
        let ok = solve(&mut a[1..], k, a0, i0 + 1, ops);

        if ok {
            true
        } else {
            while !(a[0] / k < a[1] / k && a[0] / k < a[2] / k) {
                ops.push(i0 + 1);
                a.swap(1, 2);
                a[1] += k;
            }

            {
                ops.push(i0);
                a.swap(0, 1);
                a[0] += k;
            }

            let a0 = a[0];
            let ok = solve(&mut a[1..], k, a0, i0 + 1, ops);
            assert!(ok);
            true
        }
    } else {
        let a0 = a[0];
        solve(&mut a[1..], k, a0, i0 + 1, ops)
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
