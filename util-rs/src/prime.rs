use itertools::iterate;
use std::cmp::{max, min};
use std::mem::replace;

fn pow_mod(mut x: u128, mut p: u128, m: u128) -> u128 {
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
}

// Millerテスト
fn is_prime(n: usize) -> bool {
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

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

// 非自明な約数を返す
// rho法
// https://ja.wikipedia.org/wiki/%E3%83%9D%E3%83%A9%E3%83%BC%E3%83%89%E3%83%BB%E3%83%AD%E3%83%BC%E7%B4%A0%E5%9B%A0%E6%95%B0%E5%88%86%E8%A7%A3%E6%B3%95
// https://qiita.com/Kiri8128/items/eca965fe86ea5f4cbb98
// 大体 O(N^(1/4))
#[allow(dead_code)]
fn get_factor(n: usize) -> Option<usize> {
    if n == 1 || is_prime(n) {
        return None;
    }

    for &p in &[2, 3, 5, 7, 11, 13, 17, 19] {
        if n % p == 0 {
            return Some(p);
        }
    }

    const M: usize = 20;

    for c in 1.. {
        // ここをwrapping_mulなどにしてしまうと（たぶん分布が偏って）遅いので注意
        let f = |x: usize| ((x as u128 * x as u128 + c as u128) % n as u128) as usize;

        let mut x = 2;
        let mut y = 2;
        let mut x0 = x;
        let mut y0 = y;
        let mut d = 1;

        while d == 1 {
            x0 = x;
            y0 = y;

            let q = (0..M).fold(1, |q, _| {
                x = f(x);
                y = f(f(y));

                (q as u128 * (max(x, y) - min(x, y)) as u128 % n as u128) as usize
            });

            d = gcd(q, n);
        }

        if d as usize == n {
            x = x0;
            y = y0;
            d = 1;
            while d == 1 {
                x = f(x);
                y = f(f(y));

                d = gcd(max(x, y) - min(x, y), n);
            }
        }

        if d == n {
            continue;
        }

        return Some(d);
    }

    unreachable!()
}

// ソートされていないので注意
#[allow(dead_code)]
fn get_prime_factors(n: usize) -> Vec<usize> {
    if n == 1 {
        vec![]
    } else if let Some(x) = get_factor(n) {
        let factors0 = get_prime_factors(x);
        let mut factors1 = get_prime_factors(n / x);

        factors1.extend(factors0);
        factors1
    } else {
        vec![n]
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_prime_factors() {
        for x in 1..10000 {
            // 試し割法
            let prime_factors0 = (2..)
                .scan(x, |xx, p| {
                    if p * p > *xx {
                        if *xx > 1 {
                            let y = replace(xx, 1);
                            Some(vec![y])
                        } else {
                            None
                        }
                    } else {
                        let (num, next_x) = iterate(*xx, |&y| y / p)
                            .enumerate()
                            .skip_while(|&(_, y)| y % p == 0)
                            .next()
                            .unwrap();
                        *xx = next_x;
                        Some(vec![p; num])
                    }
                })
                .flatten()
                .collect::<Vec<_>>();

            // ロー法
            let mut prime_factors1 = get_prime_factors(x);
            prime_factors1.sort();

            assert_eq!(prime_factors0, prime_factors1, "for x={}", x);
        }
    }
}
