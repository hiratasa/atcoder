#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64;
#[allow(unused_imports)]
use std::i64;
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
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
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
    };
    ($($x:expr),+,) => {
        it![$($x),+]
    };
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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

struct BinaryTrie {
    count: usize,
    result: usize,
    children: [Option<usize>; 2],
}

impl BinaryTrie {
    fn new() -> Vec<BinaryTrie> {
        vec![BinaryTrie {
            count: 0,
            result: 0,
            children: [None; 2],
        }]
    }

    fn get_child(buf: &mut Vec<BinaryTrie>, idx: usize, c: usize) -> usize {
        if let Some(idx2) = buf[idx].children[c] {
            idx2
        } else {
            buf.push(BinaryTrie {
                count: 0,
                result: 0,
                children: [None; 2],
            });
            buf[idx].children[c] = Some(buf.len() - 1);
            buf.len() - 1
        }
    }

    fn update_result(buf: &mut [BinaryTrie], idx: usize, len: usize) {
        if len == 0 {
            return;
        }

        buf[idx].result = if buf[idx].count == 0 {
            0
        } else if buf[idx].count == 1 {
            Self::get_value(buf, idx, len)
        } else {
            if let Some(x) = buf[idx]
                .children
                .citer()
                .flatten()
                .filter(|&cidx| buf[cidx].count >= 2)
                .map(|cidx| buf[cidx].result)
                .min()
            {
                x
            } else {
                (buf[buf[idx].children[0].unwrap()].result
                    ^ buf[buf[idx].children[1].unwrap()].result)
                    + (1 << (len - 1))
            }
        };
    }

    fn add(buf: &mut Vec<BinaryTrie>, idx: usize, x: usize, len: usize) {
        buf[idx].count += 1;

        if len == 0 {
            return;
        }

        let child = Self::get_child(buf, idx, (x >> (len - 1)) & 1);

        Self::add(buf, child, x, len - 1);

        Self::update_result(buf, idx, len);
    }

    fn remove(buf: &mut Vec<BinaryTrie>, idx: usize, x: usize, len: usize) {
        buf[idx].count -= 1;

        if len == 0 {
            if buf[idx].count >= 2 {
                buf[idx].result = 0;
            } else {
                buf[idx].result = usize::MAX;
            }

            return;
        }

        let child = Self::get_child(buf, idx, (x >> (len - 1)) & 1);

        Self::remove(buf, child, x, len - 1);

        Self::update_result(buf, idx, len);
    }

    fn get_value(buf: &[BinaryTrie], idx: usize, len: usize) -> usize {
        assert!(buf[idx].count == 1);

        if len == 0 {
            return 0;
        }

        buf[idx]
            .children
            .citer()
            .enumerate()
            .find_map(|(c, cidx)| {
                let cidx = cidx?;
                let child = &buf[cidx];
                if child.count == 1 {
                    Some((1 << (len - 1)) * c + Self::get_value(buf, cidx, len - 1))
                } else {
                    None
                }
            })
            .unwrap()
    }
}

fn main() {
    let q = read::<usize>();
    let query = read_vec(q, || read_row::<usize>());

    const M: usize = 32;

    query
        .iter()
        .scan(BinaryTrie::new(), |trie, qu| match qu[0] {
            1 => {
                BinaryTrie::add(trie, 0, qu[1], M);
                Some(None)
            }
            2 => {
                BinaryTrie::remove(trie, 0, qu[1], M);
                Some(None)
            }
            3 => Some(Some(trie[0].result)),
            _ => unreachable!(),
        })
        .flatten()
        .for_each(|ans| {
            println!("{}", ans);
        })
}
