#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

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
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

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

trait IteratorDpExt: Iterator + Sized {
    fn dp<T, F: FnMut(&Vec<T>, Self::Item) -> T>(self, init: Vec<T>, mut f: F) -> Vec<T> {
        self.fold(init, |mut dp, item| {
            let next = f(&dp, item);
            dp.push(next);
            dp
        })
    }
}

impl<I> IteratorDpExt for I where I: Iterator + Sized {}

fn main() {
    let s1 = read_str();
    let s2 = read_str();
    let s3 = read_str();

    let t1 = s1.iter().copied().fold(vec![0usize; 26], |mut nums, c| {
        nums[c as usize - 'A' as usize] += 1;
        nums
    });

    let t2 = s2.iter().copied().fold(vec![0usize; 26], |mut nums, c| {
        nums[c as usize - 'A' as usize] += 1;
        nums
    });

    let t3 = s3.iter().copied().fold(vec![0usize; 26], |mut nums, c| {
        nums[c as usize - 'A' as usize] += 1;
        nums
    });

    let mut init = BitSet::new(s1.len() / 2 + 1);
    init.set(0, true);
    let ans = (0..='Z' as usize - 'A' as usize)
        .try_fold(init, |bs, i| {
            if t1[i] + t2[i] >= t3[i] {
                let lower = if t1[i] >= t3[i] { 0 } else { t3[i] - t1[i] };
                let upper = if t2[i] >= t3[i] { t3[i] } else { t2[i] };
                assert!(lower <= upper);

                let mut next = BitSet::new(s1.len() / 2 + 1);
                for j in lower..=upper {
                    next |= &(&bs << j);
                }
                Some(next)
            } else {
                None
            }
        })
        .filter(|bs| bs[s1.len() / 2])
        .is_some();
    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
