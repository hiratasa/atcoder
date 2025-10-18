fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    let ans = if n % 2 == 0 {
        a.sort();
        a[n / 2..].iter().sum::<i64>() - a[..n / 2].iter().sum::<i64>()
    } else {
        let from_left = (0..n)
            .step_by(2)
            .scan((0, BinaryHeap::new(), BinaryHeap::new()), |(s, q, q2), i| {
                if i > 0 {
                    q2.push(Reverse(a[i - 2]));
                    q2.push(Reverse(a[i - 1]));
                    *s += a[i - 2];
                    *s += a[i - 1];

                    while matches!((q.peek(), q2.peek()), (Some(&x), Some(&Reverse(y))) if x > y) {
                        let x = q2.pop().unwrap().0;
                        q.push(x);
                        *s -= 2 * x;
                    }

                    while q.len() < q2.len() {
                        let x = q2.pop().unwrap().0;
                        q.push(x);
                        *s -= 2 * x;
                    }

                    while q.len() > q2.len() {
                        let x = q.pop().unwrap();
                        q2.push(Reverse(x));
                        *s += 2 * x;
                    }
                }

                Some(*s)
            })
            .collect::<Vec<_>>();

        let from_right = (0..n)
            .rev()
            .step_by(2)
            .scan((0, BinaryHeap::new(), BinaryHeap::new()), |(s, q, q2), i| {
                if i < n - 1 {
                    q2.push(Reverse(a[i + 2]));
                    q2.push(Reverse(a[i + 1]));
                    *s += a[i + 2];
                    *s += a[i + 1];

                    while matches!((q.peek(), q2.peek()), (Some(&x), Some(&Reverse(y))) if x > y) {
                        let x = q2.pop().unwrap().0;
                        q.push(x);
                        *s -= 2 * x;
                    }

                    while q.len() < q2.len() {
                        let x = q2.pop().unwrap().0;
                        q.push(x);
                        *s -= 2 * x;
                    }

                    while q.len() > q2.len() {
                        let x = q.pop().unwrap();
                        q2.push(Reverse(x));
                        *s += 2 * x;
                    }
                }

                Some(*s)
            })
            .collect::<Vec<_>>();

        izip!(from_left, from_right.into_iter().rev())
            .map(|(x, y)| x + y)
            .max()
            .unwrap()
    };

    println!("{ans}");
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
