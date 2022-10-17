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
    let t = read::<usize>();

    repeat_with(|| read_str())
        .take(t)
        .map(|s| {
            let n = s.len();

            let ablock = s
                .citer()
                .enumerate()
                .group_by(|&(_, c)| c)
                .into_iter()
                .map(|(c, mut it)| {
                    let i = it.next().unwrap().0;
                    (c, i, it.count() + 1)
                })
                .filter(|&(c, _, _)| c == 'a')
                .map(|(_, i0, l)| (i0, l))
                .collect::<Vec<_>>();
            let na = ablock.citer().map(|(_, l)| l).sum::<usize>();
            let nb = n - na;
            let lastisa = *s.last().unwrap() == 'a';

            if na == n {
                repeat_n('a', na).collect::<String>()
            } else if ablock.len() == 1 && ablock[0].0 == 0 {
                repeat_n('a', ablock[0].1 % 2)
                    .chain(repeat_n('b', nb))
                    .collect::<String>()
            } else if !lastisa && na % 2 == 0 {
                repeat_n('b', nb).collect()
            } else if !lastisa && ablock.last().map(|(i, l)| i + l == n - 1).unwrap() {
                // aが奇数個で、...ab で終わる場合
                repeat_n('b', nb - 1)
                    .chain(once('a'))
                    .chain(once('b'))
                    .collect()
            } else if !lastisa && ablock.last().map(|(i, l)| i + l + 2 == n).unwrap() {
                // aが奇数個で、...abb で終わる場合
                repeat_n('b', nb - 2)
                    .chain(once('a'))
                    .chain(repeat_n('b', 2))
                    .collect()
            } else {
                let na1 = ablock
                    .citer()
                    .filter(|&(i, l)| l == 1 && i + l != n)
                    .count();
                let sa2 = ablock
                    .citer()
                    .filter(|&(i, l)| l >= 2 && i + l != n)
                    .map(|(_, l)| l - 2)
                    .sum::<usize>();

                if lastisa {
                    let lasta = ablock.last().unwrap().1;
                    repeat_n('b', nb)
                        .chain(repeat_n('a', sa2 + lasta - (na1 % 2) as usize))
                        .collect()
                } else {
                    let hasa2 = ablock.citer().any(|(i, l)| i != 0 && l >= 2);

                    if hasa2 {
                        repeat_n('b', nb - 2)
                            .chain(repeat_n('a', sa2 + 2 - (na1 % 2) as usize))
                            .collect()
                    } else {
                        repeat_n('b', nb - 2)
                            .chain(repeat_n('a', sa2 + 1 - ((na1 + 1) % 2) as usize))
                            .collect()
                    }
                }
            }
        })
        .for_each(|ans| println!("{}", ans))
}
