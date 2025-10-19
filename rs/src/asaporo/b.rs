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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    let n = read::<usize>();
    let a = read_row::<usize>();
    let s = read_str();

    let mut b = a
        .citer()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, 0, it.count() as i64, None, None, true))
        .collect::<Vec<_>>();

    let m = b.len();

    if m == 1 {
        println!("{}", a[0]);
        return;
    }

    let mut maxs = BinaryHeap::new();
    let mut mins = BinaryHeap::new();
    for i in 0..m {
        if i > 0 {
            b[i].3 = Some(i - 1);
        }
        if i < m - 1 {
            b[i].4 = Some(i + 1);
        }
        if i > 0 && i < m - 1 && b[i - 1].0 < b[i].0 && b[i].0 > b[i + 1].0 {
            maxs.push((Reverse(b[i].2), i));
            b[i].1 = 1;
        } else if i > 0 && i < m - 1 && b[i - 1].0 > b[i].0 && b[i].0 < b[i + 1].0 {
            mins.push((Reverse(b[i].2), i));
            b[i].1 = -1;
        }
    }

    let mut f = 0;
    let mut first = 0;
    let mut last = m - 1;

    let prev = |b: &[(usize, i64, i64, Option<usize>, Option<usize>, bool)], idx: usize| {
        b[idx].3.map(|next_idx| b[next_idx])
    };
    let next = |b: &[(usize, i64, i64, Option<usize>, Option<usize>, bool)], idx: usize| {
        b[idx].4.map(|next_idx| b[next_idx])
    };

    let fix =
        |b: &mut [(usize, i64, i64, Option<usize>, Option<usize>, bool)], idx: usize, f: i64| {
            b[idx].2 += f * b[idx].1;
            b[idx].1 = 0;

            let prev = prev(b, idx)?;
            let next = next(b, idx)?;

            let x = b[idx].0;

            if prev.0 <= x && x > next.0 {
                b[idx].1 = 1;
                b[idx].2 -= f;
            } else if prev.0 > x && x <= next.0 {
                b[idx].1 = -1;
                b[idx].2 += f;
            }

            Some(b[idx].1)
        };

    for c in s {
        let mut to_fix = vec![];

        if c == 'M' {
            f += 1;

            if b[first].0 <= next(&b, first).unwrap().0 {
                b[first].2 -= 1;
            }

            if b[last].0 < prev(&b, last).unwrap().0 {
                b[last].2 -= 1;
            }

            if b[first].2 == 0 {
                b[first].5 = false;
                first = b[first].4.unwrap();
                b[first].3 = None;
                to_fix.push(first);
            }

            if b[last].2 == 0 {
                b[first].5 = false;
                last = b[last].3.unwrap();
                b[last].4 = None;
                to_fix.push(last);
            }

            while matches!(
                mins.peek(),
                Some(&(Reverse(x), i))
                if
                b[i].2 != x || b[i].1 != -1 || !b[i].5 || x - f <= 0
            ) {
                let (Reverse(x), i) = mins.pop().unwrap();

                if !(b[i].2 == x && b[i].1 == -1 && b[i].5) {
                    continue;
                }

                assert!(x - f == 0);

                b[i].5 = false;

                let left = b[i].3.unwrap();
                let right = b[i].4.unwrap();

                b[left].4 = Some(right);
                b[right].3 = Some(left);

                to_fix.push(left);
                to_fix.push(right);
            }
        } else {
            f -= 1;

            if b[first].0 > next(&b, first).unwrap().0 {
                b[first].2 -= 1;
            }

            if b[last].0 >= prev(&b, last).unwrap().0 {
                b[last].2 -= 1;
            }

            if b[first].2 == 0 {
                b[first].5 = false;
                first = b[first].4.unwrap();
                b[first].3 = None;
                to_fix.push(first);
            }

            if b[last].2 == 0 {
                b[last].5 = false;
                last = b[last].3.unwrap();
                b[last].4 = None;
                to_fix.push(last);
            }

            while matches!(
                maxs.peek(),
                Some(&(Reverse(x), i))
                if
                b[i].2 != x || b[i].1 != 1 || !b[i].5 || x + f <= 0
            ) {
                let (Reverse(x), i) = maxs.pop().unwrap();

                if !(b[i].2 == x && b[i].1 == 1 && b[i].5) {
                    continue;
                }

                assert!(x + f == 0);

                b[i].5 = false;

                let left = b[i].3.unwrap();
                let right = b[i].4.unwrap();

                b[left].4 = Some(right);
                b[right].3 = Some(left);

                to_fix.push(left);
                to_fix.push(right);
            }
        }

        for idx in to_fix {
            match fix(&mut b, idx, f) {
                Some(-1) => mins.push((Reverse(b[idx].2), idx)),
                Some(1) => maxs.push((Reverse(b[idx].2), idx)),
                _ => {}
            }
        }

        if first == last {
            break;
        }
    }

    println!("{}", b[first].0);
}
