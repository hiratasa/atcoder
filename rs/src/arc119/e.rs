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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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
    let n = read::<usize>();
    let a = read_row::<i64>();

    // 反転しない場合
    let ans0 = a
        .citer()
        .tuple_windows()
        .map(|(a0, a1)| (a0 - a1).abs())
        .sum::<i64>();

    // 端っこから反転する場合（両方）
    let ans1 = (1..n)
        .map(|i| -(a[i - 1] - a[i]).abs() + (a[0] - a[i]).abs())
        .min()
        .map_or(ans0, |x| ans0 + x);
    let ans2 = (1..n)
        .map(|i| -(a[i - 1] - a[i]).abs() + (a[i - 1] - a[n - 1]).abs())
        .min()
        .map_or(ans0, |x| ans0 + x);

    // 途中を反転する場合
    let ans3 = a
        .citer()
        .tuple_windows()
        .filter(|&(a0, a1)| a0 >= a1)
        .enumerate()
        .flat_map(|(i, (a0, a1))| it![(a1, None, i), (a0, Some(a1), i)])
        .sorted()
        .scan(
            (BinaryHeap::<Reverse<(i64, usize)>>::new(), vec![false; n]),
            |(q, inserted), (x, pair, idx)| {
                if let Some(pair) = pair {
                    inserted[idx] = false;

                    while matches!(q.peek(), Some(&Reverse((_, idx2))) if !inserted[idx2]) {
                        q.pop();
                    }

                    if let Some(&Reverse((y, _))) = q.peek() {
                        Some(Some(2 * (x - max(y, pair))))
                    } else {
                        Some(None)
                    }
                } else {
                    q.push(Reverse((x, idx)));
                    inserted[idx] = true;
                    Some(None)
                }
            },
        )
        .flatten()
        .max()
        .map_or(ans0, |x| ans0 - x);

    let ans4 = a
        .citer()
        .tuple_windows()
        .filter(|&(a0, a1)| a0 <= a1)
        .enumerate()
        .flat_map(|(i, (a0, a1))| it![(a0, None, i), (a1, Some(a0), i)])
        .sorted()
        .scan(
            (BinaryHeap::<Reverse<(i64, usize)>>::new(), vec![false; n]),
            |(q, inserted), (x, pair, idx)| {
                if let Some(pair) = pair {
                    inserted[idx] = false;

                    while matches!(q.peek(), Some(&Reverse((_, idx2))) if !inserted[idx2]) {
                        q.pop();
                    }

                    if let Some(&Reverse((y, _))) = q.peek() {
                        Some(Some(2 * (x - max(y, pair))))
                    } else {
                        Some(None)
                    }
                } else {
                    q.push(Reverse((x, idx)));
                    inserted[idx] = true;
                    Some(None)
                }
            },
        )
        .flatten()
        .max()
        .map_or(ans0, |x| ans0 - x);

    let ans = min(ans0, min(min(ans1, ans2), min(ans3, ans4)));
    println!("{}", ans);
}
