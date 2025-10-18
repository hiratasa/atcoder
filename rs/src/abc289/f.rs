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
    let (sx, sy) = read_tuple!(i64, i64);
    let (tx, ty) = read_tuple!(i64, i64);
    let (a, b, c, d) = read_tuple!(i64, i64, i64, i64);

    if sx % 2 != tx % 2 {
        println!("No");
        return;
    }

    if sy % 2 != ty % 2 {
        println!("No");
        return;
    }

    let ans = if a == b {
        if c == d {
            if (sx, sy) == (tx, ty) {
                vec![]
            } else if (sx + 2 * (a - sx), sy + 2 * (c - sy)) == (tx, ty) {
                vec![(a, c)]
            } else {
                println!("No");
                return;
            }
        } else {
            let mut y = sy;
            let mut ans = vec![];

            if sx == tx {
                // NOP
            } else if sx + 2 * (a - sx) == tx {
                ans.push((a, c));
                y = y + 2 * (c - y);
            } else {
                println!("No");
                return;
            }

            while y < ty {
                ans.push((a, c));
                ans.push((a, c + 1));
                y += 2;
            }
            while y > ty {
                ans.push((a, c + 1));
                ans.push((a, c));
                y -= 2;
            }

            ans
        }
    } else {
        if c == d {
            let mut x = sx;
            let mut ans = vec![];

            if sy == ty {
                // NOP
            } else if sy + 2 * (c - sy) == ty {
                ans.push((a, c));
                x = x + 2 * (a - x);
            } else {
                println!("No");
                return;
            }

            while x < tx {
                ans.push((a, c));
                ans.push((a + 1, c));
                x += 2;
            }
            while x > tx {
                ans.push((a + 1, c));
                ans.push((a, c));
                x -= 2;
            }

            ans
        } else {
            let (mut x, mut y) = (sx, sy);

            let mut ans = vec![];

            while x < tx {
                ans.push((a, c));
                ans.push((a + 1, c));
                x += 2;
            }
            while x > tx {
                ans.push((a + 1, c));
                ans.push((a, c));
                x -= 2;
            }
            while y < ty {
                ans.push((a, c));
                ans.push((a, c + 1));
                y += 2;
            }
            while y > ty {
                ans.push((a, c + 1));
                ans.push((a, c));
                y -= 2;
            }
            ans
        }
    };

    println!("Yes");
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
