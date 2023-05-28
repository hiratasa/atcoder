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

#[allow(dead_code)]
fn solve0(n: usize, a: i64, b: usize, c: i64, d: usize, x: &[i64]) -> i64 {
    assert_eq!(n, x.len());

    let r = if n == 0 {
        0
    } else {
        [
            x[0] - solve0(n - 1, a, b, c, d, &x[1..]),
            x[n - 1] - solve0(n - 1, a, b, c, d, &x[..n - 1]),
        ]
        .citer()
        .chain((0..=min(b, n - 1)).map(|l| {
            x[0..l].citer().sum::<i64>() + x[l + n.saturating_sub(b)..].citer().sum::<i64>()
                - a
                - solve0(n - min(n, b), a, b, c, d, &x[l..l + n.saturating_sub(b)])
        }))
        .chain((0..=min(d, n - 1)).map(|l| {
            x[0..l].citer().sum::<i64>() + x[l + n.saturating_sub(d)..].citer().sum::<i64>()
                - c
                - solve0(n - min(n, d), a, b, c, d, &x[l..l + n.saturating_sub(d)])
        }))
        .max()
        .unwrap()
    };

    // eprintln!("{:?} = {}", x, r);
    r
}

#[allow(dead_code)]
fn slide_min<'a, T: PartialOrd + Copy>(a: &'a [T], len: usize) -> impl 'a + Iterator<Item = T> {
    assert!(len <= a.len());
    let q = a[..len - 1]
        .iter()
        .enumerate()
        .fold(VecDeque::new(), |mut q, (i, &x)| {
            while matches ! (q . back () , Some (& (_ , y ) ) if y >= x ) {
                q.pop_back();
            }
            q.push_back((i, x));
            q
        });
    a[len - 1..].iter().enumerate().scan(q, move |q, (i, &x)| {
        while matches ! (q . back () , Some (& (_ , y ) ) if y >= x ) {
            q.pop_back();
        }
        q.push_back((i + len - 1, x));
        let r = q.front().unwrap().1;
        if q.front().unwrap().0 == i {
            q.pop_front();
        }
        Some(r)
    })
}

fn main() {
    let (n, a, b, c, d) = read_tuple!(usize, i64, usize, i64, usize);
    let x = read_row::<i64>();

    let dp = (1..=n).fold(
        vvec![vec![0; n + 1]; vec![-(1<<50); n + 1]; n + 1],
        |mut dp, i| {
            let s0 = x[0..i].citer().sum::<i64>();

            // 操作1
            let mut s = s0;
            for l in 0..=n - i {
                dp[i][l] = max(dp[i][l], s + s - dp[i - 1][l + 1]);
                dp[i][l] = max(dp[i][l], s + s - dp[i - 1][l]);
                if l + i < n {
                    s += x[l + i] - x[l];
                }
            }

            // 操作2
            let mut s = s0;
            let ii = i.saturating_sub(b);
            let prev = take(&mut dp[ii]);
            for (l, z) in slide_min(&prev[..n + 1 - ii], i - ii + 1).enumerate() {
                dp[i][l] = max(dp[i][l], s - a + s - z);
                if l + i < n {
                    s += x[l + i] - x[l];
                }
            }
            dp[ii] = prev;

            // 操作3
            let mut s = s0;
            let ii = i.saturating_sub(d);
            let prev = take(&mut dp[ii]);
            for (l, z) in slide_min(&prev[..n + 1 - ii], i - ii + 1).enumerate() {
                dp[i][l] = max(dp[i][l], s - c + s - z);
                if l + i < n {
                    s += x[l + i] - x[l];
                }
            }
            dp[ii] = prev;

            dp
        },
    );

    let s = x.citer().sum::<i64>();
    let ans = dp[n][0] - s;

    println!("{}", ans);
}
