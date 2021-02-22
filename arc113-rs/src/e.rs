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
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
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

trait ToString {
    fn to_string(self: Self) -> String;
}
impl<I, T> ToString for I
where
    I: IntoIterator<Item = T>,
    T: std::convert::TryInto<u32>,
{
    fn to_string(self: Self) -> String {
        self.into_iter()
            .map(|t| t.try_into().ok().unwrap())
            .map(|t| std::convert::TryInto::<char>::try_into(t).ok().unwrap())
            .collect()
    }
}

fn main() {
    let t: usize = read();

    for _ in 0..t {
        let s = read_str()
            .into_iter()
            .map(|c| (c == 'b') as usize)
            .collect::<Vec<_>>();

        let n = s.len();
        let na = s.citer().filter(|&c| c == 0).count();
        let nb = n - na;

        let ans = if na % 2 == 0 && s[n - 1] == 1 {
            // aが偶数、末尾がb
            repeat_n('b', nb).to_string()
        } else if s[n - 1] == 0 {
            // aが偶数または奇数、末尾がa
            let ablock = s
                .citer()
                .group_by(|&x| x)
                .into_iter()
                .filter(|(k, _it)| *k == 0)
                .map(|(_, it)| it.count())
                .collect_vec();
            let ma = ablock
                .citer()
                .take(ablock.len() - 1)
                .filter(|&x| x == 1)
                .count()
                % 2;
            let ma2 = ablock
                .citer()
                .take(ablock.len() - 1)
                .filter(|&x| x >= 2)
                .map(|x| x - 2)
                .sum::<usize>();

            let ka = ablock[ablock.len() - 1] + ma2 - ma;

            repeat_n('b', nb).chain(repeat_n('a', ka)).to_string()
        } else {
            // aが奇数、末尾がb
            if let Some(firstb) = s.citer().position(|c| c == 1) {
                if s.citer().skip(firstb).all_equal() {
                    once('a').chain(repeat_n('b', nb)).to_string()
                } else {
                    let lasta = s.citer().rposition(|c| c == 0).unwrap();

                    if n - lasta - 1 <= 2 {
                        s.citer()
                            .enumerate()
                            .filter_map(|(i, c)| {
                                if c == 1 {
                                    Some('b')
                                } else if i == lasta {
                                    Some('a')
                                } else {
                                    None
                                }
                            })
                            .to_string()
                    } else {
                        let mut ablock = s
                            .citer()
                            .group_by(|&x| x)
                            .into_iter()
                            .filter(|(k, _it)| *k == 0)
                            .map(|(_, it)| it.count())
                            .collect_vec();

                        if ablock.citer().all(|x| x == 1) {
                            repeat_n('b', nb - 2).chain(once('a')).to_string()
                        } else {
                            if ablock[ablock.len() - 1] == 1 {
                                let idx = ablock.citer().rposition(|x| x > 1).unwrap();
                                let al = ablock.len();
                                if idx > 0 || s[0] == 1 {
                                    ablock.swap(idx, al - 1);
                                }
                            }

                            let ma = ablock
                                .citer()
                                .take(ablock.len() - 1)
                                .filter(|&x| x == 1)
                                .count()
                                % 2;
                            let ma2 = ablock
                                .citer()
                                .take(ablock.len() - 1)
                                .filter(|&x| x >= 2)
                                .map(|x| x - 2)
                                .sum::<usize>();

                            let ka = ablock[ablock.len() - 1] + ma2 - ma;
                            // eprintln!("{} {} {} {} {:?}", na, ma, ma2, ka, ablock);

                            repeat_n('b', nb - 2).chain(repeat_n('a', ka)).to_string()
                        }
                    }
                }
            } else {
                repeat_n('a', na).to_string()
            }
        };

        println!("{}", ans);
    }
}
