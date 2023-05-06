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
    num: usize,
    children: [Option<Box<BinaryTrie>>; 2],
}

impl BinaryTrie {
    fn new() -> BinaryTrie {
        BinaryTrie {
            num: 0,
            children: [None, None],
        }
    }

    fn insert(&mut self, x: usize, n: usize) {
        if n == 0 {
            self.num += 1;
        } else {
            self.children[(x >> (n - 1)) & 1]
                .get_or_insert_with(|| Box::new(BinaryTrie::new()))
                .insert(x, n - 1);
        }
    }

    fn remove(&mut self, x: usize, n: usize) -> bool {
        if n == 0 {
            self.num -= 1;
            self.num == 0
        } else {
            let b = (x >> (n - 1)) & 1;
            if self.children[b].as_mut().unwrap().remove(x, n - 1) {
                self.children[b] = None;

                self.children[b ^ 1].is_none()
            } else {
                false
            }
        }
    }

    fn get_min(&self, xor: usize, n: usize) -> usize {
        if n == 0 {
            assert!(self.num > 0);
            0
        } else {
            let b = (xor >> (n - 1)) & 1;

            it![b, b ^ 1]
                .find_map(|bb| {
                    self.children[bb]
                        .as_ref()
                        .map(|c| c.get_min(xor, n - 1) ^ ((b ^ bb) << (n - 1)))
                })
                .unwrap()
        }
    }
}

fn main() {
    let n = read::<usize>();
    let a = read_row::<usize>();

    let b = a
        .citer()
        .scan(0, |x, y| {
            *x ^= y;
            Some(*x)
        })
        .collect::<Vec<_>>();

    let trie = b[..n - 1].citer().fold(BinaryTrie::new(), |mut trie, x| {
        trie.insert(x, 30);
        trie
    });

    let ans = (0..n)
        .scan((trie, 0), |(trie, mask), i| {
            if i < n - 1 {
                let x = trie.get_min(*mask, 30);
                *mask ^= x;
                trie.remove(*mask, 30);
                Some(x)
            } else {
                Some(*mask ^ b[n - 1])
            }
        })
        .collect::<Vec<_>>();

    println!("{}", ans.citer().join(" "));
}
