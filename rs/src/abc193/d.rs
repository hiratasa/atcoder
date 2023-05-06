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
    let k: usize = read();
    let s = read_str()
        .into_iter()
        .take(4)
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let t = read_str()
        .into_iter()
        .take(4)
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let ss = s.citer().fold(vec![0; 10], |mut a, x| {
        a[x] += 1;
        a
    });
    let tt = t.citer().fold(vec![0; 10], |mut a, x| {
        a[x] += 1;
        a
    });

    let nums = (0..10).map(|i| k - ss[i] - tt[i]).collect::<Vec<_>>();

    let calc_score = |hands: &[usize], add: usize| {
        (1..=9)
            .map(|i| {
                if i == add {
                    i * 10usize.pow(hands[i] as u32 + 1)
                } else {
                    i * 10usize.pow(hands[i] as u32)
                }
            })
            .sum::<usize>()
    };

    let m = iproduct!(1..=9, 1..=9)
        .filter(|&(a, b)| {
            if a == b {
                nums[a] >= 2 && calc_score(&ss, a) > calc_score(&tt, a)
            } else {
                nums[a] >= 1 && nums[b] >= 1 && calc_score(&ss, a) > calc_score(&tt, b)
            }
        })
        .map(|(a, b)| {
            if a == b {
                nums[a] * (nums[a] - 1)
            } else {
                nums[a] * nums[b]
            }
        })
        .sum::<usize>();

    let ans = m as f64 / ((9 * k - 8) * (9 * k - 9)) as f64;
    println!("{}", ans);
}
