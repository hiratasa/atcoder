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

#[allow(dead_code)]
struct BipartiteMatching {
    n: usize,
    m: usize,
    adjs: Vec<Vec<u8>>,
    pair: Vec<Option<u8>>,
    pair2: Vec<Option<u16>>,
}

// 問題特化版
#[allow(dead_code)]
impl BipartiteMatching {
    fn new(n: usize, m: usize, adjs: Vec<Vec<u8>>) -> BipartiteMatching {
        BipartiteMatching {
            n,
            m,
            adjs: adjs,
            pair: vec![None; n],
            pair2: vec![None; m],
        }
    }

    fn bfs(&self, left: usize, right: usize) -> Option<(Vec<u8>, Vec<u8>)> {
        let mut q = std::collections::VecDeque::new();
        let mut costs = vec![std::u8::MAX; right - left];
        let mut costs2 = vec![std::u8::MAX; self.m];

        for i in left..right {
            if self.pair[i].is_none() {
                q.push_back(i as u16);
                costs[i - left] = 0;
            }
        }

        let mut found = false;
        while let Some(v) = q.pop_front() {
            let c = costs[v as usize - left];
            self.adjs[v as usize].citer().for_each(|u| {
                if costs2[u as usize] <= c {
                    return;
                }

                costs2[u as usize] = c;

                if let Some(w) = self.pair2[u as usize] {
                    assert!(w != v);

                    if costs[w as usize - left] > c + 1 {
                        costs[w as usize - left] = c + 1;
                        q.push_back(w);
                    }
                } else {
                    found = true;
                }
            });
        }

        if found { Some((costs, costs2)) } else { None }
    }

    fn dfs(
        &mut self,
        v: u16,
        levels: &[u8],
        levels2: &[u8],
        left: usize,
        right: usize,
        itrs: &mut [u16],
    ) -> bool {
        for i in (itrs[v as usize - left] as usize)..self.adjs[v as usize].len() {
            let u = self.adjs[v as usize][i];
            if levels[v as usize - left] == levels2[u as usize] {
                assert!(self.pair[v as usize] != Some(u));
                let ok = if let Some(w) = self.pair2[u as usize] {
                    if levels[w as usize - left] == levels[v as usize - left] + 1 {
                        self.dfs(w, levels, levels2, left, right, itrs)
                    } else {
                        false
                    }
                } else {
                    true
                };

                if ok {
                    self.pair[v as usize] = Some(u);
                    self.pair2[u as usize] = Some(v);
                    return true;
                }
            }
            itrs[v as usize - left] += 1;
        }

        false
    }

    fn max_flow(&mut self, left: usize, right: usize, limit: usize) -> usize {
        let mut total_flow = 0;
        loop {
            if let Some((levels, levels2)) = self.bfs(left, right) {
                let mut itrs = vec![0; right - left];
                for v in left..right {
                    if self.pair[v].is_none()
                        && self.dfs(v as u16, &levels, &levels2, left, right, &mut itrs)
                    {
                        total_flow += 1;

                        if total_flow >= limit {
                            return total_flow;
                        }
                    }
                }
            } else {
                break;
            }
        }

        total_flow
    }
}

fn main() {
    let n = read::<usize>();
    let s = read_vec(n, || read_str());

    let l = s.iter().map(|ss| ss.len()).min().unwrap();

    for c in b'a'..=b'z' {
        let c = c as char;

        let idxs = s
            .iter()
            .enumerate()
            .fold(vec![vec![]; l + n], |mut idxs, (idx, ss)| {
                ss.citer()
                    .take(l + n)
                    .positions(|cc| cc == c)
                    .for_each(|pos| {
                        idxs[pos].push(idx as u8);
                    });
                idxs
            });

        let mut g = BipartiteMatching::new(l + n, n, idxs);

        let mut f = 0;
        for pos in 0..l {
            let f0 = f;
            if pos > 0 {
                if let Some(v) = g.pair[pos - 1] {
                    g.pair[pos - 1] = None;
                    g.pair2[v as usize] = None;
                    f -= 1;
                }
            }

            let limit = if pos == 0 { n } else { f0 + 1 - f };
            f += g.max_flow(pos, pos + n, limit);

            if f == n {
                println!("YES");
                return;
            }
        }
    }

    println!("NO");
}
