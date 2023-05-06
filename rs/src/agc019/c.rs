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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
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

fn main() {
    let (x1, y1, x2, y2) = read_tuple!(i64, i64, i64, i64);

    let n: usize = read();
    let xy = read_vec(n, || read_tuple!(i64, i64));

    let (x1, y1, x2, y2) = if x1 <= x2 {
        (x1, y1, x2, y2)
    } else {
        (x2, y2, x1, y1)
    };

    let (x1, y1, x2, y2, xy) = if y1 <= y2 {
        (x1, y1, x2, y2, xy)
    } else {
        (
            x1,
            -y1,
            x2,
            -y2,
            xy.into_iter().map(|(x, y)| (x, -y)).collect(),
        )
    };

    let dp = once((x1, y1, false))
        .chain(
            xy.citer()
                .flat_map(|(x, y)| it!((x, y, true), (x + 1, y, false))),
        )
        .sorted()
        .group_by(|t| t.0)
        .into_iter()
        .map(|(_k, it)| it.max_by_key(|&t| t.2).unwrap())
        .skip_while(|&(x, _, _)| x < x1)
        .take_while(|&(x, _, _)| x <= x2)
        .filter(|&(_, y, _)| y1 <= y && y <= y2)
        .fold(vec![(y1, y1 + 1, false)], |mut dp, (x, y, has_fountain)| {
            if has_fountain {
                if x == x2 && dp.last().unwrap().1 <= y {
                    dp.last_mut().unwrap().2 = true;
                    return dp;
                }
                match dp.binary_search_by_key(&y, |&(begin, _end, _)| begin) {
                    Ok(idx) => {
                        if idx == 0 || dp[idx - 1].2 {
                            dp[idx].2 = true;
                        }
                        if idx + 1 < dp.len() {
                            dp[idx].1 = y + 1;
                            dp[idx + 1].0 = y + 1;
                        } else {
                            dp[idx].1 = y + 1;
                            if y == y2 {
                                dp[idx].2 = true;
                            } else {
                                dp.push((y + 1, y2 + 1, false));
                            }
                        }
                    }
                    Err(idx) => {
                        if idx < dp.len() {
                            if idx > 0 {
                                dp[idx - 1].1 = y;
                            }
                            dp[idx].0 = y;
                        } else if dp[idx - 1].1 <= y {
                            dp[idx - 1].1 = y;
                            dp.push((y, y + 1, false));
                        } else {
                            // dp[idx - 1].1 > y
                            dp[idx - 1].1 = y;
                            dp.push((y, y2 + 1, false));
                        }
                    }
                }
            } else {
                dp.last_mut().unwrap().1 = y2 + 1;
            }

            dp
        });

    let ans = ((x2 - x1) + (y2 - y1)) as f64 * 100.0
        + (dp.len() - 1) as f64 * (-20.0 + 5.0 * std::f64::consts::PI)
        + dp.last().unwrap().2 as u32 as f64 * (-20.0 + 10.0 * std::f64::consts::PI);
    // eprintln!("{:?}", dp);
    println!("{}", ans);
}
