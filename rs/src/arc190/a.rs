fn main() {
    // let mut rng = SmallRng::seed_from_u64(42);

    // loop {
    //     let n = rng.random_range(1..=5);
    //     let m = rng.random_range(1..=5);
    //     let lr = repeat_with(|| (rng.random_range(0..n), rng.random_range(1..=n)))
    //         .filter(|&(l, r)| l < r)
    //         .take(m)
    //         .collect::<Vec<_>>();

    input! {
        n: usize, m: usize,
        lr: [(Usize1, usize); m],
    };

    // let ans0 = solve0(n, &lr);

    if let Some(v) = solve(n, &lr) {
        println!("{}", v.len());
        // assert!(
        //     ans0.is_some(),
        //     "{n} {m}\n{}",
        //     lr.iter().map(|(l, r)| format!("{l} {r}")).join("\n")
        // );
        // assert_eq!(
        //     ans0.as_ref().unwrap().iter().filter(|&&x| x > 0).count(),
        //     v.len(),
        //     "{n} {m}\n{}",
        //     lr.iter().map(|(l, r)| format!("{l} {r}")).join("\n")
        // );
        let mut ops = vec![0; m];
        for (i, x) in v {
            ops[i] = x;
        }
        println!("{}", ops.iter().join(" "));
        // assert!(
        //     check(n, &lr, &ops),
        //     "{n} {m}\n{}",
        //     lr.iter().map(|(l, r)| format!("{l} {r}")).join("\n")
        // );
    } else {
        // assert!(
        //     ans0.is_none(),
        //     "{n} {m}\n{}",
        //     lr.iter().map(|(l, r)| format!("{l} {r}")).join("\n")
        // );
        println!("-1");
    }
    // }
}

fn solve(n: usize, lr: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
    let m = lr.len();

    // 全体と等しいものが存在
    if let Some(i) = lr.iter().copied().position(|(l, r)| (l, r) == (0, n)) {
        return Some(vec![(i, 1)]);
    }

    if m == 1 {
        return None;
    }

    let idxs = (0..m)
        .sorted_by_key(|&i| (lr[i].0, Reverse(lr[i].1)))
        .collect::<Vec<_>>();

    // 2個で全体を覆える
    if let Some((i, j)) = (|| {
        let first_max = lr
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, (l, r))| l == 0)
            .max_by_key(|&(_, (_, r))| r)?;

        let last_min = lr
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, (l, r))| r == n)
            .min_by_key(|&(_, (l, _))| l)?;

        if first_max.1.1 >= last_min.1.0 {
            return Some((first_max.0, last_min.0));
        }

        None
    })() {
        return Some(vec![(i, 1), (j, 1)]);
    }

    // 包含関係にある
    if let Err((i, j)) = idxs
        .iter()
        .copied()
        .try_fold(BTreeSet::new(), |mut set, i| {
            let (l, r) = lr[i];

            while matches!(set.first(), Some(&(rr, _, _)) if rr <= l) {
                set.pop_first();
            }

            if let Some((_, _, j)) = set.range((r, 0, 0)..).next().copied() {
                return Err((i, j));
            }

            set.insert((r, l, i));
            Ok(set)
        })
    {
        return Some(vec![(i, 2), (j, 1)]);
    }

    // 排他関係
    if let Some((i, j)) = (|| {
        let rmin = lr
            .iter()
            .copied()
            .enumerate()
            .min_by_key(|&(_, (_, r))| r)?;

        let lmax = lr
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, (l, _))| l)?;

        if rmin.1.1 <= lmax.1.0 {
            return Some((rmin.0, lmax.0));
        }

        None
    })() {
        return Some(vec![(i, 2), (j, 2)]);
    }

    if m == 2 {
        return None;
    }

    Some(vec![(idxs[0], 1), (idxs[1], 2), (idxs[2], 1)])
}

fn solve0(n: usize, lr: &[(usize, usize)]) -> Option<Vec<usize>> {
    let m = lr.len();
    (0..m)
        .map(|_| (0..=2))
        .multi_cartesian_product()
        .filter(|ops| check(n, &lr, &ops))
        .min_by_key(|ops| ops.iter().filter(|&&x| x > 0).count())
}

fn check(n: usize, lr: &[(usize, usize)], ops: &[usize]) -> bool {
    let mut values = vec![0; n];

    ops.iter().copied().enumerate().for_each(|(i, op)| {
        let (l, r) = lr[i];
        if op == 1 {
            for j in l..r {
                values[j] = 1;
            }
        } else if op == 2 {
            for j in 0..l {
                values[j] = 1;
            }
            for j in r..n {
                values[j] = 1;
            }
        }
    });

    values.iter().copied().all(|x| x == 1)
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
