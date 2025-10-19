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

fn main() {
    let (n, k) = read_tuple!(usize, usize);

    let a = read_row::<usize>();

    let mut b = once(0).chain(a.citer()).chain(once(0)).enumerate().fold(
        BTreeMap::new(),
        |mut map, (i, aa)| {
            if !matches!(map.values().next_back(), Some(&last) if last == aa) {
                map.insert(i, aa);
            }
            map
        },
    );
    let mut q = b
        .iter()
        .tuple_windows()
        .filter(|(prev, current, next)| prev.1 < current.1 && current.1 > next.1)
        .map(|(_prev, current, next)| (Reverse(next.0 - current.0), *current.0))
        .collect::<BinaryHeap<_>>();

    let c = b
        .iter()
        .tuple_windows()
        .map(|(prev, current, next)| {
            let len = next.0 - current.0;

            let left = if prev.1 < current.1 {
                current.1 - prev.1
            } else {
                0
            };
            let right = if next.1 < current.1 {
                current.1 - next.1
            } else {
                0
            };

            left + right + len + 2 * len * current.1
        })
        .sum::<usize>();
    let mut ans = c;
    let mut k = k;
    while let Some((Reverse(len), idx)) = q.pop() {
        let (&prev_idx, &prev) = b.range(..idx).next_back().unwrap();
        let current = b[&idx];
        let (&next_idx, &next) = b.range(idx + 1..).next().unwrap();

        assert!(len == next_idx - idx);
        let h = max(prev, next);
        let r = (current - h) * len;

        if r < k {
            k -= r;
            ans -= (current - h) * (2 * len + 2);

            b.remove(&idx);

            if prev < next {
                b.remove(&next_idx);
                b.insert(idx, h);
                let (&next_idx2, &next2) = b.range(idx + 1..).next().unwrap();
                if h > next2 {
                    q.push((Reverse(next_idx2 - idx), idx));
                }
            } else if prev == next {
                b.remove(&next_idx);
                let (&_prev_idx2, &prev2) = b.range(..prev_idx).next_back().unwrap();
                let (&next_idx2, &next2) = b.range(prev_idx + 1..).next().unwrap();
                if prev2 < h && h > next2 {
                    q.push((Reverse(next_idx2 - prev_idx), prev_idx));
                }
            } else {
                let (&_prev_idx2, &prev2) = b.range(..prev_idx).next_back().unwrap();
                if prev2 < h {
                    q.push((Reverse(next_idx - prev_idx), prev_idx));
                }
            }
        } else {
            let y = k / len;
            ans -= y * (2 * len + 2);

            ans -= (k % len) * 2;
            break;
        }
    }

    println!("{}", ans);
}
