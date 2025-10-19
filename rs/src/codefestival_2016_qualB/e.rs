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

#[derive(Clone, Debug)]
struct Trie<'a> {
    idx: usize,
    parent: Option<(char, usize)>,
    children: FxHashMap<char, usize>,
    s: &'a str,
    sum: usize,
    i_str: Option<usize>,
}

impl<'a> Trie<'a> {
    fn new(idx: usize, s: &'a str) -> Trie {
        Trie {
            idx,
            parent: None,
            children: FxHashMap::default(),
            s,
            sum: 0,
            i_str: None,
        }
    }

    fn split(&mut self, buf: &mut [Trie<'a>], next_idx: &mut usize, pos: usize) {
        assert!(pos < self.s.len());

        let t = self.s;
        self.s = &self.s[..pos];
        let children = take(&mut self.children);

        let c = t[pos..].chars().next().unwrap();

        let cidx = *self
            .children
            .entry(c)
            .or_insert_with(|| replace(next_idx, *next_idx + 1));

        let mut child = Trie::new(cidx, &t[pos + 1..]);
        child.parent = Some((c, self.idx));
        child.children = children;
        child.sum = self.sum;
        child.i_str = self.i_str;
        child.s = &t[pos + 1..];
        child.children.values().for_each(|&cidx2| {
            buf[cidx2].parent.as_mut().unwrap().1 = cidx;
        });
        buf[cidx] = child;

        self.i_str = None;
    }

    fn add_child(&mut self, buf: &mut [Trie<'a>], next_idx: &mut usize, s: &'a str, i_str: usize) {
        assert!(!s.is_empty());

        let c = s.chars().next().unwrap();
        assert!(!self.children.contains_key(&c));

        let cidx = *self
            .children
            .entry(c)
            .or_insert_with(|| replace(next_idx, *next_idx + 1));

        let mut child = Trie::new(cidx, &s[1..]);
        child.parent = Some((c, self.idx));
        child.sum += 1;
        child.i_str = Some(i_str);
        buf[cidx] = child;
    }

    fn insert(&mut self, buf: &mut [Trie<'a>], next_idx: &mut usize, s: &'a str, i_str: usize) {
        if let Some(pos) = izip!(s.chars(), self.s.chars()).position(|(c0, c1)| c0 != c1) {
            self.split(buf, next_idx, pos);
            self.add_child(buf, next_idx, &s[pos..], i_str);
        } else if s.len() < self.s.len() {
            self.split(buf, next_idx, s.len());

            self.i_str = Some(i_str);
        } else if s.len() == self.s.len() {
            self.i_str = Some(i_str);
        } else {
            let pos = self.s.len();

            let c = s[pos..].chars().next().unwrap();
            if let Some(&cidx) = self.children.get(&c) {
                let mut child = replace(&mut buf[cidx], Trie::new(cidx, ""));
                child.insert(buf, next_idx, &s[pos + 1..], i_str);
                buf[cidx] = child;
            } else {
                self.add_child(buf, next_idx, &s[pos..], i_str);
            }
        }

        self.sum += 1;
    }
}

fn main() {
    let n: usize = read();
    let s = read_vec(n, || read::<String>());

    let q: usize = read();
    let query = read_vec(q, || read_tuple!(usize, String));

    let mut buf = vec![Trie::new(0, ""); 400010];
    let mut root = Trie::new(0, "");
    let mut next_idx = 1;
    for (i, ss) in s.iter().enumerate() {
        root.insert(&mut buf, &mut next_idx, ss.as_str(), i);
    }
    buf[0] = root;

    // for i in 0..next_idx {
    //     eprintln!("{} {:?}", i, buf[i]);
    // }

    let idxs = buf[..next_idx]
        .iter()
        .enumerate()
        .filter_map(|(i, t)| t.i_str.map(|i_str| (i, i_str)))
        .fold(vec![0; n], |mut idxs, (idx, i_str)| {
            idxs[i_str] = idx;
            idxs
        });
    for (k, p) in query {
        let idx = idxs[k - 1];

        let q = p.chars().enumerate().fold(vec![0; 26], |mut q, (i, c)| {
            q[c as usize - 'a' as usize] = i;
            q
        });

        let ans = successors(Some(('#', idx)), |&(_, idx)| buf[idx].parent)
            .skip(1)
            .map(|(c, pidx)| {
                let q0 = q[c as usize - 'a' as usize];

                buf[pidx].i_str.is_some() as usize
                    + buf[pidx]
                        .children
                        .iter()
                        .filter(|&(d, _)| q[*d as usize - 'a' as usize] < q0)
                        .map(|(_, cidx)| buf[*cidx].sum)
                        .sum::<usize>()
            })
            .sum::<usize>();
        println!("{}", ans + 1);
    }
}
