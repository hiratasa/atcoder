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

fn main() {
    let n = read::<usize>();
    let lr = read_vec(n, || read_tuple!(i64, i64));

    let candidates = lr
        .citer()
        .flat_map(|(l, r)| it![l, r])
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let c0 = candidates[0];
    let v = lr
        .citer()
        .map(|(l, r)| (max(l, min(c0, r)), l, r))
        .sorted_by_key(|&(x, _, r)| (x, r))
        .collect::<Vec<_>>();

    let (mut on, mut right) = v.citer().fold(
        (BinaryHeap::new(), BinaryHeap::new()),
        |(mut on, mut right), (x, l, r)| {
            if x < c0 {
                // NOP
            } else if x == c0 {
                on.push(Reverse(r));
            } else {
                right.push(Reverse((x, l, r)));
            }

            (on, right)
        },
    );

    let mut ans = v
        .citer()
        .enumerate()
        .map(|(i, (x, _, _))| {
            let i = i as i64;
            let n = n as i64;

            i * x + (n - 1 - i) * (-x)
        })
        .sum::<i64>();

    let mut left = n - on.len() - right.len();
    let mut prev = candidates[0];
    let mut d = ans;
    for &c in &candidates[1..] {
        while matches!(on.peek(), Some(&Reverse(r)) if r < c) {
            let r = on.pop().unwrap().0;
            assert!(r == prev);
            left += 1;
        }

        d += (c - prev) * on.len() as i64 * left as i64
            - (c - prev) * on.len() as i64 * right.len() as i64;

        while matches!(right.peek(), Some(&Reverse((_, l, _))) if l <= c) {
            let (x, l, r) = right.pop().unwrap().0;
            assert!(x == l);
            assert!(x == c);

            on.push(Reverse(r));
        }

        prev = c;
        ans = min(ans, d);
    }

    println!("{}", ans);
}
