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

const N: usize = 10;

fn check(board: &[u16]) -> bool {
    board
        .citer()
        .map(|s| (s, 0))
        .fold1(|(s1, s2), (col, _)| {
            // s1: そこまでルートがただ1箇所存在
            // s2: そこまでのルートが2箇所以上存在
            let x0 = (s1 << 1) & col;
            let x1 = s1 & col;
            let x2 = (s1 >> 1) & col;
            let y0 = (s2 << 1) & col;
            let y1 = s2 & col;
            let y2 = (s2 >> 1) & col;

            let next_s2 = y0 | y1 | y2 | (x0 & x1) | (x1 & x2) | (x2 & x0);
            let next_s1 = (x0 | x1 | x2) & !next_s2;

            (next_s1, next_s2)
        })
        .filter(|&(s1, s2)| s2 == 0 && s1.count_ones() == 1)
        .is_some()
}

fn solve(
    idx: usize,
    next: usize,
    counts: &mut [Vec<usize>],
    board: &mut [u16],
) -> Option<Vec<Vec<bool>>> {
    if idx == 4 {
        if check(board) {
            Some(vec![vec![false; N]; N])
        } else {
            None
        }
    } else {
        (next..N * N).find_map(|current| {
            let (i, j) = (current / N, current % N);

            let mut ok = true;
            iproduct!(1..=2, -1i32..=1, -1i32..=1)
                .map(|(s, di, dj)| (s * di, s * dj))
                .filter_map(|(di, dj)| {
                    let ii = Some(i as i32 + di).filter(|&ii| 0 <= ii && ii < N as i32)? as usize;
                    let jj = Some(j as i32 + dj).filter(|&jj| 0 <= jj && jj < N as i32)? as usize;
                    Some((ii, jj))
                })
                .for_each(|(ii, jj)| {
                    counts[ii][jj] += 1;
                    if counts[ii][jj] == 1 {
                        assert!(board[jj] & (1 << ii) > 0);
                        board[jj] ^= 1 << ii;
                        if board[jj] == 0 {
                            ok = false;
                        }
                    }
                });

            let ret = if ok {
                solve(idx + 1, current + 1, counts, board)
            } else {
                None
            };

            iproduct!(1..=2, -1i32..=1, -1i32..=1)
                .map(|(s, di, dj)| (s * di, s * dj))
                .filter_map(|(di, dj)| {
                    let ii = Some(i as i32 + di).filter(|&ii| 0 <= ii && ii < N as i32)? as usize;
                    let jj = Some(j as i32 + dj).filter(|&jj| 0 <= jj && jj < N as i32)? as usize;
                    Some((ii, jj))
                })
                .for_each(|(ii, jj)| {
                    counts[ii][jj] -= 1;
                    if counts[ii][jj] == 0 {
                        assert!(board[jj] & (1 << ii) == 0);
                        board[jj] ^= 1 << ii;
                    }
                });

            ret.map(|mut ans| {
                ans[i][j] = true;
                ans
            })
        })
    }
}

fn main() {
    let ans = solve(0, 0, &mut vec![vec![0; N]; N], &mut vec![(1 << N) - 1; N]).unwrap();

    for row in ans {
        println!(
            "{}",
            row.citer().map(|x| { if x { 'C' } else { '.' } }).join("")
        );
    }
}
