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
    let (mut a, mut b) = read_tuple!(usize, usize);

    const W: usize = 100;
    let mut ans: Vec<Vec<bool>> = vec![];
    while a > 0 || b > 0 {
        if a > 0 && b > 0 {
            if ans.is_empty() || !ans.last().unwrap()[0] {
                ans.push(vec![true; W]);
                ans.push(vvec![true; false; W]);
                ans.push(vec![true; W]);
                a -= 1;
                b -= 1;
                let l = ans.len() - 2;
                for i in (2..W).step_by(2) {
                    if a == 0 {
                        break;
                    }
                    ans[l][i] = true;
                    a -= 1;
                }
            } else {
                ans.push(vec![false; W]);
                ans.push(vvec![false; true; W]);
                ans.push(vec![false; W]);
                a -= 1;
                b -= 1;
                let l = ans.len() - 2;
                for i in (2..W).step_by(2) {
                    if b == 0 {
                        break;
                    }
                    ans[l][i] = false;
                    b -= 1;
                }
            }
        } else if a > 0 {
            assert!(b == 0);
            if !ans.last().unwrap()[W - 1] {
                ans.last_mut().unwrap()[W - 1] = true;
            }
            ans.push(vec![true; W]);
            ans.push(vvec![true; false; W]);
            ans.push(vec![true; W]);
            a -= 1;
            let l = ans.len() - 2;
            for i in (2..W).step_by(2) {
                if a == 0 {
                    break;
                }
                ans[l][i] = true;
                a -= 1;
            }
        } else if b > 0 {
            assert!(a == 0);
            if ans.last().unwrap()[W - 1] {
                ans.last_mut().unwrap()[W - 1] = false;
            }
            ans.push(vec![false; W]);
            ans.push(vvec![false; true; W]);
            ans.push(vec![false; W]);
            b -= 1;
            let l = ans.len() - 2;
            for i in (2..W).step_by(2) {
                if b == 0 {
                    break;
                }
                ans[l][i] = false;
                b -= 1;
            }
        }
    }

    println!("{} {}", ans.len(), ans[0].len());
    for row in ans {
        println!(
            "{}",
            row.citer().map(|x| if x { '#' } else { '.' }).join("")
        );
    }
}
