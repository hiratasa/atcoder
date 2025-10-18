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

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::source::{Readable, Source};

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

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

enum Digits {}

impl Readable for Digits {
    type Output = Vec<usize>;
    fn read<R: std::io::BufRead, S: Source<R>>(source: &mut S) -> Vec<usize> {
        source
            .next_token_unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

fn bfs(maze: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<usize>> {
    let h = maze.len();
    let w = maze[0].len();

    let mut dists = vec![vec![usize::MAX; w]; h];
    let mut q = VecDeque::new();

    dists[start.0][start.1] = 0;
    q.push_back(start);
    while let Some((i, j)) = q.pop_front() {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
            .filter(|&(ni, nj)| ni < h && nj < w)
            .for_each(|(ni, nj)| {
                if dists[i][j] + 1 < dists[ni][nj] {
                    dists[ni][nj] = dists[i][j] + 1;

                    if maze[ni][nj] != '#' {
                        q.push_back((ni, nj));
                    }
                }
            });
    }

    dists
}

fn main() {
    input! {
        h: usize, w: usize, lines: [Chars; h]
    }

    let start = iproduct!(0..h, 0..w)
        .find(|&(i, j)| lines[i][j] == 'S')
        .unwrap();
    let goal = iproduct!(0..h, 0..w)
        .find(|&(i, j)| lines[i][j] == 'G')
        .unwrap();

    let dists_from_start = bfs(&lines, start);
    let dists_from_goal = bfs(&lines, goal);

    let ans = izip!(
        dists_from_start.iter().flatten().copied(),
        dists_from_goal.iter().flatten().copied(),
    )
    .filter_map(|(d0, d1)| d0.checked_add(d1))
    .filter(|&d| d < usize::MAX)
    .max()
    .unwrap();

    println!("{ans}");
}
