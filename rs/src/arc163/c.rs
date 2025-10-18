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

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
}

const M: usize = 1000000000;

fn solve(
    table: &[Option<Vec<usize>>],
    n: usize,
    nums: &mut Vec<usize>,
    s: usize,
) -> Option<Vec<usize>> {
    if s == n {
        let k = nums.len();

        let a = table[k].as_ref()?;

        let ans = izip!(nums.citer(), a.citer())
            .flat_map(|(x, y)| table[x].as_ref().unwrap().citer().map(move |z| z * y))
            .collect::<Vec<_>>();

        if ans.citer().all(|x| x <= M) && ans.citer().sorted().dedup().count() == n {
            return Some(ans);
        } else {
            return None;
        }
    }

    let last = nums.last().copied().unwrap_or(0);

    (last + 1..n)
        .rev()
        .filter(|&x| s + x + (x + 1) < n)
        .chain(once(n - s).filter(|&x| x > last && x != n))
        .filter(|&x| table[x].is_some())
        .find_map(|x| {
            nums.push(x);
            let ret = solve(table, n, nums, s + x);
            nums.pop();

            ret
        })
}

fn main() {
    let mut table = vec![None, Some(vec![1usize]), None, Some(vec![2, 3, 6])];

    let n0 = table.len();
    for n in n0..=500 {
        let v = table[n - 1].clone().unwrap();

        let x0 = v
            .citer()
            .filter(|&x| x.checked_mul(x + 1).is_some())
            .filter(|&x| v.citer().all(|y| y != x + 1 && y != x * (x + 1)))
            .min()
            .unwrap();

        table.push(Some(
            v.citer()
                .filter(|&x| x != x0)
                .chain(it![x0 + 1, x0 * (x0 + 1)])
                .collect::<Vec<_>>(),
        ));
    }

    let t = read::<usize>();
    for _ in 0..t {
        let n = read::<usize>();

        if let Some(ans) = table[n].as_ref() {
            println!("Yes");
            println!("{}", ans.citer().join(" "));
        } else {
            println!("No");
        }
    }
}
