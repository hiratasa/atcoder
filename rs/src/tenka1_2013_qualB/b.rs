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
macro_rules! replaced {
    ($c:expr, $i:expr, $x:expr) => {{
        let mut c = $c;
        c[i] = x;
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
    let (q, l) = read_tuple!(usize, usize);

    let state = (0..q).try_fold((vec![], 0usize), |(stack, len), _| {
        let command = read_row::<String>();

        match command[0].as_str() {
            "Push" => {
                let n: usize = command[1].parse().unwrap();
                let m: i64 = command[2].parse().unwrap();
                if len + n > l {
                    Err("FULL")
                } else {
                    Ok((pushed!(stack, (m, n)), len + n))
                }
            }
            "Pop" => {
                let n: usize = command[1].parse().unwrap();
                let mut stack = stack;

                let (to_remove, num_remove) = once(0)
                    .chain(stack.citer().rev().map(|(_mm, nn)| nn))
                    .cumsum()
                    .take_while(|&c| c <= n)
                    .enumerate()
                    .last()
                    .unwrap();
                stack.resize(stack.len() - to_remove, (0i64, 0usize));
                if n > num_remove {
                    if let Some((_mm, nn)) = stack.last_mut() {
                        *nn -= n - num_remove;
                        Ok((stack, len - n))
                    } else {
                        Err("EMPTY")
                    }
                } else {
                    Ok((stack, len - n))
                }
            }
            "Top" => {
                if let Some(&(mm, _nn)) = stack.last() {
                    println!("{}", mm);
                    Ok((stack, len))
                } else {
                    Err("EMPTY")
                }
            }
            "Size" => {
                println!("{}", len);
                Ok((stack, len))
            }
            _ => unreachable!(),
        }
    });

    match state {
        Ok(_) => println!("SAFE"),
        Err(err) => println!("{}", err),
    }
}
