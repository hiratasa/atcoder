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

// x, yが全て1<<k以下という条件下で問題を解く
fn solve(k: usize, xy: &[(i64, i64)]) -> (Vec<i64>, Vec<Vec<char>>) {
    assert!(xy.citer().all(|(x, y)| (x + y) % 2 != 0));
    assert!(xy.citer().all(|(x, _y)| -(1 << k) <= x && x <= 1 << k));
    assert!(xy.citer().all(|(_x, y)| -(1 << k) <= y && y <= 1 << k));

    if k == 0 {
        (
            vec![1],
            xy.citer()
                .map(|(x, y)| match (x, y) {
                    (0, 1) => vec!['U'],
                    (0, -1) => vec!['D'],
                    (1, 0) => vec!['R'],
                    (-1, 0) => vec!['L'],
                    _ => unreachable!(),
                })
                .collect(),
        )
    } else {
        let (arms, dirs) = solve(
            k - 1,
            &xy.citer()
                .map(|(x, y)| {
                    if x % 2 == 0 {
                        assert!(y % 2 != 0);

                        if (x / 2 + (y - 1) / 2) % 2 != 0 {
                            (x / 2, (y - 1) / 2)
                        } else {
                            (x / 2, (y + 1) / 2)
                        }
                    } else {
                        if ((x - 1) / 2 + y / 2) % 2 != 0 {
                            ((x - 1) / 2, y / 2)
                        } else {
                            ((x + 1) / 2, y / 2)
                        }
                    }
                })
                .collect::<Vec<_>>(),
        );

        // 全てのアームを2倍にし、最後に長さ1のアームを足す
        let arms = arms
            .citer()
            .map(|d| d * 2)
            .chain(once(1))
            .collect::<Vec<_>>();
        let dirs = izip!(xy.citer(), dirs.into_iter())
            .map(|((x, y), s)| {
                if x % 2 == 0 {
                    if (x / 2 + (y - 1) / 2) % 2 != 0 {
                        // (x/2, (y-1)/2) が前段のsolve()で実現できてる
                        pushed!(s, 'U')
                    } else {
                        // (x/2, (y+1)/2) が前段のsolve()で実現できてる
                        pushed!(s, 'D')
                    }
                } else {
                    if ((x - 1) / 2 + y / 2) % 2 != 0 {
                        // ((x-1)/2, y/2) が前段のsolve()で実現できてる
                        pushed!(s, 'R')
                    } else {
                        // ((x+1)/2, y/2) が前段のsolve()で実現できてる
                        pushed!(s, 'L')
                    }
                }
            })
            .collect::<Vec<_>>();

        (arms, dirs)
    }
}

fn main() {
    let n = read::<usize>();
    let xy = read_vec(n, || read_tuple!(i64, i64));

    if !xy.citer().map(|(x, y)| (x + y).abs() % 2).all_equal() {
        println!("-1");
        return;
    }

    let parity = (xy[0].0 + xy[0].1).rem_euclid(2);

    let xy2 = xy
        .citer()
        .map(|(x, y)| (x, y + 1 - parity))
        .collect::<Vec<_>>();

    let (arms, dirs) = solve(30, &xy2);

    let arms = if parity == 0 { pushed!(arms, 1) } else { arms };
    let dirs = dirs
        .into_iter()
        .map(|s| if parity == 0 { pushed!(s, 'D') } else { s })
        .collect::<Vec<_>>();

    // verify
    // for (s, (x, y)) in izip!(dirs.iter(), xy.citer()) {
    //     let (xx, yy) = s
    //         .citer()
    //         .enumerate()
    //         .map(|(i, c)| {
    //             let d = arms[i];

    //             match c {
    //                 'L' => (-d, 0),
    //                 'R' => (d, 0),
    //                 'D' => (0, -d),
    //                 'U' => (0, d),
    //                 _ => unreachable!(),
    //             }
    //         })
    //         .fold((0, 0), |(xx, yy), (dx, dy)| (xx + dx, yy + dy));

    //     assert_eq!((xx, yy), (x, y), "{:?} {}", arms, s.citer().join(""));
    // }

    println!("{}", arms.len());
    println!("{}", arms.citer().join(" "));
    for s in dirs {
        println!("{}", s.citer().join(""));
    }
}
