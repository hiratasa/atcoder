fn main() {
    input! {
        n: usize, q: usize,
        mut a: [i64; n],
        bk: [(i64, usize); q],
    };

    a.sort();

    let set = once(-1000000000)
        .chain(a.iter().copied())
        .chain(once(1000000000))
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<BTreeSet<_>>();

    bk.into_iter()
        .map(|(b, k)| {
            lower_bound_int(0, 200000000, |d| {
                let i0 = set.range((b - d, 0)..).next().unwrap().1;
                let i1 = set.range((b + d + 1, 0)..).next().unwrap().1;

                if i1 - i0 >= k {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}

use std::collections::BTreeSet;
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
