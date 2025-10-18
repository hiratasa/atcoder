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

use std::num::NonZeroUsize;

fn main() {
    let mut s = read_str();
    let k = read::<usize>();

    let n = s.len();
    // 先頭にダミーの文字を入れておく
    s.insert(0, 'a');
    let (nexts, _) = s.citer().enumerate().rev().fold(
        // メモリ厳しいのでNonZeroUsize使う
        (vec![vec![None; 26]; n + 1], vec![None; 26]),
        |(mut nexts, mut last), (i, c)| {
            nexts[i] = last.clone();
            // i=0のときにNoneになるけどi=0のところは誰も参照しないのでok
            last[c as usize - 'a' as usize] = NonZeroUsize::new(i);
            (nexts, last)
        },
    );

    let dp = nexts
        .iter()
        .enumerate()
        .rev()
        .fold(vec![0; n + 1], |mut dp, (i, next)| {
            // i文字目を1文字目として選んだときに作れる文字列の数
            dp[i] = next
                .citer()
                .flatten()
                .map_into::<usize>()
                .map(|j| dp[j])
                .chain(once(/* 1文字からなる文字列 */ 1))
                .fold(0usize, |x, y| x.saturating_add(y));

            dp
        });

    // 先頭はダミー文字であることに注意
    // 先頭を1文字目として選んだ時に作れる文字列 = 残りの文字列で作れる文字列 + 1
    if k + 1 > dp[0] {
        println!("Eel");
        return;
    }

    // +1についてはうえのコメント参照
    let ans = successors(Some((0, k + 1)), |&(i, kk)| {
        // i+1文字目から辞書順にkk番目を選ぶ
        if kk == 1 {
            // ここまでで打ち切り
            None
        } else {
            let kk = kk - 1;

            let (j, x0, _x1) = nexts[i]
                .citer()
                .flatten()
                .map_into::<usize>()
                .map(|j| (j, dp[j]))
                .scan(0usize, |sum, (j, x)| {
                    let tmp = *sum;
                    *sum = sum.saturating_add(x);
                    Some((j, tmp, *sum))
                })
                .find(|&(_, _, x)| x >= kk)
                .unwrap();

            Some((j, kk - x0))
        }
    })
    .skip(1)
    .map(|(i, _)| s[i])
    .collect::<String>();

    println!("{}", ans);
}
