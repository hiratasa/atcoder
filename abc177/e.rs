#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
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

fn main() {
    let n = read::<usize>();
    let a = read_vec::<usize>();

    let mut is_prime = vec![true; 1000001];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=1000000 {
        if is_prime[i] {
            primes.push(i);

            for x in (2..).map(|j| i * j).take_while(|&x| x <= 1000000) {
                is_prime[x] = false;
            }
        }
    }

    let mut m = HashMap::<usize, usize>::new();
    for &aa in a.iter() {
        let mut aa = aa;
        for p in primes.iter() {
            if p * p > aa {
                break;
            }
            if aa % p == 0 {
                *m.entry(*p).or_insert(0) += 1;

                while aa % p == 0 {
                    aa /= p;
                }
            }
        }

        if is_prime[aa] {
            *m.entry(aa).or_insert(0) += 1;
        }
    }

    if m.values().all(|&v| v <= 1) {
        println!("pairwise coprime");
    } else if m.values().all(|&v| v < n) {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}