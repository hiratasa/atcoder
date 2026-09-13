fn main() {
    input! {
        n: usize, m: usize, k: usize,
        t: Chars,
        s: [Chars; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    };

    let mut corrects = (0..n)
        .map(|i| (0..k).map(|j| s[i][j] == t[j]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut states = vec![Node::new()];
    for i in 0..n {
        add(&mut states, &corrects[i], 0, 0);
    }

    for (i, j) in queries {
        remove(&mut states, &corrects[i], 0, 0);
        corrects[i][j] = !corrects[i][j];
        add(&mut states, &corrects[i], 0, 0);

        let ans = passed(&states, &corrects[i], 0, 0, m);

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

struct Node {
    total: usize,
    children: [Option<usize>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            total: 0,
            children: [None; 2],
        }
    }
}

fn add(states: &mut Vec<Node>, corrects: &[bool], idx: usize, i_state: usize) {
    let k = corrects.len();

    states[i_state].total += 1;

    if idx == k {
        return;
    }

    let c = corrects[idx] as usize;

    let ch = if let Some(ch) = states[i_state].children[c] {
        ch
    } else {
        states.push(Node::new());
        states.len() - 1
    };
    states[i_state].children[c] = Some(ch);
    add(states, corrects, idx + 1, ch);
}

fn remove(states: &mut Vec<Node>, corrects: &[bool], idx: usize, i_state: usize) {
    let k = corrects.len();

    states[i_state].total -= 1;

    if idx == k {
        return;
    }

    let c = corrects[idx] as usize;

    let ch = states[i_state].children[c].unwrap();
    remove(states, corrects, idx + 1, ch);
}

fn passed(states: &[Node], corrects: &[bool], idx: usize, i_state: usize, remain: usize) -> bool {
    if remain == 0 {
        return false;
    }

    let k = corrects.len();
    if idx == k {
        return false;
    }

    let num_correct = if let Some(ch) = states[i_state].children[1] {
        states[ch].total
    } else {
        0
    };

    if corrects[idx] {
        if num_correct <= remain {
            true
        } else {
            passed(
                states,
                corrects,
                idx + 1,
                states[i_state].children[1].unwrap(),
                remain,
            )
        }
    } else {
        if num_correct <= remain {
            passed(
                states,
                corrects,
                idx + 1,
                states[i_state].children[0].unwrap(),
                remain - num_correct,
            )
        } else {
            false
        }
    }
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
