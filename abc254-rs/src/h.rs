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

use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
struct TrieNode {
    children: [Option<NonZeroUsize>; 2],
    num_a: usize,
    num_b: usize,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            children: [None; 2],
            num_a: 0,
            num_b: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Trie {
    buf: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            buf: vec![TrieNode::new()],
        }
    }

    fn ensure(&mut self, idx: usize, i: usize) -> usize {
        if let Some(cidx) = self.buf[idx].children[i] {
            cidx.get()
        } else {
            self.buf.push(TrieNode::new());
            let cidx = self.buf.len() - 1;
            self.buf[idx].children[i] = NonZeroUsize::new(cidx);
            cidx
        }
    }

    fn add_impl(&mut self, x: usize, is_b: bool, bit_pos: usize, idx: usize) -> usize {
        assert!(idx < self.buf.len());

        if bit_pos > 0 {
            let cidx = self.ensure(idx, (x >> (bit_pos - 1)) & 1);
            self.add_impl(x, is_b, bit_pos - 1, cidx)
        } else {
            if is_b {
                self.buf[idx].num_b += 1;
            } else {
                self.buf[idx].num_a += 1;
            }

            idx
        }
    }

    fn add(&mut self, x: usize, is_b: bool) -> usize {
        self.add_impl(
            x,
            is_b,
            std::mem::size_of::<usize>() * 8 - x.leading_zeros() as usize,
            0,
        )
    }

    fn solve(&self, idx: usize) -> Option<(usize, usize, usize)> {
        let (x, sum_a, sum_b) = self.buf[idx]
            .children
            .citer()
            .enumerate()
            .filter_map(|(v, cidx)| cidx.map(|cidx| (v, cidx)))
            .map(|(v, cidx)| (v, self.solve(cidx.get())))
            .map(|(v, res)| {
                if v == 1 && matches!(res, Some((_, _, sum_b)) if sum_b > 0) {
                    None
                } else {
                    res
                }
            })
            .try_fold(
                (0, self.buf[idx].num_a, self.buf[idx].num_b),
                |(x, y, z), res| {
                    let (x1, y1, z1) = res?;
                    Some((x + x1 + y1 + z1, y + y1, z + z1))
                },
            )?;

        let r = min(sum_a, sum_b);

        Some((x, sum_a - r, sum_b - r))
    }
}

fn main() {
    let n = read::<usize>();
    let a = read_row::<usize>();
    let b = read_row::<usize>();

    let trie = chain(a.citer().map(|x| (x, false)), b.citer().map(|x| (x, true))).fold(
        Trie::new(),
        |mut trie, (x, is_b)| {
            trie.add(x, is_b);
            trie
        },
    );

    println_opt(trie.solve(0).map(|(x, _, _)| x));
}
