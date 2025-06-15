fn main() {
    input! {
        t: usize,
        cases: [(usize, usize, usize); t],
    };

    let mut rng = SmallRng::seed_from_u64(42);

    cases
        .into_iter()
        .map(|(a1, a2, a3)| {
            let (a1, a2, swap) = if a1 < a2 {
                (a1, a2, false)
            } else {
                (a2, a1, true)
            };

            if a3 < a2 || a3 > a1 + a2 {
                None
            } else if a2 == a3 {
                let x1 = 10usize.pow(a1 as u32 - 1);
                let x2 = 10usize.pow(a2 as u32 - 1);
                if swap {
                    Some((x2, x1))
                } else {
                    Some((x1, x2))
                }
            } else {
                let k: usize = a1 + a2 - a3;
                let t = 10usize.pow(k as u32);
                let b1 = a1 - k;
                let b2 = a2 - k;

                let (x1, x2) = loop {
                    let x1 = rng
                        .gen_range(10usize.pow(b1 as u32 - 1) / 2..10usize.pow(b1 as u32) / 2)
                        * t
                        * 2;
                    let x2 = (rng
                        .gen_range(10usize.pow(b2 as u32 - 1) / 2..10usize.pow(b2 as u32) / 2)
                        * 2
                        + 1)
                        * t;
                    let g = gcd(x1, x2);
                    let lcm = (x1 / g).checked_mul(x2);

                    if let Some(lcm) = lcm {
                        if 10usize.pow(a3 as u32 - 1) <= lcm && lcm < 10usize.pow(a3 as u32) {
                            break (x1, x2);
                        }
                    }
                };

                if swap {
                    Some((x2, x1))
                } else {
                    Some((x1, x2))
                }
            }
        })
        .for_each(|ans| {
            if let Some((x1, x2)) = ans {
                println!("Yes");
                println!("{x1} {x2}");
            } else {
                println!("No");
            }
        })
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
