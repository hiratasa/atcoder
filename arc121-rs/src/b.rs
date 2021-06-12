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
    let n: usize = read();

    let ac = read_vec(2 * n, || read_tuple!(i64, char));

    let mut nums = ac.citer().fold(vec![vec![]; 3], |mut nums, (a, c)| {
        match c {
            'R' => nums[0].push(a),
            'G' => nums[1].push(a),
            'B' => nums[2].push(a),
            _ => unreachable!(),
        };
        nums
    });

    if nums.iter().all(|v| v.len() % 2 == 0) {
        println!("0");
        return;
    }

    if nums[0].len() % 2 == 0 {
        nums.swap(0, 2);
    }

    if nums[1].len() % 2 == 0 {
        nums.swap(1, 2);
    }

    assert!(nums[0].len() % 2 > 0);
    assert!(nums[1].len() % 2 > 0);
    assert!(nums[2].len() % 2 == 0);

    nums[0].sort();
    nums[1].sort();
    nums[2].sort();

    let calc_min_match = |nums0: &[i64], nums1: &[i64]| {
        assert!(!nums0.is_empty());
        nums0
            .citer()
            .map(|a| {
                let idx = nums1
                    .binary_search_by(|&b| b.cmp(&a).then(Ordering::Greater))
                    .unwrap_err();

                let x = nums1.get(idx).map_or(std::i64::MAX, |b| (a - b).abs());
                let y = idx
                    .checked_sub(1)
                    .and_then(|j| nums1.get(j))
                    .map_or(std::i64::MAX, |b| (a - b).abs());

                min(x, y)
            })
            .min()
            .unwrap()
    };

    let ans0 = calc_min_match(&nums[0], &nums[1]);
    let ans1 =
        calc_min_match(&nums[0], &nums[2]).saturating_add(calc_min_match(&nums[1], &nums[2]));
    let ans = min(ans0, ans1);

    println!("{}", ans);
}
