#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
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

const M: usize = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Mod(usize);

impl Mod {
    fn new(n: usize) -> Self {
        Mod(n % M)
    }

    fn pow(self, p: usize) -> Self {
        if p == 0 {
            Mod(1)
        } else if p == 1 {
            self
        } else {
            let r = self.pow(p / 2);

            if p % 2 == 0 {
                r * r
            } else {
                r * r * self
            }
        }
    }
}

impl std::ops::Mul for Mod {
    type Output = Self;
    fn mul(self, rhs: Mod) -> Self {
        Mod(self.0 * rhs.0 % M)
    }
}

impl std::ops::MulAssign<Mod> for Mod {
    fn mul_assign(&mut self, rhs: Mod) {
        self.0 *= rhs.0;
        self.0 %= M;
    }
}

impl std::ops::Div for Mod {
    type Output = Self;
    fn div(self, rhs: Mod) -> Self {
        if self.0 == 0 {
            self
        } else {
            assert!(rhs.0 != 0);
            self * rhs.pow(M - 2)
        }
    }
}

impl std::ops::DivAssign<Mod> for Mod {
    fn div_assign(&mut self, rhs: Mod) {
        *self = *self / rhs;
    }
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);

    let a = read_row::<usize>();

    let s = a.iter().sum::<usize>() % M;
    let t = s + n;

    if m < s {
        println!("0");
        return;
    }

    let ans = (0..t)
        .map(|i| Mod::new(n + m - i) / Mod::new(t - i))
        .fold(Mod(1), |b, c| b * c);
    println!("{}", ans.0);
}
