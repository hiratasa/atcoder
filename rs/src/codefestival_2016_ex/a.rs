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
    let n = read::<usize>();
    let ab = read_vec(n, || read_tuple!(i64, i64));

    if ab[0].0 != 0 || ab[1].1 != 0 {
        println!("-1");
        return;
    }

    if ab[0].1 != ab[1].0 {
        println!("-1");
        return;
    }

    let l = ab[0].1;

    if l == 0 {
        println!("-1");
        return;
    }

    if (2..n).any(|i| ab[i].0 == 0 || ab[i].1 == 0) {
        println!("-1");
        return;
    }

    if (0..n).any(|i| ab[i].0 + ab[i].1 < l || (ab[i].0 - ab[i].1).abs() > l) {
        println!("-1");
        return;
    }

    let t = ab.citer().fold(FxHashMap::default(), |mut map, (a, b)| {
        *map.entry((a, b)).or_insert(0) += 1;
        map
    });

    if let Some(ans) = ab
        .citer()
        .sorted_by_key(|&(a, b)| (a + b, a))
        .dedup()
        .group_by(|&(a, b)| a + b)
        .into_iter()
        .map(|(_, mut it)| {
            it.try_fold((vec![0usize], i64::MIN), |(v, prev_a), (a, b)| {
                if a > 0
                    && !t.contains_key(&(a - 1, b))
                    && !t.contains_key(&(a - 1, b - 1))
                    && !t.contains_key(&(a - 1, b + 1))
                {
                    return None;
                }
                if b > 0
                    && !t.contains_key(&(a, b - 1))
                    && !t.contains_key(&(a - 1, b - 1))
                    && !t.contains_key(&(a + 1, b - 1))
                {
                    return None;
                }

                let v = if prev_a + 1 == a {
                    v
                } else {
                    vec![v.citer().min().unwrap()]
                };

                let m = t[&(a, b)];

                let u = if a == 0 {
                    assert!(v.len() == 1);
                    assert!(v[0] == 0);
                    assert!(m == 1);

                    vec![0, 1]
                } else if b == 0 {
                    assert!(v.len() >= 2);
                    assert!(m == 1);

                    let x = min(v[0].saturating_add(1), v[1..].citer().min().unwrap());

                    vec![x]
                } else {
                    if t.contains_key(&(a - 1, b - 1)) {
                        v.citer()
                            .chain(repeat(usize::MAX))
                            .enumerate()
                            .take(m + 1)
                            .scan(usize::MAX, |k, (_i, p)| {
                                *k = min(k.saturating_add(1), p.saturating_add(m));
                                Some(*k)
                            })
                            .collect::<Vec<_>>()
                    } else {
                        let x = v
                            .citer()
                            .enumerate()
                            .map(|(i, p)| p.saturating_add(m.saturating_sub(i) + m))
                            .min()
                            .unwrap();
                        vec![x; m + 1]
                    }
                };

                Some((u, a))
            })
            .map(|(v, _)| v.citer().min().unwrap())
            .filter(|&x| x < usize::MAX)
        })
        .sum::<Option<usize>>()
    {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
