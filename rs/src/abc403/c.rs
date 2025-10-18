fn main() {
    input! {
        n: usize, m: usize, q: usize,
    };

    let mut states = vec![State::Invidiual(FxHashSet::default()); n];

    for _ in 0..q {
        input! {
            ty: usize,
        };

        match ty {
            1 => {
                input! { x: Usize1, y: usize };

                match &mut states[x] {
                    State::Invidiual(set) => {
                        set.insert(y);
                    }
                    State::All => {}
                }
            }
            2 => {
                input! { x: Usize1 };

                states[x] = State::All;
            }
            3 => {
                input! { x: Usize1, y: usize };

                match &states[x] {
                    State::Invidiual(set) => {
                        if set.contains(&y) {
                            println!("Yes");
                        } else {
                            println!("No");
                        }
                    }
                    State::All => {
                        println!("Yes");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Invidiual(FxHashSet<usize>),
    All,
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
