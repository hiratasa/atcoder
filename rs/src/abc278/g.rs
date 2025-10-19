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
    let (n, l, r) = read_tuple!(usize, usize, usize);

    let (grundy, _, _, _) = (1..=n).fold(
        (vec![0; n + 1], FxHashMap::default(), BTreeSet::new(), 0),
        |(mut grundy, mut c, mut notexist, mut b), i| {
            if i >= r + 1 {
                (0..=i - (r + 1))
                    .map(|j| (j, i - (r + 1) - j))
                    .map(|(j, k)| grundy[j] ^ grundy[k])
                    .for_each(|g| {
                        *c.get_mut(&g).unwrap() -= 1;
                        if c[&g] == 0 {
                            c.remove(&g);
                            notexist.insert(g);
                        }
                    });
            }
            if i >= l {
                (0..=i - l)
                    .map(|j| (j, i - l - j))
                    .map(|(j, k)| grundy[j] ^ grundy[k])
                    .for_each(|g| {
                        *c.entry(g).or_insert(0) += 1;
                        if c[&g] == 1 {
                            if notexist.contains(&g) {
                                notexist.remove(&g);
                            }
                            if g >= b {
                                (b..g).for_each(|h| {
                                    notexist.insert(h);
                                });
                                b = g + 1;
                            }
                        }
                    });
            }
            grundy[i] = notexist.citer().next().unwrap_or(b);

            (grundy, c, notexist, b)
        },
    );

    let t = iproduct!(0..=n, 0..=n).filter(|&(i, j)| i + j <= n).fold(
        FxHashMap::default(),
        |mut t, (i, j)| {
            let s = i + j;
            let x = grundy[i] ^ grundy[j];
            t.insert((s, x), (i, j));
            t
        },
    );

    let next_hand = |cards: &[(usize, usize)]| {
        let x = cards
            .citer()
            .map(|(i, j)| grundy[j - i + 1])
            .fold(0, |x, y| x ^ y);
        let (i0, j0) = cards
            .citer()
            .find(|&(i, j)| grundy[j - i + 1] > grundy[j - i + 1] ^ x)
            .unwrap();

        let m = j0 - i0 + 1;

        let y = grundy[m] ^ x;

        assert!(m >= l);

        let (a, b) = (m.saturating_sub(r)..=(m - l))
            .find_map(|s| t.get(&(s, y)).copied())
            .unwrap();

        (i0 + a, m - (a + b))
    };

    let apply = |cards: &mut Vec<(usize, usize)>, (x, y): (usize, usize)| {
        let idx = cards
            .binary_search_by(|&(x1, _)| x1.cmp(&x).then(Ordering::Less))
            .unwrap_err();
        assert!(idx > 0);

        let (i, j) = cards.remove(idx - 1);

        it![(i, x - 1), (x + y, j)]
            .filter(|&(ii, jj)| ii <= jj)
            .rev()
            .for_each(|(ii, jj)| {
                cards.insert(idx - 1, (ii, jj));
            });
    };

    let mut cards = vec![(1, n)];

    if grundy[n] == 0 {
        println!("Second");
    } else {
        println!("First");

        let (x, y) = next_hand(&cards);
        apply(&mut cards, (x, y));

        println!("{} {}", x, y);
    }

    loop {
        let (a, b) = read_tuple!(usize, usize);
        if (a, b) == (0, 0) {
            // win
            return;
        }

        apply(&mut cards, (a, b));

        let (x, y) = next_hand(&cards);
        apply(&mut cards, (x, y));

        println!("{} {}", x, y);
    }
}
