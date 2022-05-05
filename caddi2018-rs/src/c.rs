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
    let a = read_row::<usize>();

    let comp = |x: usize, p: usize, y: usize, q: usize| {
        if p < q {
            if q - p > 100 {
                Ordering::Less
            } else {
                x.cmp(&(y.saturating_mul(2usize.saturating_pow((q - p) as u32))))
            }
        } else {
            if p - q > 100 {
                Ordering::Greater
            } else {
                x.saturating_mul(2usize.saturating_pow((p - q) as u32))
                    .cmp(&y)
            }
        }
    };

    let dec = once(0)
        .chain(a.citer().scan((vec![], 0), |(t, prev), x| {
            t.push((x, 0, x, 0, 1));
            let mut m = 0;
            while t.len() >= 2
                && comp(
                    t[t.len() - 2].0,
                    t[t.len() - 2].1,
                    t[t.len() - 1].2,
                    t[t.len() - 1].3,
                ) == Ordering::Less
            {
                let (y0, p0, z0, q0, k0) = t.pop().unwrap();
                let (y1, p1, z1, q1, k1) = t.pop().unwrap();

                let idx0 = (z0 + 1).next_power_of_two().trailing_zeros() as usize + q0;
                let idx1 = (y1 + 1).next_power_of_two().trailing_zeros() as usize + p1;

                let mut d = idx0.saturating_sub(idx1);
                if comp(y1, p1 + d, z0, q0) == Ordering::Less {
                    d += 1;
                }
                d = (d + 1) / 2 * 2;
                assert!(comp(y1, p1 + d, z0, q0) != Ordering::Less);

                t.push((y0, p0, z1, q1 + d, k0 + k1));
                m += d * k1;
            }

            *prev += m;

            Some(*prev)
        }))
        .collect::<Vec<_>>();

    let inc = once(0)
        .chain(a.citer().rev().scan((vec![], 0), |(t, prev), x| {
            t.push((x, 0, x, 0, 1));
            let mut m = 0;
            while t.len() >= 2
                && comp(
                    t[t.len() - 2].0,
                    t[t.len() - 2].1,
                    t[t.len() - 1].2,
                    t[t.len() - 1].3,
                ) == Ordering::Less
            {
                let (y0, p0, z0, q0, k0) = t.pop().unwrap();
                let (y1, p1, z1, q1, k1) = t.pop().unwrap();

                let idx0 = (z0 + 1).next_power_of_two().trailing_zeros() as usize + q0;
                let idx1 = (y1 + 1).next_power_of_two().trailing_zeros() as usize + p1;

                let mut d = idx0.saturating_sub(idx1);
                if comp(y1, p1 + d, z0, q0) == Ordering::Less {
                    d += 1;
                }
                d = (d + 1) / 2 * 2;
                assert!(comp(y1, p1 + d, z0, q0) != Ordering::Less);

                t.push((y0, p0, z1, q1 + d, k0 + k1));
                m += d * k1;
            }

            *prev += m;

            Some(*prev)
        }))
        .collect::<Vec<_>>();

    let ans = (0..=n).map(|i| i + dec[i] + inc[n - i]).min().unwrap();

    println!("{}", ans);
}
