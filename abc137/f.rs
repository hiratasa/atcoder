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
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn combi(n: usize, p: usize) -> Vec<usize> {
    // C(n, r) = C(n - 1, r) + C(n - 1, r - 1)
    (0..n+1).fold(Vec::with_capacity(n + 1), |mut c, i| {
        c.push(1);
        for r in (1..i).rev() {
            c[r] = (c.get(r - 1).unwrap() + c.get(r).unwrap()) % p;
        }
        c[0] = 1;
        c
    })
}

fn main() {
    let p = read::<i64>();
    let a = read_vec::<i64>();

    let pu = p as usize;

    let c = combi(pu - 1, pu);

    let pow = (0..p).map(|x| {
        iter::once(1).chain((1..p).scan(1, |prev, _| {
            *prev = (*prev * x) % p;
            Some(*prev)
        })).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // 1 - (x - i)^(p - 1)
    //  = 1 mod p when x = i
    //  = 0 mod p otherwise
    let coeffs = a.into_iter().enumerate().filter(|&(_, aa)| aa > 0).map(|(i, _)| i).fold(vec![0; pu], |mut coeffs, i| {
        let ii = i as i64;
        coeffs[0] = (coeffs.get(0).unwrap() + 1) % p;
        for j in 0usize..pu {
            let jj = j as i64;
            coeffs[j] = (coeffs.get(j).unwrap() + (if (p-jj) % 2 == 0 {1} else {-1}) * *c.get(j).unwrap() as i64 * pow[i][pu - 1 - j] + p) % p;
        }
        coeffs
    });

    println!("{}", coeffs.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
}