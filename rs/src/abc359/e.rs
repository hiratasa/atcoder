fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };

    println!(
        "{}",
        h.into_iter()
            .enumerate()
            .scan(
                (vec![], 0),
                |(t, s): &mut (Vec<(usize, usize, usize)>, usize), (i, x)| {
                    while let Some(&(y, _, ss)) = t.last() {
                        if y >= x {
                            break;
                        }

                        *s -= ss;
                        t.pop();
                    }

                    let l = i + 1 - t.last().map_or(0, |&(_, j, _)| j + 1);

                    t.push((x, i, l * x));
                    *s += l * x;

                    Some(*s + 1)
                },
            )
            .join(" ")
    );
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::*,
    mem::{replace, take},
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
