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
    let (n, k) = read_tuple!(usize, usize);
    let p = read_row::<usize>();

    let q = p.citer().enumerate().fold(vec![0; n + 1], |mut q, (i, x)| {
        q[x] = i;
        q
    });

    let x0 = (1..=n).find(|&i| q[i] <= k || n - q[i] <= k).unwrap();

    let pos0 = q[x0];

    // x0の前を全部削除する場合
    let ans0 = if pos0 <= k {
        let (mut a, _s) = p[pos0 + 1..]
            .citer()
            // dummy
            .chain(once(0))
            .fold((vec![x0], k - pos0), |(mut v, mut s), x| {
                while s > 0 && v[v.len() - 1] > x {
                    v.pop();
                    s -= 1;
                }
                v.push(x);
                (v, s)
            });

        // remove dummy
        a.pop();

        Some(a)
    } else {
        None
    };

    // x0を後ろから先頭に持ってくる場合
    let ans1 = if n - pos0 <= k {
        let (mut a, _, _) = p[pos0 + 1..]
            .citer()
            .map(|x| (x, true))
            .chain(p[0..pos0].citer().map(|x| (x, false)))
            // dummy
            .chain(once((0, false)))
            .fold(
                (vec![x0], vec![true], k - (n - pos0)),
                |(mut v, mut tails, mut s), (x, tail)| {
                    while (tails[tails.len() - 1] || s > 0) && v[v.len() - 1] > x {
                        if !tails[tails.len() - 1] {
                            s -= 1;
                        }
                        v.pop();
                        tails.pop();
                    }
                    v.push(x);
                    tails.push(tail);
                    (v, tails, s)
                },
            );

        // remove dummy
        a.pop();

        Some(a)
    } else {
        None
    };

    let ans = max(
        ans0.map(|ans0| Reverse(ans0)),
        ans1.map(|ans1| Reverse(ans1)),
    )
    .unwrap()
    .0;

    println!("{}", ans.citer().join(" "));
}
