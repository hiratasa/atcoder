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
use itertools::{Itertools, chain, iproduct, iterate, izip};
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

fn main() {
    let (n, k) = read_tuple!(usize, usize);
    let s = read_vec(n, || read_str());

    let t = s
        .iter()
        .sorted_by(|s0, s1| {
            let tmp0 = s0.citer().chain(s1.citer()).collect::<Vec<_>>();
            let tmp1 = s1.citer().chain(s0.citer()).collect::<Vec<_>>();

            tmp0.cmp(&tmp1)
        })
        .collect::<Vec<_>>();
    const M: usize = 2501;
    let dp = t.iter().fold(vec![(vec![], bitset!(M, 1))], |prev, tt| {
        let mut dp = vec![(vec![], bitset!(M, 1))];

        for (i, (ss, lens)) in prev.iter().enumerate() {
            let (s1, lens1) = (0..M)
                .filter(|&len| lens[len])
                .map(|len| ss[0..len].citer().chain(tt.citer()).collect::<Vec<_>>())
                .fold((vec!['~'], bitset!(M, 0)), |(s0, mut lens0), s1| {
                    if s0 == s1 {
                        (s1, lens0)
                    } else {
                        let i =
                            izip!(s0.citer().chain(repeat('#')), s1.citer().chain(repeat('#')),)
                                .position(|(c0, c1)| c0 != c1)
                                .unwrap();

                        if s0.get(i).copied().unwrap_or('~') < s1.get(i).copied().unwrap_or('~') {
                            if i == s1.len() {
                                lens0.set(i, true);
                            }

                            (s0, lens0)
                        } else {
                            let mut lens2 = bitset!(M, 0);

                            for j in 0..=i {
                                if lens0[j] {
                                    lens2.set(j, true);
                                }
                            }
                            lens2.set(s1.len(), true);

                            (s1, lens2)
                        }
                    }
                });

            // eprintln!("{:?}", s1);
            // eprintln!("{:?}", lens1);
            if i == prev.len() - 1 {
                dp.push((s1, lens1));
            } else {
                let s2 = &prev[i + 1].0;
                let lens2 = &prev[i + 1].1;

                if &s1 == s2 {
                    dp.push((s1, lens2 | &lens1));
                } else {
                    let i = izip!(s1.citer().chain(repeat('#')), s2.citer().chain(repeat('#')),)
                        .position(|(c0, c1)| c0 != c1)
                        .unwrap();

                    if s1.get(i).copied().unwrap_or('~') < s2.get(i).copied().unwrap_or('~') {
                        let mut tmp = lens1;

                        for j in 0..=i {
                            if lens2[j] {
                                tmp.set(j, true);
                            }
                        }

                        dp.push((s1, tmp));
                    } else {
                        let mut tmp = lens2.clone();

                        for j in 0..=i {
                            if lens1[j] {
                                tmp.set(j, true);
                            }
                        }

                        dp.push((s2.clone(), tmp));
                    }
                }
            }
        }

        dp
    });

    // eprintln!("{:?}", dp[k]);
    let ans = &(dp[k].0)[0..(0..M).find(|&len| (dp[k].1)[len]).unwrap()];
    println!("{}", ans.citer().join(""));
}
