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

fn get_key(colors: &[u16]) -> [u16; 4] {
    (0..4)
        .map(|i| {
            colors
                .citer()
                .cycle()
                .skip(i)
                .take(4)
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .map(|x| [x.0, x.1, x.2, x.3])
        .min()
        .unwrap()
}

fn main() {
    let n: usize = read();
    let c = read_mat::<u16>(n);

    let add = |tiles: &mut FxHashMap<[u16; 4], usize>, colors: &[u16]| {
        *tiles.entry(get_key(colors)).or_insert(0) += 1;
    };

    let remove = |tiles: &mut FxHashMap<[u16; 4], usize>, colors: &[u16]| {
        *tiles.entry(get_key(colors)).or_insert(0) -= 1;
    };

    let tiles = c.iter().fold(
        FxHashMap::default(),
        |mut map: FxHashMap<[u16; 4], usize>, colors| {
            add(&mut map, colors);

            map
        },
    );

    let ans = c
        .iter()
        .enumerate()
        .scan(tiles, |tiles, (i, colors)| {
            let key = get_key(colors);

            remove(tiles, colors);

            let ret = c[i + 1..]
                .iter()
                .map(|colors2| {
                    remove(tiles, colors2);

                    let ret = (0..4)
                        .map(|i| {
                            colors2
                                .citer()
                                .rev()
                                .cycle()
                                .skip(i)
                                .take(4)
                                .collect_tuple::<(_, _, _, _)>()
                                .unwrap()
                        })
                        .map(|x| [x.0, x.1, x.2, x.3])
                        .map(|key2| {
                            (0..4)
                                .map(|j| [key[(j + 1) % 4], key[j], key2[j], key2[(j + 1) % 4]])
                                .map(|y| get_key(&y))
                                .sorted()
                                .group_by(|&y| y)
                                .into_iter()
                                .map(|(y, it)| (y, it.count()))
                                .map(|(y, k)| {
                                    let m = tiles.get(&y).copied().unwrap_or(0);
                                    (0..=m).rev().take(k).product::<usize>()
                                        * (0..4)
                                            .filter(|&l| {
                                                y.citer()
                                                    .cycle()
                                                    .skip(l)
                                                    .take(4)
                                                    .collect_tuple::<(_, _, _, _)>()
                                                    .unwrap()
                                                    == (y[0], y[1], y[2], y[3])
                                            })
                                            .count()
                                            .pow(k as u32)
                                })
                                .product::<usize>()
                        })
                        .sum::<usize>();

                    add(tiles, colors2);

                    ret
                })
                .sum::<usize>();

            Some(ret)
        })
        .sum::<usize>();

    println!("{}", ans);
}
