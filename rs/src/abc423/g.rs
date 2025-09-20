fn main() {
    input! {
        t: usize,
        ks: [(i64, Digits); t],
    };

    ks.into_iter()
        .map(|(k, s)| {
            let n = s.len();
            let m = iterate(k, |&x| x / 10).take_while(|&x| x > 0).count();
            let r = s.iter().copied().fold(0, |r, d| (r * 10 + d) % k);

            let rn = pow_mod(10, n, k as usize) as i64;

            (0..=m)
                .map(|i| {
                    let j = m - i;

                    let z = 10i64.pow(i as u32) % k;
                    let w = rn * z % k;
                    let rr = r * z % k;

                    let (x, y) = if i < j {
                        (0..10i64.pow(i as u32))
                            .map(|y| {
                                // x * w + q * k = -rr - y
                                let c = (-rr - y).rem_euclid(k);
                                let (x, q, g) = extgcd(w, k);

                                if c % g == 0 {
                                    Some((((c / g) * x).rem_euclid(k / g), y))
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .min()?
                    } else {
                        (0..10i64.pow(j as u32))
                            .map(|x| {
                                // y = -rr - x * w (mod k)
                                let y = (-rr - x * w).rem_euclid(k);

                                if y < 10i64.pow(i as u32) {
                                    Some((x, y))
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .min()?
                    };

                    Some(
                        iterate(x, |&xx| xx / 10)
                            .take_while(|&xx| xx > 0)
                            .map(|xx| xx % 10)
                            .collect::<Vec<_>>()
                            .into_iter()
                            .rev()
                            .chain(s.iter().copied())
                            .chain(
                                iterate(y, |&yy| yy / 10)
                                    .take_while(|&yy| yy > 0)
                                    .map(|yy| yy % 10)
                                    .chain(repeat(0))
                                    .take(i)
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                    .rev(),
                            )
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .min_by(|a, b| {
                    if a.len() != b.len() {
                        a.len().cmp(&b.len())
                    } else {
                        a.cmp(b)
                    }
                })
                .unwrap()
        })
        .for_each(|ans| {
            println!("{}", ans.iter().join(""));
        });
}

#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (_zero, g, _u, v) = std::iter::successors(Some((a, b, 1, 0)), |&(a, b, u, v)| {
        if a == 0 {
            None
        } else {
            Some((b % a, a, -u * (b / a) + v, u))
        }
    })
    .last()
    .unwrap();

    (v, (g - a * v) / b, g)
}

pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
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
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

use proconio::source::{Readable, Source};
enum Digits {}
impl Readable for Digits {
    type Output = Vec<i64>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<i64> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect()
    }
}
