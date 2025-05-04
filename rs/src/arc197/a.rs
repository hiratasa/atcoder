fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            h: usize, w: usize,
            s: Chars,
        };

        let nd = s.iter().filter(|&&c| c == 'D').count();
        let nr = s.iter().filter(|&&c| c == 'R').count();
        let md = h - 1 - nd;
        let mr = w - 1 - nr;

        let [t0, t1] = array::from_fn(|idx| {
            s.iter()
                .copied()
                .scan(0, |i, c| match c {
                    'D' | 'R' => Some(c),
                    _ => {
                        *i += 1;
                        if idx == 0 {
                            if *i <= md {
                                Some('D')
                            } else {
                                Some('R')
                            }
                        } else {
                            if *i <= mr {
                                Some('R')
                            } else {
                                Some('D')
                            }
                        }
                    }
                })
                .scan((0, 0), |(i, j), c| {
                    let ij = (*i, *j);
                    if c == 'D' {
                        *i += 1;
                    } else {
                        *j += 1;
                    }
                    Some(ij)
                })
                .chain(once((h - 1, w - 1)))
                .fold(vec![usize::MAX; h], |mut t, (i, j)| {
                    if t[i] == usize::MAX || idx == 1 {
                        t[i] = j;
                    }
                    t
                })
        });

        let ans = (0..h).map(|i| t1[i] - t0[i] + 1).sum::<usize>();

        println!("{ans}");
    }
}

use std::array;
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
