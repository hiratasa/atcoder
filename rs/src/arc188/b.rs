fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: i64, k: i64,
        };

        let ans = solve(n, k) || (n % 2 == 0 && solve(n, n / 2 - k));

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve(n: i64, k: i64) -> bool {
    if k == 0 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if 2 * k == n {
        return false;
    }

    let (x, y, g) = extgcd(2 * k, n);
    let a = if k % g == 0 {
        let a = (-(k / g) * x).rem_euclid(n / g);
        2 * a + 1
    } else {
        i64::MAX
    };
    let b = if n % 2 == 0 && (k + n / 2) % g == 0 {
        let a = (-((k + n / 2) / g) * x).rem_euclid(n / g);
        2 * a + 1
    } else {
        i64::MAX
    };

    let c = min(a, b);
    if c == i64::MAX {
        false
    } else if n % 2 == 0 {
        2 * c == n
    } else {
        c == n
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (_zero, g, _u, v) = std::iter::successors(Some((a, b, 1, 0)), |&(a, b, u, v)| {
        if a == 0 {
            None
        } else {
            Some((b % a, a, -u * (b / a) + v, u))
        }
    })
    .last()
    .unwrap();

    (v, (g - a * v) / b, g)
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
