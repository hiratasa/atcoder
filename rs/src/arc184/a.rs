fn main() {
    input_interactive! {
        n: usize, m: usize, q: usize,
    };

    let mut t = (1..=n).map(|x| vec![x]).collect::<Vec<_>>();

    let mut non_target = None;
    let mut targets = vec![];

    for _ in 0..4 {
        let mut next = vec![];

        while !t.is_empty() {
            if t.len() >= 2 {
                let a = t.pop().unwrap();
                let b = t.pop().unwrap();

                println!("? {} {}", a[0], b[0]);

                input_interactive! {
                    is_diff: usize,
                };

                if is_diff == 0 {
                    let mut a = a;
                    a.extend(b);
                    next.push(a);
                } else {
                    targets.push((a, b));
                }
            } else {
                next.push(t.pop().unwrap());
            }
        }

        t = next;
        t.reverse();
    }

    for a in t {
        if a.len() <= m {
            targets.push((a, vec![]));
        } else {
            non_target = Some(a[0]);
        }
    }

    let non_target = non_target.unwrap();

    let mut ans = vec![];
    for (x, y) in targets {
        println!("{non_target} {}", x[0]);

        input_interactive! {
            is_diff: usize,
        };

        if is_diff == 0 {
            ans.extend(y);
        } else {
            ans.extend(x);
        }
    }

    ans.sort();
    println!("! {}", ans.into_iter().join(" "));
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
