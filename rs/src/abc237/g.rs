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

#[derive(Debug, Clone, Copy)]
struct Block {
    l: usize,
    r: usize,
    val: usize,
}

fn main() {
    let (n, q, x) = read_tuple!(usize, usize, usize);
    let p = read_row::<usize>();
    let query = read_vec(q, || read_tuple!(usize, usize, usize));

    let a = p
        .citer()
        .map(|z| match z.cmp(&x) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        })
        .enumerate()
        .map(|(i, k)| {
            (
                i + 1,
                Block {
                    l: i,
                    r: i + 1,
                    val: k,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let a = query.citer().fold(a, |mut a, (c, l, r)| {
        let l = l - 1;
        let asc = c == 1;

        let keys = a
            .range(l + 1..)
            .take_while(|(_, block)| block.l < r)
            .map(|(rr, _)| *rr)
            .collect::<Vec<_>>();
        let mut nums = [0; 3];
        for rr in keys {
            let mut block = a.remove(&rr).unwrap();

            if block.l < l {
                let mut block0 = block;
                let mut block1 = block;

                block0.r = l;
                block1.l = l;

                a.insert(block0.r, block0);
                block = block1;
            }

            if block.r > r {
                let mut block0 = block;
                let mut block1 = block;

                block0.r = r;
                block1.l = r;

                a.insert(block1.r, block1);
                block = block0;
            }

            nums[block.val] += block.r - block.l;
        }

        if asc {
            a.extend(
                (0..3)
                    .filter(|&i| nums[i] > 0)
                    .scan(l, |ll, i| Some((i, replace(ll, *ll + nums[i]))))
                    .map(|(i, ll)| {
                        (
                            ll + nums[i],
                            Block {
                                l: ll,
                                r: ll + nums[i],
                                val: i,
                            },
                        )
                    }),
            );
        } else {
            a.extend(
                (0..3)
                    .rev()
                    .filter(|&i| nums[i] > 0)
                    .scan(l, |ll, i| Some((i, replace(ll, *ll + nums[i]))))
                    .map(|(i, ll)| {
                        (
                            ll + nums[i],
                            Block {
                                l: ll,
                                r: ll + nums[i],
                                val: i,
                            },
                        )
                    }),
            );
        }

        a
    });

    let ans = a
        .values()
        .find(|block| block.val == 1)
        .map(|block| block.l)
        .unwrap();

    println!("{}", ans + 1);
}
