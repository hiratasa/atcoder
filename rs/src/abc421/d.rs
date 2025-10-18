fn main() {
    input! {
        rc0: (i64, i64),
        rc1: (i64, i64),
        n: usize,
        m: usize,
        l: usize,
        sa: [(char, i64); m],
        tb: [(char, i64); l],
    };

    let rc = (rc1.0 - rc0.0, rc1.1 - rc0.1);

    let ans = sa
        .into_iter()
        .scan((0, 0), |(ib, idx), (c, a)| {
            let mut v = vec![];
            let mut i = 0;
            while i < a {
                if a - i <= tb[*ib].1 - *idx {
                    v.push((c, tb[*ib].0, a - i));
                    *idx += a - i;
                    break;
                }
                if tb[*ib].1 > *idx {
                    v.push((c, tb[*ib].0, tb[*ib].1 - *idx));
                }
                i += tb[*ib].1 - *idx;
                *ib += 1;
                *idx = 0;
            }

            Some(v)
        })
        .flatten()
        .scan(rc, |(r, c), (a, b, x)| {
            let delta0 = match a {
                'U' => (1, 0),
                'D' => (-1, 0),
                'L' => (0, 1),
                'R' => (0, -1),
                _ => unreachable!(),
            };

            let delta1 = match b {
                'U' => (-1, 0),
                'D' => (1, 0),
                'L' => (0, -1),
                'R' => (0, 1),
                _ => unreachable!(),
            };

            let delta = (delta0.0 + delta1.0, delta0.1 + delta1.1);

            let check = |z: i64, d: i64| {
                if z.signum() == -d.signum() {
                    if d.abs() == 1 && z.abs() <= x {
                        Some(z.abs())
                    } else if d.abs() == 2 && z.abs() <= 2 * x && z.abs() % 2 == 0 {
                        Some(z.abs() / 2)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            let ret = if *r == 0 {
                if *c == 0 {
                    if delta == (0, 0) { Some(x) } else { Some(0) }
                } else {
                    if delta.0 == 0 && check(*c, delta.1).is_some() {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }
            } else {
                if *c == 0 {
                    if delta.1 == 0 && check(*r, delta.0).is_some() {
                        Some(1)
                    } else {
                        Some(0)
                    }
                } else if check(*r, delta.0) == check(*c, delta.1) && check(*r, delta.0).is_some() {
                    Some(1)
                } else {
                    Some(0)
                }
            };

            *r += delta.0 * x;
            *c += delta.1 * x;

            ret
        })
        .sum::<i64>();

    println!("{ans}");
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
