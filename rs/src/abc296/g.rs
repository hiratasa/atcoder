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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
    let n = read::<usize>();
    let xy = read_vec(n, || read_tuple!(i64, i64));
    let q = read::<usize>();
    let ab = read_vec(q, || read_tuple!(i64, i64));

    let i_xmin = xy.citer().position_min().unwrap();
    let i_xmax = xy.citer().position_max().unwrap();

    let n_lower = (i_xmax + n - i_xmin) % n;
    let n_upper = (i_xmin + n - i_xmax) % n;

    let lower = xy
        .citer()
        .cycle()
        .skip(i_xmin)
        .take(n_lower + 1)
        .collect::<Vec<_>>();
    let upper = xy
        .citer()
        .cycle()
        .skip(i_xmax)
        .take(n_upper + 1)
        .collect::<Vec<_>>();

    let xmin_ymin = xy
        .citer()
        .filter(|&(x, _)| x == xy[i_xmin].0)
        .map(|(_, y)| y)
        .min()
        .unwrap();
    let xmin_ymax = xy
        .citer()
        .filter(|&(x, _)| x == xy[i_xmin].0)
        .map(|(_, y)| y)
        .max()
        .unwrap();
    let xmax_ymin = xy
        .citer()
        .filter(|&(x, _)| x == xy[i_xmax].0)
        .map(|(_, y)| y)
        .min()
        .unwrap();
    let xmax_ymax = xy
        .citer()
        .filter(|&(x, _)| x == xy[i_xmax].0)
        .map(|(_, y)| y)
        .max()
        .unwrap();

    enum Result {
        On,
        In,
        Out,
    };

    ab.citer()
        .map(|(a, b)| {
            if a == xy[i_xmin].0 {
                if b < xmin_ymin {
                    return Result::Out;
                } else if b > xmin_ymax {
                    return Result::Out;
                } else {
                    return Result::On;
                }
            } else if a < xy[i_xmin].0 {
                return Result::Out;
            }

            if a == xy[i_xmax].0 {
                if b < xmax_ymin {
                    return Result::Out;
                } else if b > xmax_ymax {
                    return Result::Out;
                } else {
                    return Result::On;
                }
            } else if a > xy[i_xmax].0 {
                return Result::Out;
            }

            let idx = lower
                .binary_search_by_key(&a, |&(x, _)| x)
                .map_or_else(|idx| idx, |idx| idx);

            let (x0, y0) = lower[idx - 1];
            let (x1, y1) = lower[idx];

            let (dx1, dy1) = (x1 - x0, y1 - y0);
            let (dx2, dy2) = (a - x0, b - y0);

            match (dx1 * dy2 - dx2 * dy1).cmp(&0) {
                Ordering::Greater => {}
                Ordering::Equal => return Result::On,
                Ordering::Less => return Result::Out,
            }

            let idx = upper
                .binary_search_by_key(&Reverse(a), |&(x, _)| Reverse(x))
                .map_or_else(|idx| idx, |idx| idx);

            let (x0, y0) = upper[idx - 1];
            let (x1, y1) = upper[idx];

            let (dx1, dy1) = (x1 - x0, y1 - y0);
            let (dx2, dy2) = (a - x0, b - y0);

            match (dx1 * dy2 - dx2 * dy1).cmp(&0) {
                Ordering::Greater => {}
                Ordering::Equal => return Result::On,
                Ordering::Less => return Result::Out,
            }

            Result::In
        })
        .for_each(|res| match res {
            Result::On => println!("ON"),
            Result::Out => println!("OUT"),
            Result::In => println!("IN"),
        })
}
