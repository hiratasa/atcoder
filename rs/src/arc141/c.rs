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

#[allow(dead_code)]
fn check0(p: &[usize], q: &[usize], ans: &[bool]) -> bool {
    let mut t = ans.citer().map(|b| b).enumerate().collect::<Vec<_>>();
    let mut p2 = vec![];
    while !t.is_empty() {
        let k = t
            .citer()
            .map(|(_, b)| if b { 1 } else { -1 })
            .cumsum::<i64>()
            .take_while(|&x| x >= 0)
            .count();
        assert!(k % 2 == 0, "{:?}", t);

        if k > 0 {
            p2.extend(t[..k].citer().map(|(i, _)| i + 1));
            t = t[k..].to_vec();
        } else {
            let i = t.citer().position(|(_, b)| b).unwrap();

            p2.push(t[i].0 + 1);
            p2.push(t[0].0 + 1);

            t.remove(i);
            t.remove(0);
        }
    }

    let mut t = ans.citer().map(|b| b).enumerate().collect::<Vec<_>>();
    let mut q2 = vec![];
    while !t.is_empty() {
        let k = t
            .citer()
            .rev()
            .map(|(_, b)| if b { 1 } else { -1 })
            .cumsum::<i64>()
            .take_while(|&x| x >= 0)
            .count();
        assert!(k % 2 == 0, "{:?}", t);

        if k > 0 {
            q2.extend(t[t.len() - k..].citer().rev().map(|(i, _)| i + 1));
            t = t[..t.len() - k].to_vec();
        } else {
            let i = t.citer().rposition(|(_, b)| b).unwrap();

            q2.push(t[i].0 + 1);
            q2.push(t[t.len() - 1].0 + 1);

            let l = t.len();

            t.remove(l - 1);
            t.remove(i);
        }
    }

    p == &p2[..] && q == &q2[..]
}

#[allow(dead_code)]
fn check1(p: &[usize], q: &[usize]) -> bool {
    let n = p.len() / 2;

    iterate((1usize << n) - 1, |&s| {
        let z = s.trailing_zeros();
        let y = s + (1 << z);
        ((s & !y) >> (1 + z)) | y
    })
    .take_while(|&s| s < 1 << (2 * n))
    .any(|s| {
        let t = (0..2 * n).map(|i| s & (1 << i) > 0).collect::<Vec<_>>();
        check0(p, q, &t)
    })
}

fn solve(n: usize, p: &[usize], q: &[usize]) -> Option<Vec<char>> {
    let (t0, set0) = if let Some((t, set, _)) = p
        .citer()
        .tuples()
        .group_by(|&(i, j)| i < j)
        .into_iter()
        .map(|(inc, it)| (inc, it.collect::<Vec<_>>()))
        .try_fold(
            (vec![], FxHashSet::default(), 1),
            |(t, mut set, st), (inc, v)| {
                if inc {
                    if v[0].0 == st
                        && v.citer()
                            .flat_map(|(i, j)| it![i, j])
                            .eq(st..st + v.len() * 2)
                    {
                        Some((
                            pushed!(t, (st, st + v.len() * 2, true)),
                            set,
                            st + v.len() * 2,
                        ))
                    } else {
                        None
                    }
                } else {
                    if v.citer()
                        .map(|(i, _)| i)
                        .tuple_windows()
                        .all(|(i, j)| i < j)
                        && v.citer()
                            .map(|(_, j)| j)
                            .tuple_windows()
                            .all(|(i, j)| i < j)
                        && v.citer().map(|(_, j)| j).min().unwrap() == st
                        && v.citer().map(|(i, _)| i).max().unwrap() == st + v.len() * 2 - 1
                    {
                        set.extend(v.citer().map(|(i, _)| i));
                        Some((
                            pushed!(t, (st, st + v.len() * 2, false)),
                            set,
                            st + v.len() * 2,
                        ))
                    } else {
                        None
                    }
                }
            },
        ) {
        (t, set)
    } else {
        return None;
    };

    let (t1, set1) = if let Some((t, set, _)) = q
        .citer()
        .map(|i| 2 * n + 1 - i)
        .tuples()
        .group_by(|&(i, j)| i < j)
        .into_iter()
        .map(|(inc, it)| (inc, it.collect::<Vec<_>>()))
        .try_fold(
            (vec![], FxHashSet::default(), 1),
            |(t, mut set, st), (inc, v)| {
                if inc {
                    if v[0].0 == st
                        && v.citer()
                            .flat_map(|(i, j)| it![i, j])
                            .eq(st..st + v.len() * 2)
                    {
                        Some((
                            pushed!(t, (st, st + v.len() * 2, true)),
                            set,
                            st + v.len() * 2,
                        ))
                    } else {
                        None
                    }
                } else {
                    if v.citer()
                        .map(|(i, _)| i)
                        .tuple_windows()
                        .all(|(i, j)| i < j)
                        && v.citer()
                            .map(|(_, j)| j)
                            .tuple_windows()
                            .all(|(i, j)| i < j)
                        && v.citer().map(|(_, j)| j).min().unwrap() == st
                        && v.citer().map(|(i, _)| i).max().unwrap() == st + v.len() * 2 - 1
                    {
                        set.extend(v.citer().map(|(i, _)| 2 * n + 1 - i));
                        Some((
                            pushed!(t, (st, st + v.len() * 2, false)),
                            set,
                            st + v.len() * 2,
                        ))
                    } else {
                        None
                    }
                }
            },
        ) {
        (t, set)
    } else {
        return None;
    };

    if !t0.citer().eq(t1
        .citer()
        .rev()
        .map(|(i, j, inc)| (2 * n + 1 - j + 1, 2 * n + 1 - i + 1, !inc)))
    {
        return None;
    }

    assert!(set0.len() + set1.len() == n);

    let ans = (1..=2 * n)
        .map(|i| {
            if set0.contains(&i) || set1.contains(&i) {
                '('
            } else {
                ')'
            }
        })
        .collect();

    Some(ans)
}

fn main() {
    let n = read::<usize>();
    let p = read_row::<usize>();
    let q = read_row::<usize>();

    let ans = solve(n, &p, &q);

    if let Some(ans) = ans {
        println!("{}", ans.citer().join(""));
    } else {
        println!("-1");
    }
}
