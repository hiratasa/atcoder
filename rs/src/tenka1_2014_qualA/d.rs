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
    let n = read::<usize>();
    let barriers = read_vec(n, || read_tuple!(i64, i64, i64, i64));

    let point_idxs = barriers
        .citer()
        .flat_map(|(x0, y0, x1, y1)| it![(x0, y0), (x1, y1)])
        .sorted_by(|&(x0, y0), &(x1, y1)| {
            ((y0, x0) < (0, 0))
                .cmp(&((y1, x1) < (0, 0)))
                .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
        })
        .map(|p| vec![p])
        .coalesce(|ps0, ps1| {
            let (x0, y0) = ps0[0];
            let (x1, y1) = ps1[0];
            if ((y0, x0) < (0, 0)) == ((y1, x1) < (0, 0)) && x1 * y0 == x0 * y1 {
                let mut ps0 = ps0;
                ps0.extend(ps1);
                Ok(ps0)
            } else {
                Err((ps0, ps1))
            }
        })
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (i, ps)| {
            map.extend(ps.citer().map(|p| (p, i)));
            map
        });

    let m = *point_idxs.values().max().unwrap() + 1;

    let barriers_idx = barriers
        .citer()
        .map(|(x0, y0, x1, y1)| {
            if x0 * y1 - x1 * y0 > 0 {
                (x0, y0, x1, y1)
            } else {
                (x1, y1, x0, y0)
            }
        })
        .map(|(x0, y0, x1, y1)| (point_idxs[&(x0, y0)], point_idxs[&(x1, y1)]))
        // .map(|(idx0, idx1)| {
        //     if idx0 <= idx1 {
        //         (idx0, idx1)
        //     } else {
        //         (idx0, idx1 + m)
        //     }
        // })
        .sorted_by_key(|&(idx0, idx1)| (idx1, idx0))
        .collect::<Vec<_>>();

    let ans = (0..m)
        .map(|i| {
            let j0 = barriers_idx
                .citer()
                .position(|(_idx0, idx1)| i < idx1)
                .unwrap_or(n);

            1 + (j0..j0 + n)
                .map(|j| barriers_idx[j % n])
                // to relative idx
                .map(|(idx0, idx1)| ((idx0 + m - i) % m, (idx1 + m - i) % m))
                .filter(|&(idx0, idx1)| idx0 <= idx1)
                .scan(0, |last, (idx0, idx1)| {
                    // eprintln!(
                    //     "#i={}, j0={}, last={}, (idx0, idx1)=({}, {})",
                    //     i, j0, *last, idx0, idx1
                    // );
                    if idx0 <= *last {
                        Some(0)
                    } else {
                        *last = idx1;
                        Some(1)
                    }
                })
                .sum::<usize>()
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
