fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [usize; n],
    };

    if n == m {
        println!("{}", repeat_n(0, n).join(" "));
        return;
    }

    let sum = a.iter().sum::<usize>();

    let r = k - sum;

    let top_m = (0..n)
        .sorted_by_key(|&i| Reverse((a[i], i)))
        .take(m)
        .collect::<Vec<_>>();
    let sums = once(0)
        .chain(top_m.iter().map(|&i| a[i]))
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    let top_m1 = (0..n)
        .sorted_by_key(|&i| Reverse((a[i], i)))
        .take(m + 1)
        .collect::<Vec<_>>();
    let sums1 = once(0)
        .chain(top_m1.iter().map(|&i| a[i]))
        .cumsum::<usize>()
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..n)
            .map(|i| {
                let (tops, sums, d) = if (a[i], i) >= top_m.last().map(|&j| (a[j], j)).unwrap() {
                    (&top_m1, &sums1, true)
                } else {
                    (&top_m, &sums, false)
                };

                let b = lower_bound_int(0, r + 1, |x| {
                    let p = a[i] + x;

                    let idx = tops
                        .binary_search_by(|&j| a[j].cmp(&p).then(Ordering::Less).reverse())
                        .unwrap_err();

                    let l = tops.len() - idx;

                    let q = l * (p + 1) - (sums[tops.len()] - sums[idx]);

                    let q = if d { q - (p + 1 - a[i]) } else { q };

                    // eprintln!(
                    //     "#{i} {x} {tops:?} {sums:?} {idx} {q} {l} {p} {} {d} {:?}",
                    //     a[i],
                    //     p + q > r
                    // );

                    if x + q > r {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                if b == r + 1 { -1 } else { b as i64 }
            })
            .join(" ")
    );
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

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}
