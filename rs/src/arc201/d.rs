fn main() {
    input! {
        t: usize,
    };

    (0..t)
        .map(|_| {
            input! {
                n: usize, m: usize,
                a: [usize; n],
                b: [usize; n],
            };
            // let n = 300000;
            // let m = 1000000000;
            // let mut rng = SmallRng::seed_from_u64(42);
            // let a = (0..n).map(|_| rng.random_range(0..m)).collect::<Vec<_>>();
            // let b = (0..n).map(|_| rng.random_range(0..m)).collect::<Vec<_>>();

            let vals = chain(a.iter().copied(), b.iter().copied())
                .flat_map(|x| [x, (m - x) % m])
                .sorted()
                .dedup()
                .collect::<Vec<_>>();
            let idxs = vals
                .iter()
                .enumerate()
                .map(|(i, &v)| (v, i))
                .collect::<FxHashMap<_, _>>();
            let invvals = vals.iter().map(|&x| ((m - x) % m)).collect::<Vec<_>>();
            let invs = vals
                .iter()
                .map(|&x| idxs[&((m - x) % m)])
                .collect::<Vec<_>>();

            let l = idxs.len();
            let freqs_a = a.iter().fold(vec![0; l], |mut freqs, &v| {
                freqs[idxs[&v]] += 1;
                freqs
            });
            let freqs_b = b.iter().fold(vec![0; l], |mut freqs, &v| {
                freqs[idxs[&v]] += 1;
                freqs
            });

            // let sums_a = once(0).chain(freqs_a.iter().copied().cycle().map(|x| x as i64).take(2 * l)).cumsum::<i64>().collect::<Vec<_>>();
            let sums_b = once(0)
                .chain(
                    freqs_b
                        .iter()
                        .copied()
                        .cycle()
                        .map(|x| x as i64)
                        .take(2 * l),
                )
                .cumsum::<i64>()
                .collect::<Vec<_>>();

            let c = once(0)
                .chain(
                    (0..l)
                        .map(|i| {
                            let x = vals[i];
                            let y = (m - x) % m;
                            let j = idxs[&y];
                            freqs_b[j] as i64 - freqs_a[i] as i64
                        })
                        .cycle()
                        .take(2 * l),
                )
                .cumsum::<i64>()
                .collect::<Vec<_>>();

            lower_bound_int(0, m, |r| {
                let d = (0..l)
                    .scan(l - 1, |j1, i| {
                        let x = vals[i];
                        let y = invvals[i];
                        let j0 = invs[i];
                        let j2 = if vals[l - 1] <= (y + r) % m {
                            l
                        } else {
                            if vals[*j1] <= (y + r) % m {
                                *j1 = l - 1;
                            }
                            while *j1 > 0 && vals[*j1 - 1] > (y + r) % m {
                                *j1 -= 1;
                            }
                            *j1
                        };
                        let j2 = if j2 < j0 + 1 { j2 + l } else { j2 };
                        Some(sums_b[j2] - sums_b[j0 + 1])
                    })
                    .collect::<Vec<_>>();

                let ok = (1..=2 * l)
                    .scan(c[0] - d[0], |ma, i| {
                        let ok = c[i] >= *ma;
                        *ma = max(*ma, c[i] - d[i % l]);
                        Some(ok)
                    })
                    .all(|ok| ok);

                if ok {
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
