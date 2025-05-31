fn main() {
    input! {
        t: usize,
        cases: [(i128, i128, i128, i128); t],
    };

    // let mut rng = SmallRng::seed_from_u64(42);

    // repeat_with(|| {
    //     let a = rng.gen_range(1..=20);
    //     let b = rng.gen_range(1..=20);
    //     let c = rng.gen_range(1..=20);
    //     let d = rng.gen_range(1..=20);

    //     match (a * d).cmp(&(b * c)) {
    //         Ordering::Less => Some((a, b, c, d)),
    //         Ordering::Equal => None,
    //         Ordering::Greater => Some((c, d, a, b)),
    //     }
    // })
    // .flatten()
    cases
        .into_iter()
        .map(|(a, b, c, d)| {
            let g = gcd(a, b);
            let (a, b) = (a / g, b / g);
            let g = gcd(c, d);
            let (c, d) = (c / g, d / g);

            let (x0, y0) = if b < d { (c, d) } else { (a, b) };

            // let ans0 = solve0(a, b, c, d);

            if a / b + 1 < (c + d - 1) / d {
                // assert_eq!(ans0.1, 1, "{a} {b} {c} {d}");
                return 1;
            }

            let (p, q) = (0..)
                .try_fold((0, 1, 1, 0, x0, y0), |(p2, p1, q2, q1, x, y), _| {
                    if y == 0 {
                        let g = gcd(a + c, b + d);
                        return Err(((a + c) / g, (b + d) / g));
                    }

                    let z = x / y;

                    let p = z * p1 + p2;
                    let q = z * q1 + q2;

                    if p * d <= q * c && p * b >= q * a {
                        let n = lower_bound_int(0, z, |i| {
                            let p = i * p1 + p2;
                            let q = i * q1 + q2;
                            if p * d < q * c && p * b > q * a {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        });
                        let p = n * p1 + p2;
                        let q = n * q1 + q2;
                        if p * d < q * c && p * b > q * a {
                            return Err((p, q));
                        }
                    }

                    let xx = y;
                    let yy = x - z * y;
                    Ok((p1, p, q1, q, xx, yy))
                })
                .unwrap_err();

            // assert_eq!(ans0.1, q, "{a} {b} {c} {d}");

            q
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}

fn solve0(a: i128, b: i128, c: i128, d: i128) -> (i128, i128) {
    let n0 = a / b;
    let n1 = (c + d - 1) / d;
    assert!(
        n0 < n1,
        "n0: {n0}, n1: {n1}, a: {a}, b: {b}, c: {c}, d: {d}"
    );

    (1..)
        .find_map(|q| {
            (n0 * q..n1 * q)
                .find(|&p| p * d < q * c && p * b > q * a)
                .map(|p| (p, q))
        })
        .unwrap()
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
