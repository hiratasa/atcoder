fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    };

    cases
        .into_iter()
        .map(|(m, k)| {
            (
                if m < 7 {
                    (1..1 << m)
                        .map(|s| (0..m).filter(|&i| s & (1 << i) != 0).collect::<Vec<_>>())
                        .find(|v| {
                            let t = chain(
                                v.iter().copied().map(|x| x + x),
                                v.iter().copied().tuple_combinations().map(|(x, y)| x + y),
                            )
                            .fold(vec![false; m], |mut t, x| {
                                t[x % m] = true;
                                t
                            });

                            t.iter().filter(|&&x| x).count() == k
                        })
                } else if k == 2 {
                    if m % 2 == 0 {
                        Some(vec![0, m / 2])
                    } else {
                        None
                    }
                } else if k == 4 {
                    if m % 4 == 0 {
                        Some(vec![0, m / 4, m / 2, 3 * m / 4])
                    } else {
                        None
                    }
                } else if k == m {
                    Some((0..m).collect::<Vec<_>>())
                } else if k % 2 == 0 {
                    let l = k / 2;
                    Some((0..l - 1).chain(once(l)).collect::<Vec<_>>())
                } else {
                    let l = k / 2;
                    Some((0..=l).collect::<Vec<_>>())
                },
                m,
                k,
            )
        })
        .for_each(|(ans, m, k)| {
            if let Some(ans) = ans {
                // verify
                // let k_actual = chain(
                //     ans.iter().copied().map(|x| x + x),
                //     ans.iter().copied().tuple_combinations().map(|(x, y)| x + y),
                // )
                // .fold(vec![false; m], |mut t, x| {
                //     t[x % m] = true;
                //     t
                // })
                // .into_iter()
                // .filter(|&x| x)
                // .count();
                // assert_eq!(k_actual, k, "m = {}, k = {}, ans = {:?}", m, k, ans);

                println!("Yes");
                println!("{}", ans.len());
                println!("{}", ans.iter().join(" "));
            } else {
                println!("No");
            }
        })
}

use std::vec;
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
