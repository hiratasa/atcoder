fn main() {
    input! {
        t: usize,
    };

    // let rng = &mut SmallRng::seed_from_u64(42);
    // repeat_with(|| {
    //     let a = (0..100000)
    //         .map(|_| rng.random_range(1..=1000000000))
    //         .collect::<Vec<_>>();

    //     let n = a.len();
    //     let m = rng.random_range(2..=a.iter().sum::<usize>() - n) / 2 * 2 - 1;

    //     (n, m, a)
    // })
    // .for_each(|(n, m, a)| {
    // iproduct!(1..=6, 1..=4)
    //     .flat_map(|(n, m)| {
    //         repeat_n(1usize..=6, n)
    //             .multi_cartesian_product()
    //             .map(move |v| (n, m, v))
    //     })
    //     .filter(|(n, m, a)| (n + m) % 2 == 1 && n + m <= a.iter().sum::<usize>())
    //     .for_each(|(n, m, a)| {
    (0..t).for_each(|_| {
        input! {
            n: usize, m: usize,
            a: [usize; n],
        };

        let ans = lower_bound_int(1usize, 1 << 60, |b| {
            // 長さb以下の棒を floor((N+M)/2)本以下にできるか

            let mut smalls = a
                .iter()
                .copied()
                .filter(|&x| x <= b)
                .map(|x| (x, 1))
                .collect::<Vec<_>>();
            let mut larges = a
                .iter()
                .copied()
                .filter(|&x| x > b)
                .map(|x| (x, 1))
                .collect::<Vec<_>>();

            let mut remain = m;
            let mut c = 0usize;

            let mut num_2b1 = 0usize;
            let mut borders = vec![];
            while let Some((x, k)) = larges.pop() {
                let l = (0..60).rev().find(|&i| x >> i > b).unwrap();

                let y0 = x >> l;
                let y1 = y0 + 1;

                let t = k * ((1 << l) - 1);
                if t > remain {
                    remain = 0;
                } else {
                    let r = x % (1 << l);

                    if y0 == 2 * b + 1 {
                        num_2b1 += ((1 << l) - r) * k;
                        larges.push((y1, r * k));
                    } else {
                        borders.push((y0, ((1 << l) - r) * k));
                        if y1 == 2 * b + 1 {
                            num_2b1 += r * k;
                        } else {
                            borders.push((y1, r * k));
                        }
                    }
                    remain -= t;
                }
            }

            {
                let t = min(remain, num_2b1);
                let y0 = b;
                let y1 = b + 1;
                smalls.push((y0, t));
                borders.push((y1, t));
                remain -= t;
            }

            for (x, k) in smalls {
                assert!(x <= b);

                let k2 = k * x - k;
                let t = min(remain, k2);
                c += t + k;
                remain -= t;
            }

            borders.sort();
            borders.reverse();
            for (x, k) in borders {
                assert!((x + 1) / 2 <= b);

                let p = min((remain + x - 2) / (x - 1), k);
                c += p + min(remain, p * (x - 1));
                remain -= min(remain, p * (x - 1));
            }

            if c <= (n + m) / 2 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // let ans0 = solve0(&a, m, &mut FxHashMap::default());
        // assert_eq!(ans, ans0, "{n} {m}\n{}", a.iter().join(" "));
        println!("{ans}");
    });
}

fn solve0(ar: &[usize], m: usize, memo: &mut FxHashMap<Vec<usize>, usize>) -> usize {
    if m == 0 {
        assert!(ar.len() % 2 == 1);
        return ar[ar.len() / 2];
    }

    if let Some(&ans) = memo.get(ar) {
        return ans;
    }

    let mut r = 0;
    for i in 0..ar.len() {
        let x = ar[i];
        if x <= 1 {
            continue;
        }
        let y0 = x / 2;
        let y1 = (x + 1) / 2;

        let mut ar = ar.to_vec();
        ar.remove(i);
        ar.push(y0);
        ar.push(y1);
        ar.sort();

        r = max(solve0(&ar, m - 1, memo), r);
    }

    memo.insert(ar.to_vec(), r);

    r
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
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
