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
    let (n, d) = read_tuple!(usize, i64);

    let (x, y) = read_tuple!(i64, i64);

    if x.abs() % d > 0 || y.abs() % d > 0 {
        println!("0");
        return;
    }

    let x = (x.abs() / d) as usize;
    let y = (y.abs() / d) as usize;

    let combi = iterate(vec![1.0], |prevcombi| {
        once(1.0)
            .chain(prevcombi.citer().tuple_windows().map(|(a, b)| a + b))
            .chain(once(1.0))
            .collect_vec()
    })
    .take(n + 1)
    .collect_vec();
    let invpow2 = iterate(1.0, |&p| p / 2.0).take(2 * n + 1).collect_vec();

    let ans = (0..=n)
        .filter_map(|np| np.checked_sub(x).map(|nn| (np, nn)))
        .filter(|(np, nn)| n >= y + np + nn)
        .filter(|(np, nn)| (n - y - np - nn) % 2 == 0)
        .map(|(np, nn)| {
            // np + nn + ynp + ynn = n
            // ynp - ynn = y
            (np, nn, (n + y - np - nn) / 2, (n - y - np - nn) / 2)
        })
        // C(n, np + nn) * C(np + nn, np) * C(ynp + ynn, ynp)
        .map(|(np, nn, ynp, ynn)| {
            combi[n][np + nn]
                * invpow2[n]
                * combi[np + nn][np]
                * invpow2[np + nn]
                * combi[ynp + ynn][ynp]
                * invpow2[ynp + ynn]
        })
        .sum::<f64>();
    println!("{}", ans);
}
