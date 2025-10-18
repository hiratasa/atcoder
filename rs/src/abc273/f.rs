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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
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

fn main() {
    let (n, x) = read_tuple!(usize, i64);
    let y = read_row::<i64>();
    let z = read_row::<i64>();

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        Start(i64),
        Goal(i64),
        Wall(i64, usize),
        Hammer(i64, usize),
    };
    impl Event {
        fn pos(self) -> i64 {
            match self {
                Start(x) => x,
                Goal(x) => x,
                Wall(x, _) => x,
                Hammer(x, _) => x,
            }
        }
    }
    use Event::*;

    let events = once(Start(0))
        .chain(once(Goal(x)))
        .chain(y.citer().enumerate().map(|(idx, pos)| Wall(pos, idx)))
        .chain(z.citer().enumerate().map(|(idx, pos)| Hammer(pos, idx)))
        .sorted_by_key(|e| e.pos())
        .collect::<Vec<_>>();
    let m = events.len();

    let idx0 = events.citer().position(|e| e == Start(0)).unwrap();
    let idx1 = events.citer().position(|e| e == Goal(x)).unwrap();
    let hammer_idxs = events
        .citer()
        .enumerate()
        .fold(vec![0; n], |mut idxs, (idx, e)| {
            match e {
                Hammer(_, i_hammer) => {
                    idxs[i_hammer] = idx;
                }
                _ => {}
            }

            idxs
        });

    let mut init = vec![vec![[std::i64::MAX, std::i64::MAX]; m]; m];
    init[idx0][idx0] = [0, 0];

    let dp = (0..m)
        .flat_map(|w| (0..m - w).map(move |l| (l, l + w)))
        .fold(init, |mut dp, (l, r)| {
            (0..2).for_each(|k| {
                let cost = dp[l][r][k];

                let pos = if k == 0 {
                    events[l].pos()
                } else {
                    events[r].pos()
                };

                it![
                    l.checked_sub(1).map(|ll| (ll, r, 0, ll)),
                    Some(r + 1).filter(|&rr| rr < m).map(|rr| (l, rr, 1, rr))
                ]
                .flatten()
                .for_each(|(next_l, next_r, next_k, next_idx)| {
                    let e = events[next_idx];

                    match e {
                        Wall(_, i_hammer) => {
                            if hammer_idxs[i_hammer] < l || hammer_idxs[i_hammer] > r {
                                return;
                            }
                        }
                        _ => {}
                    }

                    let next_pos = e.pos();

                    let next_cost = cost.saturating_add((next_pos - pos).abs());

                    dp[next_l][next_r][next_k] = min(dp[next_l][next_r][next_k], next_cost);
                });
            });

            dp
        });

    let ans = (0..m)
        .map(|idx| min(dp[idx1][idx][0], dp[idx][idx1][1]))
        .min()
        .unwrap();

    if ans == std::i64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
