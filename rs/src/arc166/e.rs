#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let x = $x;
        let mut c = $c;
        c.push(x);
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
}

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

// calc sum[x=0 to n-1] floor((a+x*b)/c)
#[allow(dead_code)]
fn floor_sum(n: usize, mut a: usize, mut b: usize, c: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut ret = 0;

    // if c < 0 {
    //     return floor_sum(n, -a, -b, -c);
    // }

    ret += a.div_euclid(c) * n;
    a = a.rem_euclid(c);
    // assert!(a >= 0);

    ret += b.div_euclid(c) * n * (n - 1) / 2;
    b = b.rem_euclid(c);
    // assert!(b >= 0);

    if b == 0 {
        return ret;
    }

    let last = a + n * b;
    ret += floor_sum(last / c, last % c, c, b);
    ret
}

// 分数a/bの*上側*最良近似分数の列を一次式の形で求める
// 返り値: ((c, d), (e, f), k) -> (c+e*i)/(d+f*i) (0<=i<=k)
fn get_upper_best_rational_approximations(
    a: usize,
    b: usize,
) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut rationals = vec![];

    let mut lower = (0, 1);
    let mut upper = (1, 1);

    // rationals.push((lower, upper, 0));
    rationals.push((upper, lower, 0));

    let bb = b / gcd(a, b);

    let mut denom = 1;
    while denom < bb {
        if (lower.0 + upper.0) * b < (lower.1 + upper.1) * a {
            let u = (lower.1 * a - lower.0 * b) / (upper.0 * b - upper.1 * a);

            // rationals.push((lower, upper, u));

            lower = (lower.0 + upper.0 * u, lower.1 + upper.1 * u);

            denom = lower.1;
        } else {
            let u = (upper.0 * b - upper.1 * a) / (lower.1 * a - lower.0 * b);

            rationals.push((upper, lower, u));

            upper = (lower.0 * u + upper.0, lower.1 * u + upper.1);

            denom = upper.1;
        }
    }

    rationals
}

fn main() {
    input! {
        cases: [(usize, usize, usize)]
    }

    cases
        .citer()
        .map(|(n, a, b)| {
            let g = gcd(a, b);
            let aa = a / g;
            let bb = b / g;

            let k = lower_bound_int(1, 1 << 42, |k: usize| {
                let na = k - 1;
                let nb = (bb - 1 + k * aa - 1) / bb;
                let m = na - nb;
                m.cmp(&n).then(Ordering::Less)
            }) - 1;

            let len = a * k - 2;

            let r = (bb - (bb - 1 + k * aa) % bb) % bb * g;

            // bで割った余りがr以上になるaの倍数で最小のものを求める
            // let x = lower_bound_int(0, bb, |x: usize| {
            //     // x*a%bがr以上
            //     // ⇔ floor((x*a+(b-c))/b) - floor(x*a/b) > 0

            //     let ok = floor_sum(x + 1, b - r, a, b) - floor_sum(x + 1, 0, a, b) > 0;

            //     if ok {
            //         Ordering::Greater
            //     } else {
            //         Ordering::Less
            //     }
            // });

            let best_rational_approximations = get_upper_best_rational_approximations(a, b);
            let x = best_rational_approximations
                .citer()
                .find_map(|((c, d), (e, f), u)| {
                    // y/x=(c+i*e)/(d+i*f) where 0<=i<=u

                    // b - r >= (b-余り) = yb - xa = (cb-da) - i*(fa-eb)
                    let det0 = c * b - d * a;
                    let det1 = f * a - e * b;
                    if det1 == 0 {
                        if b - r >= det0 { Some((c, e)) } else { None }
                    } else if b - r + u * det1 >= det0 {
                        let idx = (det0.saturating_sub(b - r) + det1 - 1) / det1;

                        Some((c + idx * e, d + idx * f))
                    } else {
                        None
                    }
                })
                .unwrap()
                .1;

            (x * a + 1, x * a + 1 + len)
        })
        .for_each(|ans| {
            println!("{} {}", ans.0, ans.1);
        });
}
