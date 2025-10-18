fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                n: usize,
                p: [usize; 1 << n],
            };

            let mut mins = vec![0; 1 << (n + 1)];
            calc_mins(&p, 0, 0, 1 << n, &mut mins);
            let mut ans = vec![];
            calc(&p, 0, &mins, &mut ans);
            ans
        })
        .for_each(|ans| {
            println!("{}", ans.iter().join(" "));
        });
}

fn calc_mins(p: &[usize], idx: usize, l: usize, r: usize, mins: &mut [usize]) -> usize {
    if l + 1 == r {
        mins[idx] = p[l];
    } else {
        let m = (l + r) / 2;
        let ll = calc_mins(p, idx * 2 + 1, l, m, mins);
        let rr = calc_mins(p, idx * 2 + 2, m, r, mins);
        mins[idx] = min(ll, rr);
    }

    mins[idx]
}

fn calc(p: &[usize], idx: usize, mins: &[usize], out: &mut Vec<usize>) {
    if 2 * idx + 2 < mins.len() {
        let ll = mins[2 * idx + 1];
        let rr = mins[2 * idx + 2];

        if ll < rr {
            calc(p, 2 * idx + 1, mins, out);
            calc(p, 2 * idx + 2, mins, out);
        } else {
            calc(p, 2 * idx + 2, mins, out);
            calc(p, 2 * idx + 1, mins, out);
        }
    } else {
        out.push(p[idx - (p.len() - 1)]);
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
