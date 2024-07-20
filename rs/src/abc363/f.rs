fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("1");
        return;
    }

    let d0 = iterate(n, |&z| z / 10).take_while(|&z| z > 0).count();

    let nums = (2usize..)
        .filter(|&x| {
            iterate(x, |&z| z / 10)
                .take_while(|&z| z > 0)
                .all(|z| z % 10 > 0)
        })
        .map(|x| {
            let y = iterate(x, |&z| z / 10)
                .take_while(|&z| z > 0)
                .map(|z| z % 10)
                .fold(0, |y, z| 10 * y + z);

            (x, y, x.saturating_mul(y))
        })
        .take_while(|&(x, _, z)| {
            2 * iterate(x, |&z| z / 10).take_while(|&z| z > 0).count() - 1 <= d0
        })
        .chain(
            (1..)
                .flat_map(|i| {
                    let d = (i + 1) / 2;

                    (10usize.pow(d - 1)..10usize.pow(d))
                        .filter(|&x| {
                            iterate(x, |&z| z / 10)
                                .take_while(|&z| z > 0)
                                .all(|z| z % 10 > 0)
                        })
                        .map(move |x| {
                            let z = if i % 2 == 0 {
                                iterate(x, |&z| z / 10)
                                    .take_while(|&z| z > 0)
                                    .map(|z| z % 10)
                                    .fold(x, |y, z| 10 * y + z)
                            } else {
                                iterate(x, |&z| z / 10)
                                    .take_while(|&z| z > 0)
                                    .map(|z| z % 10)
                                    .skip(1)
                                    .fold(x, |y, z| 10 * y + z)
                            };

                            (z, 0, z)
                        })
                })
                .take_while(|&(_, _, z)| z <= n)
                .skip(1),
        )
        .collect::<Vec<_>>();

    let nums = nums
        .into_iter()
        .filter(|&(_, _, z)| n % z == 0)
        .collect::<Vec<_>>();

    let mut memo = FxHashMap::default();
    if calc(n, false, &nums, &mut memo) {
        let (a, b) = successors(Some((n, false)), |&(m, use0)| {
            let i = memo.get(&(m, use0))?.unwrap();
            let z = nums[i].2;
            Some((m / z, use0 || nums[i].1 == 0))
        })
        .filter_map(|(m, use0)| Some(nums[memo.get(&(m, use0))?.unwrap()]))
        .sorted_by_key(|&(_, y, _)| y == 0)
        .fold((vec![], vec![]), |(mut a, mut b), (x, y, _)| {
            a.push(x);
            if y != 0 {
                b.push(y);
            }

            (a, b)
        });

        println!("{}", a.into_iter().chain(b.into_iter().rev()).join("*"));
    } else {
        println!("-1");
    }
}

fn calc(
    n: usize,
    use0: bool,
    nums: &[(usize, usize, usize)],
    memo: &mut FxHashMap<(usize, bool), Option<usize>>,
) -> bool {
    if n == 1 {
        return true;
    }

    if let Some(&ret) = memo.get(&(n, use0)) {
        return ret.is_some();
    }

    if let Some(i) = nums.iter().position(|&(_, y, z)| {
        (!use0 || y != 0) && n % z == 0 && calc(n / z, use0 || y == 0, nums, memo)
    }) {
        memo.insert((n, use0), Some(i));

        true
    } else {
        memo.insert((n, use0), None);

        false
    }
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
