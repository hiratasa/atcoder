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
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
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

#[allow(dead_code)]
fn solve0(s: &[char], odd: bool) -> usize {
    let n = s.len();

    if n < 2 {
        return 0;
    }

    let mut ret = 0;
    for i in 0..n - 2 {
        if &s[i..i + 3] == &['A', 'R', 'C'] {
            let t = if odd {
                s[..i]
                    .citer()
                    .chain(once('R'))
                    .chain(s[i + 3..].citer())
                    .collect::<Vec<_>>()
            } else {
                s[..i]
                    .citer()
                    .chain(once('A'))
                    .chain(once('C'))
                    .chain(s[i + 3..].citer())
                    .collect::<Vec<_>>()
            };

            ret = max(ret, 1 + solve0(&t, !odd));
        }
    }

    ret
}

fn main() {
    let n = read::<usize>();
    let s = read_str();

    let mut q = s
        .citer()
        .map(|c| match c {
            'A' => (1, 0, 0),
            'R' => (0, 1, 0),
            'C' => (0, 0, 1),
            _ => unreachable!(),
        })
        .coalesce(|x, y| {
            if x.0 > 0 && x.1 == 0 {
                assert!(x.2 == 0);
                if y.0 == 1 {
                    Ok((x.0 + 1, 0, 0))
                } else if y.1 == 1 {
                    Ok((x.0, 1, 0))
                } else {
                    Ok((0, 0, 0))
                }
            } else if x.0 > 0 && x.1 > 0 && x.2 == 0 {
                assert!(x.1 == 1);
                if y.0 == 1 {
                    Ok(y)
                } else if y.1 == 1 {
                    Ok((0, 0, 0))
                } else {
                    Ok((x.0, x.1, 1))
                }
            } else if x.0 > 0 && x.1 > 0 && x.2 > 0 {
                assert!(x.1 == 1);
                if y.0 == 1 {
                    Err((x, y))
                } else if y.1 == 1 {
                    Err((x, (0, 0, 0)))
                } else {
                    Ok((x.0, x.1, x.2 + 1))
                }
            } else {
                Ok(y)
            }
        })
        // .inspect(|x| eprintln!("#{:?}", x))
        .filter(|&x| x.1 == 1 && x.0 > 0 && x.2 > 0)
        .map(|(x, _y, z)| Reverse(min(x, z)))
        .collect::<BinaryHeap<_>>();

    let mut m1 = 0;
    while matches!(q.peek(), Some(Reverse(1))) {
        m1 += 1;
        q.pop();
    }

    let mut ans = 0;
    loop {
        // odd turn
        if let Some(Reverse(x)) = q.pop() {
            assert!(x >= 2);
            ans += 1;
            if x - 1 == 1 {
                m1 += 1;
            } else {
                q.push(Reverse(x - 1));
            }
        } else if m1 > 0 {
            m1 -= 1;
            ans += 1;
        } else {
            break;
        }

        // even turn
        if m1 > 0 {
            m1 -= 1;
            ans += 1;
        } else if let Some(Reverse(_)) = q.pop() {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
