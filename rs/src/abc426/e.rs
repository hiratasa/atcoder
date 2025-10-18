fn main() {
    input! {
        t: usize,
        cases: [[[[f64; 2]; 2]; 2]; t],
    };

    cases
        .into_iter()
        .map(|sg| {
            let s = [[sg[0][0][0], sg[0][0][1]], [sg[1][0][0], sg[1][0][1]]];
            let g = [[sg[0][1][0], sg[0][1][1]], [sg[1][1][0], sg[1][1][1]]];
            let e = [
                normalize(sub(g[0], s[0])).unwrap(),
                normalize(sub(g[1], s[1])).unwrap(),
            ];

            let l0 = len(sub(s[0], g[0]));
            let l1 = len(sub(s[1], g[1]));

            [
                (sub(s[0], s[1]), sub(e[0], e[1]), 0.0, f64::min(l0, l1)),
                (sub(s[0], g[1]), e[0], l1, l0),
                (sub(g[0], s[1]), neg(e[1]), l0, l1),
            ]
            .into_iter()
            .map(|(ss, ee, t0, t1)| {
                // eprintln!("{ss:?} {ee:?} {t0:?} {t1:?}");
                if t0 > t1 {
                    f64::MAX
                } else {
                    let c0 = dot(ss, ss);
                    let c1 = dot(ss, ee);
                    let c2 = dot(ee, ee);

                    let f = |t: f64| len(add(ss, mul(t, ee)));

                    let z0 = if c2 != 0.0 && t0 <= -c1 / c2 && -c1 / c2 <= t1 {
                        Some(f(-c1 / c2))
                    } else {
                        None
                    };

                    // eprintln!("{c0} {c1} {c2} {z0:?}");

                    [Some(f(t0)), Some(f(t1)), z0]
                        .into_iter()
                        // .inspect(|z| eprintln!("#{z:?}"))
                        .flatten()
                        .min_by(f64::total_cmp)
                        .unwrap()
                }
            })
            .min_by(f64::total_cmp)
            .unwrap()
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}

fn add(s: [f64; 2], g: [f64; 2]) -> [f64; 2] {
    [s[0] + g[0], s[1] + g[1]]
}

fn sub(s: [f64; 2], g: [f64; 2]) -> [f64; 2] {
    add(s, neg(g))
}

fn neg(s: [f64; 2]) -> [f64; 2] {
    [-s[0], -s[1]]
}

fn mul(c: f64, s: [f64; 2]) -> [f64; 2] {
    [c * s[0], c * s[1]]
}

fn dot(s: [f64; 2], g: [f64; 2]) -> f64 {
    s[0] * g[0] + s[1] * g[1]
}

fn len(s: [f64; 2]) -> f64 {
    dot(s, s).sqrt()
}

fn normalize(s: [f64; 2]) -> Option<[f64; 2]> {
    let l = len(s);

    if l == 0.0 {
        None
    } else {
        Some([s[0] / l, s[1] / l])
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
