fn main() {
    input! {
        n: usize, c: usize,
        wvk: [(usize, usize, usize); n],
    };

    let raw_items1 = wvk
        .iter()
        .copied()
        .filter(|&(w, _, _)| w == 1)
        .map(|(_, v, k)| (v, k))
        .sorted_by_key(|&(v, _)| Reverse(v))
        .collect::<Vec<_>>();
    let raw_items2 = wvk
        .iter()
        .copied()
        .filter(|&(w, _, _)| w == 2)
        .map(|(_, v, k)| (v, k))
        .sorted_by_key(|&(v, _)| Reverse(v))
        .collect::<Vec<_>>();
    let items3 = wvk
        .iter()
        .copied()
        .filter(|&(w, _, _)| w == 3)
        .map(|(_, v, k)| (v, k))
        .sorted_by_key(|&(v, _)| Reverse(v))
        .collect::<Vec<_>>();

    let t = (0..=1)
        .map(|r| {
            raw_items2
                .iter()
                .copied()
                .map(|(v, k)| (v, k))
                .chain(
                    raw_items1
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(i, (v, k))| if r == 0 || i > 0 { (v, k) } else { (v, k - 1) })
                        .filter(|&(_, k)| k > 0)
                        .chain(once((0, 2)))
                        .scan(0, |c, (v, k)| {
                            if *c == 0 {
                                if k % 2 == 0 {
                                    *c = 0;
                                    Some([Some((2 * v, k / 2)), None])
                                } else {
                                    *c = v;
                                    Some([Some((2 * v, k / 2)), None])
                                }
                            } else {
                                let p = *c;
                                if k % 2 == 0 {
                                    *c = v;
                                    Some([Some((p + v, 1)), Some((2 * v, k / 2 - 1))])
                                } else {
                                    *c = 0;
                                    Some([Some((p + v, 1)), Some((2 * v, k / 2))])
                                }
                            }
                        })
                        .flatten()
                        .flatten(),
                )
                .sorted_by_key(|&(v, _)| Reverse(v))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sums_v = (0..2)
        .map(|r| {
            once(0)
                .chain(t[r].iter().map(|(v, k)| v * k))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sums_k = (0..2)
        .map(|r| {
            once(0)
                .chain(t[r].iter().map(|(_v, k)| *k))
                .cumsum::<usize>()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n3 = items3.iter().map(|&(_, k)| k).sum::<usize>();

    let ans = (0..=min(n3, 1))
        .map(|z| {
            let r = (c % 2) ^ z;
            let mut c = c - r;
            let mut v0 = 0;

            if r == 1 && !raw_items1.is_empty() {
                v0 += raw_items1[0].0
            };
            if z == 1 {
                if c < 3 {
                    return v0;
                }
                v0 += items3[0].0;
                c -= 3;
            }
            let c = c;
            let v0 = v0;

            let items3 = if z == 1 {
                &items3
                    .iter()
                    .enumerate()
                    .map(|(i, &(v, k))| if i == 0 { (v, k - 1) } else { (v, k) })
                    .filter(|&(v, k)| k > 0)
                    .collect::<Vec<_>>()
            } else {
                &items3
            };

            let items2 = &t[r];
            let sums2_v = &sums_v[r];
            let sums2_k = &sums_k[r];
            let n2 = *sums2_k.last().unwrap();

            let sums3_v = once(0)
                .chain(items3.iter().map(|&(v, k)| v * k))
                .cumsum::<usize>()
                .collect::<Vec<_>>();
            let sums3_k = once(0)
                .chain(items3.iter().map(|&(_v, k)| k))
                .cumsum::<usize>()
                .collect::<Vec<_>>();
            let n3 = *sums3_k.last().unwrap();

            assert_eq!(c % 2, 0);

            let calc_items_v =
                |items: &[(usize, usize)], sums_v: &[usize], sums_k: &[usize], m: usize| {
                    let l = sums_k
                        .binary_search_by(|&k| k.cmp(&m).then(Ordering::Less))
                        .unwrap_err()
                        - 1;
                    if l < items.len() {
                        let remainig = m - sums_k[l];
                        sums_v[l] + items[l].0 * remainig
                    } else {
                        sums_v[l]
                    }
                };

            let calc_v = |s: usize| {
                let m2 = (c / 2) % 3 + 3 * s;
                assert_eq!((c - 2 * m2) % 3, 0);
                let m3 = (c - 2 * m2) / 3;

                let v3 = calc_items_v(&items3, &sums3_v, &sums3_k, m3);
                let v2 = calc_items_v(&items2, &sums2_v, &sums2_k, m2);

                v0 + v2 + v3
            };

            let s = lower_bound_int(0, c / 2 / 3, |s| {
                let x = calc_v(s);
                let y = calc_v(s + 1);

                if x <= y {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            calc_v(s)
        })
        .max()
        .unwrap();

    println!("{ans}");
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
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
