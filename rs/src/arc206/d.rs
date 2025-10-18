fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    };

    cases
        .into_iter()
        .map(|(n, k)| {
            let ans = match (n, k) {
                (_, 0) if n < 8 => None,
                (_, 0) => Some(
                    (3..=n - 4)
                        .chain((n - 1..=n).rev())
                        .chain((1..=2).rev())
                        .chain((n - 3..=n - 2))
                        .collect::<Vec<_>>(),
                ),
                (1, 1) => Some(vec![1]),
                (_, 1) if n < 5 => None,
                (_, 1) => Some(
                    (2..=n - 3)
                        .chain(once(n))
                        .chain(once(n - 2))
                        .chain(once(1))
                        .chain(once(n - 1))
                        .collect::<Vec<_>>(),
                ),
                (_, _) => Some(
                    (1..n - k + 1)
                        .chain((n - k + 1..=n).rev())
                        .collect::<Vec<_>>(),
                ),
            };

            // if let Some(ans) = ans.as_ref() {
            //     check(n, k, ans);
            // }

            ans
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{}", ans.iter().join(" "));
            } else {
                println!("-1");
            }
        })
}

fn check(n: usize, k: usize, ans: &[usize]) {
    assert_eq!(ans.len(), n, "{n} {k} {ans:?}");

    assert_eq!(
        ans.iter().copied().sorted().collect::<Vec<_>>(),
        (1..=n).collect::<Vec<_>>(),
        "{n} {k} {ans:?}"
    );

    let (lis, t0) = ans
        .iter()
        .fold((vec![], vec![0; n + 1]), |(mut lis, mut t), &x| {
            let i = lis.binary_search(&x).unwrap_err();

            if i == lis.len() {
                lis.push(x);
            } else {
                lis[i] = x;
            }

            t[x] = i;

            (lis, t)
        });
    let (lds, t1) = ans
        .iter()
        .fold((vec![], vec![0; n + 1]), |(mut lis, mut t), &x| {
            let i = lis.binary_search(&Reverse(x)).unwrap_err();

            if i == lis.len() {
                lis.push(Reverse(x));
            } else {
                lis[i] = Reverse(x);
            }

            t[x] = i;

            (lis, t)
        });
    let (_, t2) = ans
        .iter()
        .rev()
        .fold((vec![], vec![0; n + 1]), |(mut lis, mut t), &x| {
            let i = lis.binary_search(&x).unwrap_err();

            if i == lis.len() {
                lis.push(x);
            } else {
                lis[i] = x;
            }

            t[x] = i;

            (lis, t)
        });
    let (_, t3) = ans
        .iter()
        .rev()
        .fold((vec![], vec![0; n + 1]), |(mut lis, mut t), &x| {
            let i = lis.binary_search(&Reverse(x)).unwrap_err();

            if i == lis.len() {
                lis.push(Reverse(x));
            } else {
                lis[i] = Reverse(x);
            }

            t[x] = i;

            (lis, t)
        });

    let m = (1..=n)
        .filter(|&i| t0[i] + t3[i] == lis.len() - 1 && t1[i] + t2[i] == lds.len() - 1)
        .count();

    assert_eq!(m, k, "{n} {k} but {m}; {ans:?}");
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
