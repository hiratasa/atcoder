#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::iter;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn multiply(m1: &Vec<Vec<usize>>, m2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = m1.len();

    let mut m = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                m[i][k] ^= m1[i][j] & m2[j][k];
            }
        }
    }

    m
}

fn pow(m: &Vec<Vec<usize>>, i: usize) -> Vec<Vec<usize>> {
    if i == 0 {
        let n = m.len();
        (0..n)
            .map(|j| {
                let mut r = vec![0; n];
                r[j] = 0usize.wrapping_sub(1);
                r
            })
            .collect()
    } else if i == 1 {
        m.clone()
    } else {
        let t = pow(m, i / 2);
        let t = multiply(&t, &t);

        if i % 2 == 0 { t } else { multiply(&t, m) }
    }
}

fn main() {
    let (k, m) = read_cols!(usize, usize);

    let a = read_vec::<usize>();
    let c = read_vec::<usize>();

    let coeff = (0..k).fold(vec![], |mut coeff: Vec<Vec<usize>>, i| {
        let r0 = iter::repeat(0)
            .take(i)
            .chain(c.iter().rev().copied().take(k - i))
            .collect::<Vec<_>>();

        let r = coeff
            .iter()
            .zip(c.iter().take(i).rev().copied())
            .fold(r0, |mut r, (rr, cc)| {
                r.iter_mut().zip(rr.iter()).for_each(|(x, y)| {
                    *x = *x ^ (*y & cc);
                });
                r
            });

        coeff.push(r);

        coeff
    });

    let m = m - 1;

    let p = pow(&coeff, m / k);

    let d = p
        .iter()
        .map(|r| {
            r.iter()
                .zip(a.iter())
                .map(|(rr, aa)| rr & aa)
                .fold(0, |acc, x| acc ^ x)
        })
        .collect::<Vec<_>>();

    println!("{}", d[m % k]);
}
