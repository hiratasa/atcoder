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

fn basis(nums: &[usize], k: usize) -> Option<Vec<usize>> {
    if nums.len() < k {
        return None;
    }

    (0..k)
        .try_fold(
            (nums.to_vec(), (0..nums.len()).collect::<Vec<_>>()),
            |(mut nums, mut idxs), i| {
                let j = (i..nums.len()).find(|&j| nums[j] & (1 << i) > 0)?;
                nums.swap(i, j);
                idxs.swap(i, j);

                for l in i + 1..nums.len() {
                    if nums[l] & (1 << i) > 0 {
                        nums[l] ^= nums[i];
                    }
                }

                Some((nums, idxs))
            },
        )
        .map(|(_, idxs)| idxs[0..k].citer().map(|i| nums[i]).collect())
}

fn main() {
    let (n, m) = read_tuple!(usize, usize);
    let a = read_row::<usize>();

    let k = n.trailing_zeros() as usize;

    let b = a
        .citer()
        .fold(vec![true; 1 << k], |mut b, aa| {
            b[aa] = false;
            b
        })
        .into_iter()
        .positions(|bb| bb)
        .skip(1)
        .collect::<Vec<_>>();

    let nums = if let Some(nums) = basis(&b, k) {
        nums
    } else {
        println!("-1");
        return;
    };

    (0..1 << k)
        // gray code
        .map(|t| t ^ (t >> 1))
        .map(|s| {
            let bs = bitset!(k, s);
            (0..k)
                .filter(|&i| bs[i])
                .map(|i| nums[i])
                .fold(0, |x, y| x ^ y)
        })
        .tuple_windows()
        .for_each(|(u, v)| {
            println!("{} {}", u, v);
        })
}
