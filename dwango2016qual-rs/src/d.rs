#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::*;
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
    let (h, w) = read_tuple!(usize, usize);

    let b = read_mat::<i64>(h);

    let c = chain(
        std::iter::once(vec![0; w]),
        b.iter().scan(vec![0; w], |prev, r| {
            zip(prev.iter_mut(), r.iter()).for_each(|(p, bb)| *p += bb);

            Some(prev.clone())
        }),
    )
    .collect_vec();

    let from_topleft: Vec<Vec<i64>> = (0..h)
        .map(|u| {
            (0..=u)
                .map(|l| {
                    (0..w)
                        .scan((0i64, 0i64), |(cum, minimum), x| {
                            *cum += c[u + 1][x] - c[l][x];

                            let tmp = *minimum;
                            *minimum = std::cmp::min(*minimum, *cum);

                            Some(*cum - tmp)
                        })
                        .collect_vec()
                })
                .fold1(|mut maximum_row, row| {
                    zip(&mut maximum_row, &row)
                        .for_each(|(mm, mm2)| *mm = std::cmp::max(*mm, *mm2));
                    maximum_row
                })
                .unwrap()
        })
        .collect_vec();

    let from_bottomright: Vec<Vec<i64>> = (0..h)
        .map(|l| {
            (l..h)
                .map(|u| {
                    (0..w)
                        .rev()
                        .scan((0i64, 0i64), |(cum, minimum), x| {
                            *cum += c[u + 1][x] - c[l][x];

                            let tmp = *minimum;
                            *minimum = std::cmp::min(*minimum, *cum);

                            Some(*cum - tmp)
                        })
                        .collect_vec()
                })
                .update(|row| row.reverse())
                .fold1(|mut maximum_row, row| {
                    zip(&mut maximum_row, &row)
                        .for_each(|(mm, mm2)| *mm = std::cmp::max(*mm, *mm2));
                    maximum_row
                })
                .unwrap()
        })
        .collect_vec();

    let from_left = from_topleft
        .iter()
        .fold(vec![std::i64::MIN; w], |mut maximum_row, row| {
            zip(maximum_row.iter_mut(), row.iter())
                .for_each(|(mm, mm2)| *mm = std::cmp::max(*mm, *mm2));
            maximum_row
        })
        .into_iter()
        .scan(std::i64::MIN, |maximum, aa| {
            *maximum = std::cmp::max(*maximum, aa);
            Some(*maximum)
        })
        .collect_vec();

    let from_right = from_bottomright
        .iter()
        .fold(vec![std::i64::MIN; w], |mut maximum_row, row| {
            zip(maximum_row.iter_mut(), row.iter())
                .for_each(|(mm, mm2)| *mm = std::cmp::max(*mm, *mm2));
            maximum_row
        })
        .into_iter()
        .rev()
        .scan(std::i64::MIN, |maximum, aa| {
            *maximum = std::cmp::max(*maximum, aa);
            Some(*maximum)
        })
        .collect_vec();

    let from_top = from_topleft
        .iter()
        .map(|row| row.iter().copied().max().unwrap())
        .scan(std::i64::MIN, |maximum, aa| {
            *maximum = std::cmp::max(*maximum, aa);
            Some(*maximum)
        })
        .collect_vec();

    let from_bottom = from_bottomright
        .iter()
        .map(|row| row.iter().copied().max().unwrap())
        .rev()
        .scan(std::i64::MIN, |maximum, aa| {
            *maximum = std::cmp::max(*maximum, aa);
            Some(*maximum)
        })
        .collect_vec();

    let ans_leftright = (0..w - 1)
        .map(|i| from_left[i] + from_right[w - 2 - i])
        .max()
        .unwrap();
    let ans_topbottom = (0..h - 1)
        .map(|i| from_top[i] + from_bottom[h - 2 - i])
        .max()
        .unwrap();

    let ans = std::cmp::max(ans_leftright, ans_topbottom);

    println!("{}", ans);
}
