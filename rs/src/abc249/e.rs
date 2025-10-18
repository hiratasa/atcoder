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
        bs.buffer_mut()[0] = $x as u64;
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
    let (n, p) = read_tuple!(usize, usize);

    let table = it![0, 0]
        .chain(
            (1..=n)
                .map(|i| {
                    (
                        i,
                        1 + iterate(i, |&ii| ii / 10).take_while(|&ii| ii > 0).count(),
                    )
                })
                .dedup_by(|&(_, l0), &(_, l1)| l0 == l1)
                .map(|(i, _)| i),
        )
        .chain(once(n + 1))
        .collect::<Vec<_>>();
    let calc = |l: usize| {
        table
            .binary_search_by(|&ll| ll.cmp(&l).then(Ordering::Less))
            .unwrap_err()
            - 1
    };

    let dp = (1..=n).fold(vec![vec![0; n], vvec![1; 0; n]], |dp, i| {
        let mut t = table
            .citer()
            .tuple_windows()
            .enumerate()
            .filter(|&(_, (st, _))| st > 0 && st <= i)
            .map(|(j, (st, ed))| {
                itertools::repeat_n(0, j)
                    .chain(
                        izip!(
                            dp[i + 1 - st].citer(),
                            dp[(i + 1).saturating_sub(ed)].citer()
                        )
                        .map(|(x, y)| ((x + p - y) % p * 25) % p),
                    )
                    .take(n)
                    .collect::<Vec<_>>()
            })
            // 0からの寄与だけ25倍じゃなくて26倍なのでさらに1回足す
            .chain(once((|| {
                let mut v = vec![0; n];
                let idx = calc(i);
                if idx < n {
                    v[idx] = 1;
                }
                v
            })()))
            .fold(vec![0; n], |mut t, t1| {
                izip!(t.iter_mut(), t1.into_iter()).for_each(|(x, y)| *x = (*x + y) % p);

                t
            });

        // 累積和にする
        izip!(t.iter_mut(), dp[i].citer()).for_each(|(x, y)| *x = (*x + y) % p);

        pushed!(dp, t)
    });

    let ans = izip!(dp[n + 1][..n].citer(), dp[n][..n].citer())
        .map(|(x, y)| (x + p - y) % p)
        .fold(0, |s, x| (s + x) % p);

    println!("{}", ans);
}
