fn main() {
    input! {
        a: [usize; 6],
    };

    let mut states = vec![];
    all_states(0, 0, [0; 6], &mut states);

    let mut idxs = [1 << 50; 6usize.pow(6)];
    let encode = |state: &[usize; 6]| {
        (((((state[0] * 6 + state[1]) * 6 + state[2]) * 6) + state[3]) * 6 + state[4]) * 6
            + state[5]
    };
    for (idx, state) in states.iter().enumerate() {
        idxs[encode(state)] = idx;
    }

    let freq = (0..5).map(|_| (0..6)).multi_cartesian_product().fold(
        vec![0; states.len()],
        |mut freq, v| {
            let mut state = [0; 6];
            for i in v {
                state[i] += 1;
            }
            let idx = idxs[encode(&state)];
            freq[idx] += 1;
            freq
        },
    );

    let scores = states
        .iter()
        .map(|state| {
            (0..6)
                .map(|i| {
                    (0..6)
                        .filter(|&j| a[i] == a[j])
                        .map(|j| state[j])
                        .sum::<usize>()
                        * a[i]
                })
                .max()
                .unwrap() as f64
        })
        .collect::<Vec<_>>();

    let scores1 = states
        .iter()
        .map(|state| {
            let v = (0..6)
                .flat_map(|i| repeat_n(i, state[i]))
                .collect::<Vec<_>>();
            assert_eq!(v.len(), 5);

            let mut t = (0usize..1 << 5)
                .map(|s| {
                    let b = |i: usize| if s & (1 << i) != 0 { v[i] } else { 0 };
                    let e = |i: usize| if s & (1 << i) != 0 { v[i] } else { 5 };

                    let mut sum = 0.0;
                    let mut num = 0.0;
                    for i0 in b(0)..=e(0) {
                        for i1 in b(1)..=e(1) {
                            for i2 in b(2)..=e(2) {
                                for i3 in b(3)..=e(3) {
                                    for i4 in b(4)..=e(4) {
                                        let mut next_state = [0; 6];
                                        next_state[i0] += 1;
                                        next_state[i1] += 1;
                                        next_state[i2] += 1;
                                        next_state[i3] += 1;
                                        next_state[i4] += 1;

                                        sum += scores[idxs[encode(&next_state)]];
                                        num += 1.0;
                                    }
                                }
                            }
                        }
                    }

                    sum / num
                })
                .collect::<Vec<_>>();

            for i in 0..5 {
                for s in 0..1 << 5 {
                    if s & (1 << i) == 0 {
                        t[s] = t[s].max(t[s | (1 << i)]);
                    }
                }
            }

            t
        })
        .collect::<Vec<_>>();

    let scores1 = scores1
        .into_iter()
        .enumerate()
        .map(|(idx, row)| {
            let state = states[idx];
            let v = (0..6)
                .flat_map(|i| repeat_n(i, state[i]))
                .collect::<Vec<_>>();

            row.into_iter()
                .enumerate()
                .map(|(s, t)| {
                    let mut locked = [0; 6];
                    for i in 0..5 {
                        if s & (1 << i) != 0 {
                            locked[v[i]] += 1;
                        }
                    }

                    (encode(&locked), t)
                })
                .fold(vec![0.0; 6usize.pow(6)], |mut tt, (ss, t)| {
                    tt[ss] = t;
                    tt
                })
        })
        .collect::<Vec<_>>();

    let ans = states
        .iter()
        .enumerate()
        .map(|(idx, state)| {
            let v = (0..6)
                .flat_map(|i| repeat_n(i, state[i]))
                .collect::<Vec<_>>();
            assert_eq!(v.len(), 5);

            (0..1 << 5)
                .map(|s| {
                    let b = |i: usize| if s & (1 << i) != 0 { v[i] } else { 0 };
                    let e = |i: usize| if s & (1 << i) != 0 { v[i] } else { 5 };
                    let mut locked = [0; 6];
                    for i in 0..5 {
                        if s & (1 << i) != 0 {
                            locked[v[i]] += 1;
                        }
                    }

                    let mut sum = 0.0;
                    let mut num = 0.0;
                    for i0 in b(0)..=e(0) {
                        for i1 in b(1)..=e(1) {
                            for i2 in b(2)..=e(2) {
                                for i3 in b(3)..=e(3) {
                                    for i4 in b(4)..=e(4) {
                                        let mut next_state = [0; 6];
                                        next_state[i0] += 1;
                                        next_state[i1] += 1;
                                        next_state[i2] += 1;
                                        next_state[i3] += 1;
                                        next_state[i4] += 1;

                                        sum += scores1[idxs[encode(&next_state)]][encode(&locked)];
                                        num += 1.0;
                                    }
                                }
                            }
                        }
                    }

                    sum / num
                })
                .max_by(f64::total_cmp)
                .unwrap()
                * freq[idx] as f64
        })
        .sum::<f64>()
        / 6f64.powi(5);

    println!("{ans}");
}

fn all_states(i: usize, s: usize, mut state: [usize; 6], states: &mut Vec<[usize; 6]>) {
    if i == 6 {
        if s == 5 {
            states.push(state);
        }
        return;
    }

    for j in 0..=5 - s {
        state[i] = j;
        all_states(i + 1, s + j, state, states);
    }
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
