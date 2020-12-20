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

#[derive(Clone, Copy)]
enum Bracket {
    Left,
    Right,
}

const M: usize = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Mod(usize);
#[allow(dead_code)]
impl Mod {
    fn new(n: usize) -> Self {
        Mod(n % M)
    }
    fn zero() -> Self {
        Mod::new(0)
    }
    fn one() -> Self {
        Mod::new(1)
    }
    fn pow(self, p: usize) -> Self {
        if p == 0 {
            Mod::new(1)
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
impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for Mod {
    type Err = <usize as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}
impl std::ops::Add for Mod {
    type Output = Self;
    fn add(self, rhs: Mod) -> Self {
        Mod::new(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign for Mod {
    fn add_assign(&mut self, rhs: Mod) {
        *self = *self + rhs;
    }
}
impl std::ops::Sub for Mod {
    type Output = Self;
    fn sub(self, rhs: Mod) -> Self {
        Mod::new(self.0 + M - rhs.0)
    }
}
impl std::ops::SubAssign for Mod {
    fn sub_assign(&mut self, rhs: Mod) {
        *self = *self - rhs;
    }
}
impl std::ops::Mul for Mod {
    type Output = Self;
    fn mul(self, rhs: Mod) -> Self {
        Mod::new(self.0 * rhs.0)
    }
}
impl std::ops::MulAssign for Mod {
    fn mul_assign(&mut self, rhs: Mod) {
        *self = *self * rhs;
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
impl std::ops::DivAssign for Mod {
    fn div_assign(&mut self, rhs: Mod) {
        *self = *self / rhs;
    }
}
impl std::iter::Product for Mod {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}
impl std::iter::Sum for Mod {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}

fn main() {
    let s = read::<String>()
        .chars()
        .map(|c| match c {
            '(' => Bracket::Left,
            ')' => Bracket::Right,
            _ => unreachable!(),
        })
        .collect_vec();
    let k: usize = read();

    let slide_one = |m: Vec<Mod>| {
        if m.len() == 1 {
            vec![Mod::zero(), m[0]]
        } else {
            izip!(
                // +1
                once(m[1]).chain(m.iter().copied()),
                // -1
                m.iter().copied().skip(1).chain(repeat(Mod::zero())),
            )
            .map(|(a, b)| a + b)
            .collect_vec()
        }
    };

    let merge = |m1: Vec<Mod>, m2: Vec<Mod>| {
        iproduct!(
            m1.iter().copied().enumerate(),
            m2.iter().copied().enumerate()
        )
        .fold(
            vec![Mod::zero(); m1.len() + m2.len() - 1],
            |mut ret, ((i, a), (j, b))| {
                ret[i + j] += a * b;

                if i > 0 && j > 0 {
                    if i >= j {
                        ret[i - j] += a * b;
                    }
                    if j >= i {
                        ret[j - i] += a * b;
                    }
                }

                ret
            },
        )
    };

    let m = s
        .iter()
        .copied()
        .fold(vec![vec![Mod::one()]], |mut stack, b| {
            match b {
                Bracket::Left => {
                    stack.push(vec![Mod::one()]);
                }
                Bracket::Right => {
                    let mut m = slide_one(slide_one(stack.pop().unwrap()));
                    m.truncate(k + 1);
                    let m2 = merge(stack.pop().unwrap(), m);
                    stack.push(m2);
                }
            };
            stack
        });
    assert!(m.len() == 1);
    let m = &m[0];
    let ans = m[0] + m.iter().copied().skip(1).sum::<Mod>() * Mod::new(2);
    println!("{}", ans);
}
