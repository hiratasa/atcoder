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

fn main() {
    let s = read_str();
    let k: usize = read();

    let n = s.len();

    let k = min(k, n * (n - 1) / 2);

    let s = s
        .citer()
        .map(|c| match c {
            'K' => 0,
            'E' => 1,
            'Y' => 2,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let pos = (0..3)
        .map(|i| s.citer().positions(|c| c == i).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nums = pos.iter().map(|v| v.len()).collect::<Vec<_>>();

    let dp = (0..n).fold(
        once(([0, 0, 0], vvec![1; 0; k + 1])).collect::<FxHashMap<_, _>>(),
        |prev, i| {
            let mut dp = FxHashMap::default();

            for (m, ts) in prev {
                for j in 0..3 {
                    if m[j] == nums[j] {
                        continue;
                    }

                    let i0 = pos[j][m[j]];
                    let t2 = (0..3)
                        .map(|k| {
                            if k == j {
                                0
                            } else {
                                let idx = pos[k].binary_search_by(|&p| p.cmp(&i0)).unwrap_err();

                                m[k].saturating_sub(idx)
                            }
                        })
                        .sum::<usize>();

                    let mut m2 = m;
                    m2[j] += 1;

                    for t in 0..=k {
                        if t + t2 > k {
                            break;
                        }

                        dp.entry(m2).or_insert(vec![0; k + 1])[t + t2] += ts[t];
                    }
                }
            }

            dp
        },
    );

    let ans = dp[&[nums[0], nums[1], nums[2]]].citer().sum::<usize>();
    println!("{}", ans);
}
