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
    let k = read::<usize>();
    let s = read_str();

    let n = s.len();

    if k > 0 && n >> min(63, k - 1) == 0 {
        println!("impossible");
        return;
    } else if n >> min(63, k) == 1 {
        println!("impossible");
        return;
    }

    let m = n >> k;

    let t = (0usize..(1 << k))
        .flat_map(|i| {
            let l = (i + 1).trailing_zeros() as usize;
            let c = i != (1 << k) - 1 && n & (1 << (k - l - 1)) > 0;

            if i % 2 == 0 {
                (0..m).chain(once(m + l).filter(|_| c)).collect::<Vec<_>>()
            } else {
                (0..m)
                    .rev()
                    .chain(once(m + l).filter(|_| c))
                    .collect::<Vec<_>>()
            }
        })
        .zip(s)
        .fold(vec![[0usize; 26]; m + k], |mut t, (idx, c)| {
            t[idx][c as usize - 'a' as usize] += 1;

            t
        });

    let chars = t
        .iter()
        .map(|table| {
            let sum = table.citer().sum::<usize>();
            let c0 = table.citer().position_max().unwrap();
            let c1 = (0..26)
                .filter(|&c| c != c0)
                .max_by_key(|&c| table[c])
                .unwrap();

            (sum - table[c0], sum - table[c1], c0)
        })
        .collect::<Vec<_>>();

    let x = chars.citer().map(|(x, _, _)| x).sum::<usize>();
    let ans = if (0..m).all(|i| chars[i].2 == chars[m - i - 1].2) {
        let z = chars[..m]
            .citer()
            .enumerate()
            .filter(|&(i, _)| 2 * i + 1 != m)
            .map(|(_, (x, y, _))| y - x)
            .min()
            .unwrap_or(0);

        x + z
    } else {
        x
    };

    println!("{}", ans);
}
