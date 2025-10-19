fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
        };

        let p = calc(n);
        let m = (p - 1) / n;
        let a = primitive_root(p);
        let b = pow_mod(a, m, p);

        // for i in 1..n {
        //     assert_ne!(pow_mod(b, i, p), 1);
        // }
        // assert_eq!(pow_mod(b, n, p), 1);

        println!("{b} {p}");
    }
}

fn calc(n: u64) -> u64 {
    (1..)
        .map(|x| n.checked_mul(x).unwrap().checked_add(1).unwrap())
        .find(|&x| is_prime(x))
        .unwrap()
}

fn pow_mod<T: Into<u128>>(x: T, y: T, m: T) -> u128 {
    let mut x = x.into();
    let mut y = y.into();
    let m = m.into();
    let mut z = 1u128;

    x = x % m;
    while y > 0 {
        if y & 1 > 0 {
            z = z * x % m;
        }

        x = x * x % m;
        y >>= 1;
    }

    z
}

// Millerテスト
fn is_prime(n: u64) -> bool {
    let n = n as u128;

    if n == 1 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }

    let primes = [2, 3, 5, 7, 11];

    if primes.iter().any(|&p| p == n) {
        return true;
    }

    let m = n - 1;

    // m = 2^s * d
    let (s, d) = iterate(m, |&mm| mm / 2)
        .enumerate()
        .skip_while(|&(_, mm)| mm % 2 == 0)
        .next()
        .unwrap();

    // https://primes.utm.edu/prove/prove2_3.html
    let k = if n < 1373653 {
        2
    } else if n < 25326001 {
        3
    } else if n < 118670087467 {
        if n == 3215031751 {
            return false;
        }
        4
    } else if n < 2152302898747 {
        5
    } else {
        unreachable!()
    };

    primes.iter().take(k).all(|&a| {
        if a == n {
            return true;
        }

        let x = pow_mod(a, d, n);

        if x == 1 {
            true
        } else {
            (0..s)
                .scan(x, |xx, _| Some(replace(xx, *xx * *xx % n)))
                .any(|b| b == n - 1)
        }
    })
}

fn primitive_root(m: u64) -> u64 {
    match m {
        2 => return 1,
        167772161 => return 3,
        469762049 => return 3,
        754974721 => return 11,
        998244353 => return 3,
        1224736769 => return 3,
        1811939329 => return 13,
        2013265921 => return 31,
        _ => {}
    };

    // m - 1の素因数分解
    let primes = (2..)
        .try_fold((vec![], m - 1), |(mut primes, x), i| {
            if i * i > x {
                if x > 1 {
                    primes.push(x);
                }
                Err(primes)
            } else if x % i > 0 {
                Ok((primes, x))
            } else {
                primes.push(i);
                let x = itertools::iterate(x, |x| x / i)
                    .find(|&x| x % i > 0)
                    .unwrap();
                Ok((primes, x))
            }
        })
        .unwrap_err();

    (2..)
        .find(|&g| primes.iter().all(|&p| pow_mod(g, (m - 1) / p, m) != 1))
        .unwrap()
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
