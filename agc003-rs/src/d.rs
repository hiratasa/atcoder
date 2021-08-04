#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

fn pow_mod(mut x: u128, mut p: u128, m: u128) -> u128 {
    let mut y = 1;

    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y
}

fn is_prime(n: usize) -> bool {
    let n = n as u128;

    // Millerテスト
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

fn main() {
    let n: usize = read();
    let s = read_col::<usize>(n);

    let t = s
        .citer()
        .map(|ss| {
            let factors = get_prime_factors(ss);

            // ss = x * y^2 * z^3
            let (x, y) = factors
                .citer()
                .sorted()
                .group_by(|&p| p)
                .into_iter()
                .map(|(p, it)| (p, it.count() % 3))
                .fold((1, 1), |(x, y), (p, m)| match m {
                    0 => (x, y),
                    1 => (x * p, y),
                    2 => (x, y * p),
                    _ => unreachable!(),
                });

            (x, y)
        })
        .sorted()
        .group_by(|&v| v)
        .into_iter()
        .map(|(v, it)| (v, it.count()))
        .collect::<FxHashMap<_, _>>();
    // eprintln!("{:?}", t);

    let inv = |(x, y): (usize, usize)| (y, x);

    let ans = t
        .iter()
        .map(|(&v, &d)| {
            if v == (1, 1) {
                1
            } else {
                let iv = inv(v);
                if let Some(d2) = t.get(&iv) {
                    match d.cmp(&d2) {
                        Ordering::Less => 0,
                        Ordering::Equal => {
                            if v < iv {
                                d
                            } else {
                                0
                            }
                        }
                        Ordering::Greater => d,
                    }
                } else {
                    d
                }
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}
