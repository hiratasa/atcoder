fn main() {
    input! {
        t: usize,
        cases: [(Chars, Chars); t],
    };

    cases
        .into_iter()
        .map(|(a, b)| {
            let [a, b] = [a, b].map(|a| {
                let mut buf = vec![];
                for x in a {
                    buf.push(x);
                    if buf.len() >= 4 && &buf[buf.len() - 4..] == &['(', 'x', 'x', ')'] {
                        buf.truncate(buf.len() - 4);
                        buf.push('x');
                        buf.push('x');
                    }
                }
                buf
            });

            a == b
        })
        .for_each(|ans| {
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        });
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
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
