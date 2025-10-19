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
fn solve0(a: &[usize], b: &[usize]) -> (usize, Vec<usize>, Vec<usize>) {
    iproduct!(
        a.citer().permutations(a.len()),
        b.citer().permutations(b.len())
    )
    .map(|(a2, b2)| {
        let s = izip!(a2.citer().chain(repeat(0)), b2.citer().chain(repeat(0)),)
            .take(max(a.len(), b.len()) + 1)
            .scan(0, |carry, (x, y)| {
                let z = *carry + x + y;

                *carry = z / 10;
                Some(z % 10)
            })
            .sum::<usize>();
        (s, a2.to_vec(), b2.to_vec())
    })
    .min()
    .unwrap()
}

fn main() {
    let a = read_str()
        .into_iter()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let b = read_str()
        .into_iter()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    // use rand::rngs::SmallRng;
    // use rand::{Rng, SeedableRng};
    // let mut rng = SmallRng::seed_from_u64(42);

    // let n_a = rng.random_range(3, 5);
    // let a = repeat_with(|| rng.random_range(1, 10))
    //     .take(n_a)
    //     .collect::<Vec<_>>();
    // let n_b = rng.random_range(3, 5);
    // let b = repeat_with(|| rng.random_range(1, 10))
    //     .take(n_b)
    //     .collect::<Vec<_>>();

    let (a, b, swapped) = if a.len() < b.len() {
        (a, b, false)
    } else {
        (b, a, true)
    };

    let nums_a = a.citer().fold(vec![0usize; 10], |mut nums_a, d| {
        nums_a[d] += 1;
        nums_a
    });

    let nums_b = b.citer().fold(vec![0usize; 10], |mut nums_b, d| {
        nums_b[d] += 1;
        nums_b
    });

    let (a2, b2, nums2, _) = (0..10)
        .filter(|&d0| nums_a[d0] > 0 && nums_b[10 - d0..].citer().any(|x| x > 0))
        .map(|d0| {
            let d1 = nums_b
                .citer()
                .enumerate()
                .skip(10 - d0)
                .find(|&(_, x)| x > 0)
                .unwrap()
                .0;

            let mut nums_b2 = nums_b.clone();
            nums_b2[d1] -= 1;

            let (a2, b2, nums, z, last_carry) = (0..10)
                .rev()
                .flat_map(|d| repeat(d).take(if d == d0 { nums_a[d] - 1 } else { nums_a[d] }))
                .fold(
                    (vec![d0], vec![d1], nums_b2, 0usize, true),
                    |(mut a2, mut b2, mut nums, z, _last_carry), d| {
                        let st = 9 - d;

                        let (d2, zz) = if let Some((d2, _)) =
                            nums.citer().enumerate().skip(st).find(|&(_, x)| x > 0)
                        {
                            (d2, 1)
                        } else {
                            (nums.citer().position(|x| x > 0).unwrap(), 0)
                        };

                        a2.push(d);
                        b2.push(d2);
                        nums[d2] -= 1;

                        (a2, b2, nums, z + zz, zz > 0)
                    },
                );

            if last_carry {
                let zz = nums[9];
                (a2, b2, nums, z + zz)
            } else {
                (a2, b2, nums, z)
            }
        })
        .max_by_key(|(_, _, _, z)| *z)
        .unwrap_or((a.clone(), b.clone(), vec![0; 10], 0));

    let a3 = a2.citer().rev().join("");
    let b3 = nums2
        .citer()
        .enumerate()
        .flat_map(|(i, x)| repeat(i).take(x))
        .chain(b2.citer().rev())
        .join("");
    if !swapped {
        println!("{}", a3);
        println!("{}", b3);
    } else {
        println!("{}", b3);
        println!("{}", a3);
    }

    // let s = izip!(
    //     a3.chars()
    //         .rev()
    //         .map(|d| d.to_digit(10).unwrap() as usize)
    //         .chain(repeat(0)),
    //     b3.chars()
    //         .rev()
    //         .map(|d| d.to_digit(10).unwrap() as usize)
    //         .chain(repeat(0)),
    // )
    // .take(max(a.len(), b.len()) + 1)
    // .scan(0, |carry, (x, y)| {
    //     let z = *carry + x + y;
    //     *carry = z / 10;
    //     Some(z % 10)
    // })
    // .sum::<usize>();
    // let (s0, a0, b0) = solve0(&a, &b);
    // if s != s0 {
    //     println!("{}", a3);
    //     println!("{}", b3);
    //     println!("{}", a0.citer().rev().join(""));
    //     println!("{}", b0.citer().rev().join(""));
    //     println!("{} vs {}", s, s0);
    // }
}
