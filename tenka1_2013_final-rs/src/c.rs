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

use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn reverse(self) -> Side {
        Self::from_usize((self as usize + 2) % 4).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Piece([u8; 4]);

impl Piece {
    fn new() -> Self {
        Self([0, 0, 0, 0])
    }

    fn from_stdin() -> Self {
        let (p0, p1, p2, p3) = read_tuple!(u8, u8, u8, u8);

        Self([p0, p1, p2, p3])
    }

    fn rotate(self, n: usize) -> Self {
        Self([
            self.0[(4 - n) % 4],
            self.0[(5 - n) % 4],
            self.0[(6 - n) % 4],
            self.0[(7 - n) % 4],
        ])
    }

    fn normalize(self) -> Self {
        (0..4).map(|i| self.rotate(i)).min().unwrap()
    }

    fn rotate_idx_to_normalize(self) -> usize {
        (0..4).map(|i| self.rotate(i)).position_min().unwrap()
    }

    fn normalized_pieces() -> impl Iterator<Item = Self> {
        [
            Piece([0, 0, 1, 1]),
            Piece([0, 0, 1, 2]),
            Piece([0, 0, 2, 1]),
            Piece([0, 0, 2, 2]),
            Piece([0, 1, 1, 1]),
            Piece([0, 1, 1, 2]),
            Piece([0, 1, 2, 1]),
            Piece([0, 2, 1, 1]),
            Piece([0, 1, 2, 2]),
            Piece([0, 2, 1, 2]),
            Piece([0, 2, 2, 1]),
            Piece([0, 2, 2, 2]),
            Piece([1, 1, 1, 1]),
            Piece([1, 1, 1, 2]),
            Piece([1, 1, 2, 2]),
            Piece([1, 2, 1, 2]),
            Piece([1, 2, 2, 2]),
            Piece([2, 2, 2, 2]),
        ]
        .citer()
    }

    fn get(self, side: Side) -> u8 {
        self.0[side as usize]
    }

    fn fit(self, other: Option<Self>, side: Side) -> bool {
        match other {
            None => self.get(side) == 0,
            Some(other) => self.get(side) + other.get(side.reverse()) == 3,
        }
    }
}

fn solve(
    state: &mut [Vec<Piece>],
    i: usize,
    j: usize,
    pieces: &mut FxHashMap<Piece, usize>,
    perfects: &mut Vec<Vec<Vec<Piece>>>,
) {
    let n = state.len();
    let m = state[0].len();

    if i == n {
        perfects.push(state.to_vec());
        return;
    }

    if j == m {
        solve(state, i + 1, 0, pieces, perfects);
        return;
    }

    for piece in Piece::normalized_pieces() {
        if pieces.get(&piece).copied().unwrap_or(0) == 0 {
            continue;
        }

        let top = i.checked_sub(1).map(|ii| state[ii][j]);
        let left = j.checked_sub(1).map(|jj| state[i][jj]);

        for k in 0..4 {
            let rpiece = piece.rotate(k);

            if (i == n - 1) != rpiece.fit(None, Side::Bottom) {
                continue;
            }

            if (j == m - 1) != rpiece.fit(None, Side::Right) {
                continue;
            }

            if rpiece.fit(top, Side::Top) && rpiece.fit(left, Side::Left) {
                state[i][j] = rpiece;
                *pieces.get_mut(&piece).unwrap() -= 1;

                solve(state, i, j + 1, pieces, perfects);

                *pieces.get_mut(&piece).unwrap() += 1;
            }
        }
    }
}

fn main() {
    let (n, m, q) = read_tuple!(usize, usize, usize);
    let pieces = read_vec(n * m, || Piece::from_stdin());

    let mut normalized_to_idxs = pieces
        .citer()
        .map(|p| {
            let ridx = p.rotate_idx_to_normalize();
            (p.rotate(ridx), ridx)
        })
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (i, (p, ridx))| {
            map.entry(p).or_insert(FxHashMap::default()).insert(i, ridx);
            map
        });

    let mut num_normalized_pieces = normalized_to_idxs
        .iter()
        .map(|(p, v)| (*p, v.len()))
        .collect::<FxHashMap<_, _>>();

    let mut states = vec![];
    solve(
        &mut vec![vec![Piece::new(); m]; n],
        0,
        0,
        &mut num_normalized_pieces,
        &mut states,
    );

    let i_corner0 = pieces
        .citer()
        .position(|piece| piece.0.citer().filter(|&x| x == 0).count() == 2)
        .unwrap();

    states.retain(|state| state[0][0].normalize() == pieces[i_corner0].normalize());

    let mut ans = vec![vec![(0, 0); m]; n];
    ans[0][0] = (
        i_corner0,
        (pieces[i_corner0].rotate_idx_to_normalize() + 3) % 4,
    );
    normalized_to_idxs
        .get_mut(&pieces[i_corner0].normalize())
        .unwrap()
        .remove(&i_corner0);

    let mut num_query = 0;
    for (i, j) in iproduct!(0..n, 0..m).skip(1) {
        let mut candidates = states
            .iter()
            .map(|state| {
                let ridx = state[i][j].rotate_idx_to_normalize();
                (state[i][j].rotate(ridx), (4 - ridx) % 4)
            })
            .sorted()
            .dedup()
            .filter_map(|(p, ridx)| normalized_to_idxs.get(&p).map(|v| (v, ridx)))
            .flat_map(|(v, ridx)| {
                v.iter()
                    .map(move |(&idx, &nridx)| (idx, (nridx + ridx) % 4))
            })
            .collect::<Vec<_>>();

        assert!(!candidates.is_empty());
        while candidates.len() > 1 {
            let (idx, ridx) = candidates.pop().unwrap();

            assert!(num_query < q);
            if j == 0 {
                let (top, topridx) = ans[i - 1][j];
                println!(
                    "? {} {} {} {}",
                    top + 1,
                    (6 - topridx) % 4 + 1,
                    idx + 1,
                    (4 - ridx) % 4 + 1
                );
            } else {
                let (left, leftridx) = ans[i][j - 1];
                println!(
                    "? {} {} {} {}",
                    left + 1,
                    (5 - leftridx) % 4 + 1,
                    idx + 1,
                    (3 - ridx) % 4 + 1
                );
            }
            num_query += 1;

            if read::<String>() == "yes" {
                candidates = vec![(idx, ridx)];
                break;
            }
        }

        ans[i][j] = candidates[0];
        normalized_to_idxs
            .get_mut(&pieces[ans[i][j].0].normalize())
            .unwrap()
            .remove(&ans[i][j].0);

        states.retain(|state| state[i][j] == pieces[ans[i][j].0].rotate(ans[i][j].1));
    }

    println!("!");
    for row in ans {
        println!(
            "{}",
            row.citer()
                .map(|(idx, ridx)| format!("{} {}", idx + 1, ridx + 1))
                .join(" ")
        );
    }
}
