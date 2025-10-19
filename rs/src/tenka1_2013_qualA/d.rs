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

#[derive(Debug)]
enum Compressed {
    Repeat {
        words: Vec<Compressed>,
        num_repeat: usize,
        unit_len: usize,
    },
    Word(Vec<char>),
}

impl Compressed {
    fn len(&self) -> usize {
        match self {
            Compressed::Repeat {
                words: _,
                num_repeat,
                unit_len,
            } => num_repeat * unit_len,
            Compressed::Word(word) => word.len(),
        }
    }
}

fn parse(s: &[char]) -> (Vec<Compressed>, usize) {
    let mut v = vec![];

    let mut i = 0;
    while i < s.len() {
        match s[i] {
            '(' => {
                i += 1;
                let (words, num_read) = parse(&s[i..]);
                i += num_read;
                assert!(s[i] == ')');
                i += 1;
                let num_repeat_str = s[i..]
                    .citer()
                    .take_while(|&c| c.is_digit(10))
                    .collect::<String>();
                let num_repeat = num_repeat_str.parse().unwrap();

                if num_repeat == 1 {
                    v.extend(words);
                } else {
                    let unit_len = words.iter().map(|w| w.len()).sum::<usize>();
                    v.push(Compressed::Repeat {
                        words,
                        num_repeat,
                        unit_len,
                    });
                }
                i += num_repeat_str.len();
            }
            ')' => {
                // unmatched closing bracket
                return (v, i);
            }
            'A'..='Z' | 'a'..='z' => {
                if let Some(Compressed::Word(word)) = v.last_mut() {
                    word.push(s[i]);
                } else {
                    v.push(Compressed::Word(vec![s[i]]));
                }
                i += 1;
            }
            _ => unreachable!(),
        }
    }

    (v, s.len())
}

fn query(s: &[Compressed], mut idx: usize, mut len: usize) -> Vec<char> {
    let mut ret = vec![];

    for p in s {
        if idx >= p.len() {
            idx -= p.len();
        } else {
            match p {
                Compressed::Repeat {
                    words,
                    num_repeat,
                    unit_len,
                } => {
                    let m = idx / unit_len;
                    let mut w = query(words, idx % unit_len, len);
                    for _ in m + 1..*num_repeat {
                        let wlen = w.len();
                        if len == wlen {
                            break;
                        }
                        assert!(len > wlen);
                        w.extend(query(words, 0, len - wlen));
                    }
                    idx = 0;
                    len -= w.len();
                    ret.extend(w);
                }
                Compressed::Word(word) => {
                    ret.extend(&word[idx..min(idx + len, word.len())]);
                    len -= min(idx + len, word.len()) - idx;
                    idx = 0;
                }
            }
        }

        if len == 0 {
            break;
        }
    }

    ret
}

fn main() {
    let (b, l, n) = read_tuple!(i64, usize, usize);

    let s = read_str();

    let (p, _) = parse(&s);

    // eprintln!("{:?}", p);

    let ans = if b >= 0 {
        query(&p, b as usize, l)
    } else {
        let len = p.iter().map(|q| q.len()).sum::<usize>();
        query(&p, (len as i64 + b) as usize, l)
    };

    println!("{}", ans.citer().collect::<String>());
}
