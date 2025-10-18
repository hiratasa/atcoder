fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, k: usize, x: usize,
            a: [f64; n],
        };

        let mut untouched = n;
        let mut k = k;
        let mut q = BinaryHeap::new();
        for &v in &a {
            q.push((ordered_float::OrderedFloat(v), 1, true));
        }
        while untouched > 0 && k > 0 {
            let (ordered_float::OrderedFloat(v), m, is_first) = q.pop().unwrap();
            if is_first {
                untouched -= 1;
            }

            if k >= m {
                q.push((ordered_float::OrderedFloat(v / 2.0), 2 * m, false));
                k -= m;
            } else {
                q.push((ordered_float::OrderedFloat(v / 2.0), 2 * k, false));
                q.push((ordered_float::OrderedFloat(v), m - k, false));
                k = 0;
            }
        }

        let mut f = 1.0;
        let l = q.iter().map(|&(_, m, _)| m).sum::<usize>();
        let mut z = 1;
        while k >= l * z {
            f /= 2.0;
            k -= l * z;
            z *= 2;
        }

        let mut q = q
            .into_iter()
            .map(|(v, m, _)| (v * f, m * z))
            .collect::<BinaryHeap<_>>();
        while k > 0 {
            let (ordered_float::OrderedFloat(v), m) = q.pop().unwrap();

            if k >= m {
                q.push((ordered_float::OrderedFloat(v / 2.0), 2 * m));
                k -= m;
            } else {
                q.push((ordered_float::OrderedFloat(v / 2.0), 2 * k));
                q.push((ordered_float::OrderedFloat(v), m - k));
                k = 0;
            }
        }

        let ans = q
            .into_iter()
            .sorted()
            .rev()
            .scan(x, |x, (ordered_float::OrderedFloat(v), m)| {
                *x = x.saturating_sub(m);
                if *x == 0 { Some(Some(v)) } else { Some(None) }
            })
            .flatten()
            .next()
            .unwrap();

        println!("{ans}");
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
