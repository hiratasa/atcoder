fn main() {
    input_interactive! {
        n: usize,
    };

    let mut v = vec![1];
    for i in 2..=n {
        let idx = lower_bound_int(0, v.len(), |idx| {
            println!("? {} {}", i, v[idx]);
            input_interactive! {
                res: usize,
            };

            if res == 1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        v.insert(idx, i);
    }

    while v.len() > 1 {
        let i = v.remove(0);
        let j = v.pop().unwrap();

        println!("+ {i} {j}");

        input_interactive! {
            pos: usize,
        };
        assert!(pos >= n + 1);

        let idx = lower_bound_int(0, v.len(), |idx| {
            println!("? {} {}", pos, v[idx]);
            input_interactive! {
                res: usize,
            };

            if res == 1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        v.insert(idx, pos);
    }

    println!("!");
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
