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
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let x = read_row::<usize>();
    let d = read_row::<i64>();

    const M: usize = 1000001;
    let q = x
        .citer()
        .enumerate()
        .fold(vec![(false, vec![]); M], |mut q, (i, xx)| {
            q[xx].1.push((i, false));
            q
        });

    let (q, zero, _, _, mut ans) = d.citer().enumerate().fold(
        (q, 0i64, 0, M - 1, vec![Err(std::i64::MAX); n]),
        |(mut q, zero, left, right, mut ans), (i, dd)| {
            let new_zero = if zero <= left as i64 {
                zero + dd
            } else {
                assert!(right as i64 <= zero);
                zero - dd
            };

            if 0 <= new_zero && new_zero < M as i64 {
                for (idx, _) in take(&mut q[new_zero as usize].1) {
                    ans[idx] = Ok(i + 1);
                }
            }

            fn merge(q: &mut Vec<(bool, Vec<(usize, bool)>)>, idx0: usize, idx1: usize) {
                if q[idx0].1.len() <= q[idx1].1.len() {
                    let s0 = q[idx0].0;
                    let s1 = q[idx1].0;
                    let v = take(&mut q[idx0].1);
                    for (x, sgn) in v {
                        q[idx1].1.push((x, !(sgn ^ s0 ^ s1)));
                    }
                } else {
                    q[idx0].0 = !q[idx0].0;
                    let s0 = q[idx0].0;
                    let s1 = q[idx1].0;
                    let v = take(&mut q[idx1].1);
                    for (x, sgn) in v {
                        q[idx0].1.push((x, sgn ^ s0 ^ s1));
                    }
                    q.swap(idx0, idx1);
                };
            };

            let area = if new_zero <= left as i64 {
                0
            } else if new_zero <= ((left + right) / 2) as i64 {
                1
            } else if new_zero <= right as i64 {
                2
            } else {
                3
            };

            let (left, right) = match area {
                0 => (left, right),
                1 => {
                    for i in 1..=dd {
                        if new_zero - (i as i64) < left as i64 {
                            break;
                        }

                        merge(&mut q, (new_zero - i) as usize, (new_zero + i) as usize);
                    }

                    (new_zero as usize, right)
                }
                2 => {
                    for i in 1..=dd {
                        if new_zero + i as i64 > right as i64 {
                            break;
                        }

                        merge(&mut q, (new_zero + i) as usize, (new_zero - i) as usize);
                    }

                    (left, new_zero as usize)
                }
                3 => (left, right),
                _ => unreachable!(),
            };

            (q, new_zero, left, right, ans)
        },
    );

    for i in 0..M {
        for &(idx, s) in &q[i].1 {
            let positive = (i as i64 >= zero) ^ q[i].0 ^ s;

            ans[idx] = Err(if positive {
                (i as i64 - zero).abs()
            } else {
                -(i as i64 - zero).abs()
            });
        }
    }

    for x in ans {
        match x {
            Ok(x) => println!("Yes {}", x),
            Err(x) => println!("No {}", x),
        }
    }
}
