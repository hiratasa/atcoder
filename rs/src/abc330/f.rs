use std::{
    cmp::{Ordering, min},
    iter::once,
};

use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize, k: i64,
        xy: [(i64, i64); n],
    };

    let xs = xy.iter().map(|&(x, _)| x).sorted().collect::<Vec<_>>();
    let ys = xy.iter().map(|&(_, y)| y).sorted().collect::<Vec<_>>();

    let xsum = once(0)
        .chain(xs.iter().copied())
        .cumsum::<i64>()
        .collect::<Vec<_>>();
    let ysum = once(0)
        .chain(ys.iter().copied())
        .cumsum::<i64>()
        .collect::<Vec<_>>();

    let ans = lower_bound_int(0i64, 1000000000, |r| {
        let num_x0 = xs
            .iter()
            .copied()
            .enumerate()
            .scan(0, |j, (i, x)| {
                while *j < n && xs[*j] < x + r {
                    *j += 1;
                }

                Some((x * i as i64 - xsum[i]) + ((xsum[n] - xsum[*j]) - (x + r) * (n - *j) as i64))
            })
            .min()
            .unwrap();
        let num_x1 = xs
            .iter()
            .copied()
            .enumerate()
            .scan(0, |j, (i, x)| {
                while *j < n && xs[*j] < x - r {
                    *j += 1;
                }

                Some(((x - r) * *j as i64 - xsum[*j]) + ((xsum[n] - xsum[i]) - x * (n - i) as i64))
            })
            .min()
            .unwrap();
        let num_y0 = ys
            .iter()
            .copied()
            .enumerate()
            .scan(0, |j, (i, x)| {
                while *j < n && ys[*j] < x + r {
                    *j += 1;
                }

                Some((x * i as i64 - ysum[i]) + ((ysum[n] - ysum[*j]) - (x + r) * (n - *j) as i64))
            })
            .min()
            .unwrap();
        let num_y1 = ys
            .iter()
            .copied()
            .enumerate()
            .scan(0, |j, (i, x)| {
                while *j < n && ys[*j] < x - r {
                    *j += 1;
                }

                Some(((x - r) * *j as i64 - ysum[*j]) + ((ysum[n] - ysum[i]) - x * (n - i) as i64))
            })
            .min()
            .unwrap();

        if min(num_x0, num_x1) + min(num_y0, num_y1) <= k {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("{ans}");
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
