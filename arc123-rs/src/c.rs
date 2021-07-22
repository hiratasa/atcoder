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
    let t: usize = read();

    const M: usize = 80;
    const C: usize = 30;

    for _ in 0..t {
        let n: usize = read();

        let digits = iterate(n, |&m| m / 10)
            .take_while(|&m| m > 0)
            .map(|m| m % 10)
            .collect::<Vec<_>>();

        let dp = digits.citer().fold(
            vvec![(0..=M).collect::<Vec<_>>(); vec![usize::MAX; M + 1]; C + 1],
            |prev, d| {
                let mut dp = vec![vec![usize::MAX; M + 1]; C + 1];

                // 前の桁からのcarry
                for c in 0..=C {
                    // 今の桁で使う個数
                    for i in 0..=M {
                        // 前の桁でi個以上でできてなかったら無視
                        if prev[c][i] == usize::MAX {
                            break;
                        }

                        let i0 = (i..).find(|&j| (c + j) % 10 == d).unwrap();

                        // 今の桁でできる数
                        for j in (0..).map(|k| i0 + 10 * k).take_while(|&j| j <= 3 * i) {
                            dp[(c + j) / 10][i] = min(dp[(c + j) / 10][i], prev[c][i]);
                        }
                    }
                }

                // eprintln!(
                //     "{} {:?}",
                //     d,
                //     dp[..10]
                //         .iter()
                //         .map(|row| row[..10].citer().collect::<Vec<_>>())
                //         .collect::<Vec<_>>()
                // );
                for c in 0..=C {
                    for i in (0..M).rev() {
                        dp[c][i] = min(dp[c][i], dp[c][i + 1]);
                    }
                }

                dp
            },
        );

        let ans = dp[0][0];
        println!("{}", ans);
    }
}
