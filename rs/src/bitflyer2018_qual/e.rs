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

fn main() {
    let (y, w) = read_tuple!(i64, i64);
    let (n, m, d) = read_tuple!(usize, usize, i64);
    let a = read_col::<i64>(n);
    let bc = read_vec(m, || read_tuple!(i64, i64));

    let calc = |x: i64| {
        let x = max(x, 0);
        if x <= d { x } else { 0 }
    };

    let add = |set: &mut BTreeSet<(i64, bool)>, x: i64, ty: bool| {
        let prev = *set.range(..(x, ty)).next_back().unwrap();
        let next = *set.range((x, ty)..).next().unwrap();

        let old = calc(next.0 - prev.0 - 1);
        let new = calc(x - prev.0 - 1) + calc(next.0 - x - 1);

        set.insert((x, ty));

        let extra = if set.contains(&(x, !ty)) { 0 } else { 1 };

        new - old + extra
    };

    let remove = |set: &mut BTreeSet<(i64, bool)>, x: i64, ty: bool| {
        set.take(&(x, ty)).unwrap();

        let prev = *set.range(..(x, ty)).next_back().unwrap();
        let next = *set.range((x, ty)..).next().unwrap();

        let old = calc(x - prev.0 - 1) + calc(next.0 - x - 1);
        let new = calc(next.0 - prev.0 - 1);

        let extra = if set.contains(&(x, !ty)) { 0 } else { -1 };

        new - old + extra
    };

    let mut set = BTreeSet::new();
    set.insert((-10 * d, false));
    set.insert(((y + 1) * w + 10 * d, false));

    let mut current = 0;
    for &x in &a {
        current += add(&mut set, x, false);
    }
    for &(b, c) in &bc {
        current += add(&mut set, c + w * (b - 1), true);
    }

    println!("{}", current);

    let bc_by_dow = bc
        .citer()
        .fold(vec![vec![]; w as usize + 1], |mut bc_by_dow, (b, c)| {
            bc_by_dow[c as usize].push(b);
            bc_by_dow
        });

    for i in 1..w {
        for &x in &a {
            current += remove(&mut set, x + i - 1, false);
        }
        for &x in &a {
            current += add(&mut set, x + i, false);
        }

        let c = i;
        for &b in &bc_by_dow[i as usize] {
            current += remove(&mut set, c + w * (b - 1), true);
        }
        for &b in &bc_by_dow[i as usize] {
            current += add(&mut set, c + w * b, true);
        }

        println!("{}", current);
    }
}
