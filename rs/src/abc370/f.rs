fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    let l = a.iter().sum::<usize>();

    let calc_nexts = |x: usize| {
        let next = a
            .iter()
            .scan((0, 0), |(i, s), aa| {
                while *s < x {
                    *s += a[*i % n];
                    *i += 1;
                }

                *s -= aa;

                Some(*i)
            })
            .collect::<Vec<_>>();

        let mut nexts = vec![vec![]; 20];
        nexts[0] = next;
        for i in 1..20 {
            nexts[i].resize(n, 0);
            for j in 0..n {
                nexts[i][j] = nexts[i - 1][nexts[i - 1][j] % n] + nexts[i - 1][j] / n * n;
            }
        }

        nexts
    };

    let is_ok = |nexts: &[Vec<usize>], i: usize| {
        (0..20)
            .filter(|&j| k & (1 << j) > 0)
            .try_fold(i, |c, j| {
                let c = nexts[j][c % n] + c / n * n;

                if c > n + i { None } else { Some(c) }
            })
            .is_some()
    };

    let x = lower_bound_int(1, l, |x| {
        let nexts = calc_nexts(x);

        let ok = (0..n).any(|i| is_ok(&nexts, i));

        if ok {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }) - 1;

    let nexts = calc_nexts(x);
    let y = (0..n).filter(|&i| !is_ok(&nexts, i)).count();

    println!("{x} {y}");
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
