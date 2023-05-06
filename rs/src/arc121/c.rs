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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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

#[allow(dead_code)]
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
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

fn solve3(p: &[usize], start_odd: bool) -> Vec<usize> {
    let l = iterate((p.to_vec(), start_odd), |(q, odd)| {
        let mut q = q.clone();
        if *odd {
            q.swap(0, 1);
        } else {
            q.swap(1, 2);
        }

        (q, !odd)
    })
    .take_while(|(q, _)| q != &[1, 2, 3])
    .count();

    it![1 + !start_odd as usize, 1 + start_odd as usize]
        .cycle()
        .take(l)
        .collect()
}

fn main() {
    let t = read::<usize>();

    repeat_with(|| {
        let n = read::<usize>();
        let p = read_row::<usize>();

        p
    })
    .take(t)
    .map(|p| {
        (0..)
            .try_fold((p, vec![]), |(mut p, mut ops), _| {
                if p.len() == 2 {
                    assert!(ops.is_empty());
                    if p[0] == 1 {
                        Err(vec![])
                    } else {
                        Err(vec![1])
                    }
                } else if p.len() == 3 {
                    ops.extend(solve3(&p, ops.len() % 2 == 0));
                    Err(ops)
                } else {
                    let pos = p.citer().position_max().unwrap();

                    if pos == p.len() - 1 {
                        p.pop();
                        Ok((p, ops))
                    } else if pos % 2 == ops.len() % 2 {
                        ops.extend((pos + 1)..p.len());
                        p.remove(pos);
                        Ok((p, ops))
                    } else if ops.len() % 2 == 0 {
                        assert!(pos % 2 > 0);
                        if pos > 1 {
                            ops.push(1);
                            p.swap(0, 1);
                        } else {
                            ops.push(3);
                            p.swap(2, 3);
                        }
                        Ok((p, ops))
                    } else {
                        assert!(pos % 2 == 0);
                        if pos != 2 {
                            ops.push(2);
                            p.swap(1, 2);
                        } else {
                            ops.push(2);
                            p.swap(1, 2);
                            ops.push(3);
                            p.swap(2, 3);
                        }

                        Ok((p, ops))
                    }
                }
            })
            .unwrap_err()
    })
    .for_each(|ans| {
        println!("{}", ans.len());
        println!("{}", ans.citer().join(" "));
    });
}
