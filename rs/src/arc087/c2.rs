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

struct Node {
    zero: Option<Box<Node>>,
    one: Option<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            zero: None,
            one: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.zero.is_none() && self.one.is_none()
    }
}

fn insert(node: &mut Node, str: &[char], idx: usize) {
    if str.len() == idx {
        return;
    }

    if str[idx] == '0' {
        let mut child = node.zero.take().unwrap_or(Box::new(Node::new()));
        insert(&mut child, str, idx + 1);
        node.zero = Some(child);
    } else {
        let mut child = node.one.take().unwrap_or(Box::new(Node::new()));
        insert(&mut child, str, idx + 1);
        node.one = Some(child);
    }
}

fn grundy(node: &Node, depth: usize, l: usize) -> usize {
    let mut g = 0;

    if node.is_leaf() {
        return 0;
    }

    if let Some(child) = &node.zero {
        g ^= grundy(child, depth + 1, l);
    } else {
        g ^= 1 << (l - depth).trailing_zeros();
    }

    if let Some(child) = &node.one {
        g ^= grundy(child, depth + 1, l);
    } else {
        g ^= 1 << (l - depth).trailing_zeros();
    }

    g
}

fn main() {
    let (n, l) = read_tuple!(usize, usize);

    let s = read_vec(n, || read_str());

    let mut trie = Node::new();
    for str in s {
        insert(&mut trie, &str, 0);
    }

    let g = grundy(&trie, 0, l);

    if g == 0 {
        println!("Bob");
    } else {
        println!("Alice");
    }
}
