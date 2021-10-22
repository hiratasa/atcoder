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
use itertools::{chain, iproduct, iterate, izip, Itertools};
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

fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
}

fn main() {
    let (n, k) = read_tuple!(usize, usize);

    let s = read_vec(n, || read_str());

    let dp = (0..n)
        .rev()
        .scan(bitset!(k + 1, 1), |dp, i| {
            let a = dp.clone() << s[i].len();
            *dp |= &a;
            Some(a)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let (ans, _) = (0..n).fold(
        (vec![], bitset!(k + 1, 1)),
        |(mut t, mut bs): (Vec<char>, _), i| {
            let z = z_algorithm(&s[i].citer().chain(t.citer()).collect::<Vec<_>>());
            let z = pushed!(z, 0);

            // j番目以降をs[i]に置き換えたものとl番目以降をs[i]に置き換えたものの最初に異なる箇所
            let first_diff_pos = |j: usize, l: usize| -> usize {
                let (j, l) = if j < l { (j, l) } else { (l, j) };

                let idx = min(z[s[i].len() + j], s[i].len());

                if j + idx < l {
                    j + idx
                } else {
                    let idx2 = min(z[l - j], s[i].len() - (l - j));

                    l + idx2
                }
            };

            // j番目以降をs[i]に置き換えた文字列のidx番目
            let get = |j: usize, idx: usize| -> char {
                if idx < j {
                    t[idx]
                } else {
                    s[i].get(idx - j).copied().unwrap_or(std::char::MAX)
                }
            };

            if let Some(j0) = (0..=t.len())
                .filter(|&j| bs[j] && dp[i][k - j])
                .filter(|&j| {
                    // 元より悪いやつと内包されてるやつをはじく
                    let idx = min(z[s[i].len() + j], s[i].len());

                    s[i].get(idx).copied().unwrap_or(std::char::MAX)
                        < t.get(j + idx).copied().unwrap_or(std::char::MAX)
                })
                .min_by(|&j, &l| {
                    let pos = first_diff_pos(j, l);

                    get(j, pos).cmp(&get(l, pos))
                })
            {
                let idx = min(z[s[i].len() + j0], s[i].len());

                for l in j0 + idx + 1..=k {
                    bs.set(l, false);
                }

                let bs2 = (0..=t.len())
                    .filter(|&j| bs[j] && dp[i][k - j])
                    .filter_map(|j| {
                        let pos = first_diff_pos(j, j0);

                        // 新しい文字列に内包されているか
                        if get(j, pos) == std::char::MAX {
                            Some(pos)
                        } else {
                            None
                        }
                    })
                    .fold(bitset!(k + 1, 0), |mut bs2, pos| {
                        bs2.set(pos, true);
                        bs2
                    });

                t.resize_with(j0, || unreachable!());
                t.extend(s[i].citer());

                (t, bs | &bs2)
            } else {
                (t, bs)
            }
        },
    );

    println!("{}", ans.citer().join(""));
}
