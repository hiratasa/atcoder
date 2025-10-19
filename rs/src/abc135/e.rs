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
fn read_digits() -> Vec<usize> {
    read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
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

fn solve(x: i64, y: i64, k: i64) -> Option<Vec<(i64, i64)>> {
    if (x, y) == (0, 0) {
        return Some(vec![]);
    }

    if (x + y) % 2 != 0 && k % 2 == 0 {
        return None;
    }

    let (invx, x) = if x < 0 { (true, -x) } else { (false, x) };
    let (invy, y) = if y < 0 { (true, -y) } else { (false, y) };

    let d = x + y;

    let mut ans = if d < k {
        if d % 2 > 0 {
            once((k, 0))
                .chain(
                    solve(x - k, y, k)
                        .unwrap()
                        .citer()
                        .map(|(xx, yy)| (xx + k, yy)),
                )
                .collect::<Vec<_>>()
        } else {
            if x < y {
                vec![((x + y) / 2 - k, (x + y) / 2), (x, y)]
            } else {
                vec![((x + y) / 2, (x + y) / 2 - k), (x, y)]
            }
        }
    } else if d % k == 0 {
        successors(Some((0, 0)), |&(xx, yy)| {
            if xx < x {
                if xx + k < x {
                    Some((xx + k, yy))
                } else {
                    Some((x, yy + k - (x - xx)))
                }
            } else {
                if yy < y { Some((xx, yy + k)) } else { None }
            }
        })
        .skip(1)
        .collect::<Vec<_>>()
    } else {
        let c = (d + k - 1) / k * k;

        if c % 2 == d % 2 {
            let r = (c - d) / 2;

            if r + y >= k {
                successors(Some((0, 0)), |&(xx, yy)| {
                    if yy == 0 && xx < x + r {
                        if xx + k < x + r {
                            Some((xx + k, 0))
                        } else {
                            Some((x + r, k - (x + r - xx)))
                        }
                    } else if yy < y {
                        if yy + k < y {
                            Some((x + r, yy + k))
                        } else {
                            Some((x + r - (k - (y - yy)), y))
                        }
                    } else {
                        assert!(xx == x);
                        assert!(yy == y);
                        None
                    }
                })
                .skip(1)
                .collect::<Vec<_>>()
            } else {
                solve(y, x, k)
                    .map(|v| v.citer().map(|(xx, yy)| (yy, xx)).collect::<Vec<_>>())
                    .unwrap()
            }
        } else {
            let r = d % k;

            if r <= y {
                let s0 = solve(0, r, k).unwrap();
                let s1 = solve(x, y - r, k).unwrap();

                chain(s0, s1.citer().map(|(xx, yy)| (xx, yy + r))).collect::<Vec<_>>()
            } else {
                let s0 = solve(x % k, y, k).unwrap();
                let s1 = solve(x - x % k, 0, k).unwrap();

                chain(s0, s1.citer().map(|(xx, yy)| (xx + x % k, yy + y))).collect::<Vec<_>>()
            }
        }
    };

    for (xx, yy) in ans.iter_mut() {
        if invx {
            *xx *= -1;
        }
        if invy {
            *yy *= -1;
        }
    }

    Some(ans)
}

fn main() {
    let k = read::<i64>();
    let (x, y) = read_tuple!(i64, i64);

    if let Some(ans) = solve(x, y, k) {
        assert!(
            ans.citer()
                .tuple_windows()
                .all(|((x0, y0), (x1, y1))| { (x0 - x1).abs() + (y0 - y1).abs() == k })
        );

        println!("{}", ans.len());
        for (x, y) in ans {
            println!("{} {}", x, y);
        }
    } else {
        println!("-1");
    }
}
