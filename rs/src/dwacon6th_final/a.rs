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

fn solve(h: usize, w: usize) -> Option<Vec<Vec<usize>>> {
    let (h, w, transpose) = if h == 2 && (w % 7 == 0 || w % 7 == 1 || w % 7 == 6) {
        (w, h, true)
    } else if w == 2 && (h % 7 == 0 || h % 7 == 1 || h % 7 == 6) {
        (h, w, false)
    } else if h == 1 && (w % 7 == 2 || w % 7 == 0 || w % 7 == 5) {
        (w, h, true)
    } else if w == 1 && (h % 7 == 2 || h % 7 == 0 || h % 7 == 5) {
        (h, w, false)
    } else if h == 3 && w == 3 {
        return Some(vec![vec![2, 2, 5], vec![5, 5, 5], vec![2, 2, 5]]);
    } else {
        return None;
    };

    let mut ans = vec![];

    if w == 2 {
        if h % 7 == 0 {
            for _ in 0..h / 7 {
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 2]);
                ans.push(vec![2, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![2, 2]);
            }
        } else if h % 7 == 1 {
            ans.push(vec![2, 2]);
            for _ in 0..h / 7 {
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 2]);
                ans.push(vec![2, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![2, 2]);
            }
        } else if h % 7 == 6 {
            ans.push(vec![5, 5]);
            ans.push(vec![5, 5]);
            ans.push(vec![5, 2]);
            ans.push(vec![2, 5]);
            ans.push(vec![5, 5]);
            ans.push(vec![5, 5]);
            for _ in 0..h / 7 {
                ans.push(vec![2, 2]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 2]);
                ans.push(vec![2, 5]);
                ans.push(vec![5, 5]);
                ans.push(vec![5, 5]);
            }
        }
    } else if w == 1 {
        if h % 7 == 0 {
            for _ in 0..h / 7 {
                for _ in 0..2 {
                    ans.push(vec![2]);
                }
                for _ in 0..5 {
                    ans.push(vec![5]);
                }
            }
        } else if h % 7 == 2 {
            for _ in 0..2 {
                ans.push(vec![2]);
            }
            for _ in 0..h / 7 {
                for _ in 0..5 {
                    ans.push(vec![5]);
                }
                for _ in 0..2 {
                    ans.push(vec![2]);
                }
            }
        } else if h % 7 == 5 {
            for _ in 0..5 {
                ans.push(vec![5]);
            }
            for _ in 0..h / 7 {
                for _ in 0..2 {
                    ans.push(vec![2]);
                }
                for _ in 0..5 {
                    ans.push(vec![5]);
                }
            }
        }
    }

    if transpose {
        ans = (0..w)
            .map(|i| (0..h).map(|j| ans[j][i]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    Some(ans)
}

// fn check(h: usize, w: usize, ans: &[Vec<usize>]) -> bool {
//     if h != ans.len() {
//         return false;
//     }
//     if w != ans[0].len() {
//         return false;
//     }
//     for i in 0..h {
//         for j in 0..w {
//             if ans[i][j] != 2 && ans[i][j] != 5 {
//                 return false;
//             }
//         }
//     }

//     #[derive(Clone, Copy, Debug)]
//     enum UnionFindNode {
//         Root { size: usize },
//         Child { parent: usize },
//     }
//     struct UnionFind {
//         g: Vec<UnionFindNode>,
//     }
//     #[allow(dead_code)]
//     impl UnionFind {
//         fn new(n: usize) -> UnionFind {
//             use UnionFindNode::*;
//             UnionFind {
//                 g: (0..n).map(|_| Root { size: 1 }).collect(),
//             }
//         }
//         fn root(&mut self, v: usize) -> usize {
//             use UnionFindNode::*;
//             let p = match self.g[v] {
//                 Root { size: _ } => return v,
//                 Child { parent: p } => p,
//             };
//             let r = self.root(p);
//             self.g[v] = Child { parent: r };
//             r
//         }
//         fn unite(&mut self, v: usize, u: usize) -> bool {
//             use UnionFindNode::*;
//             let rv = self.root(v);
//             let ru = self.root(u);
//             if rv == ru {
//                 return false;
//             }
//             let size_rv = self.size(rv);
//             let size_ru = self.size(ru);
//             let (rsmall, rlarge) = if size_rv < size_ru {
//                 (rv, ru)
//             } else {
//                 (ru, rv)
//             };
//             self.g[rsmall] = Child { parent: rlarge };
//             self.g[rlarge] = Root {
//                 size: size_rv + size_ru,
//             };
//             true
//         }
//         fn same(&mut self, v: usize, u: usize) -> bool {
//             self.root(v) == self.root(u)
//         }
//         fn size(&mut self, v: usize) -> usize {
//             use UnionFindNode::*;
//             let rv = self.root(v);
//             match self.g[rv] {
//                 Root { size } => size,
//                 Child { parent: _ } => unreachable!(),
//             }
//         }
//     }

//     let mut uf = iproduct!(0..h, 0..w).fold(UnionFind::new(h * w), |mut uf, (i, j)| {
//         let x = ans[i][j];

//         if x == 2 {
//             iproduct!(it![usize::MAX, 0, 1], it![usize::MAX, 0, 1])
//                 .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
//                 .filter(|&(ni, nj)| ni < h && nj < w)
//                 .filter(|&(ni, nj)| ans[ni][nj] == ans[i][j])
//                 .for_each(|(ni, nj)| {
//                     uf.unite(i * w + j, ni * w + nj);
//                 });
//         } else {
//             it![(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)]
//                 .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
//                 .filter(|&(ni, nj)| ni < h && nj < w)
//                 .filter(|&(ni, nj)| ans[ni][nj] == ans[i][j])
//                 .for_each(|(ni, nj)| {
//                     uf.unite(i * w + j, ni * w + nj);
//                 });
//         }

//         uf
//     });

//     for i in 0..h {
//         for j in 0..w {
//             if uf.root(i * w + j) == i * w + j {
//                 let x = ans[i][j];
//                 let s = uf.size(i * w + j);

//                 if s != x {
//                     return false;
//                 }
//             }
//         }
//     }

//     true
// }

// fn solve0(h: usize, w: usize) -> Option<Vec<Vec<usize>>> {
//     let n = h * w;
//     (0..1 << n)
//         .map(|s| {
//             (0..h)
//                 .map(|i| {
//                     (0..w)
//                         .map(|j| if s & (1 << (i * w + j)) > 0 { 2 } else { 5 })
//                         .collect::<Vec<_>>()
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .find(|ans| check(h, w, ans))
// }

fn main() {
    let (h, w) = read_tuple!(usize, usize);

    let ans = solve(h, w);

    if let Some(ans) = ans {
        // assert!(check(h, w, &ans), "{} {} {:?}", h, w, ans);
        println!("Yes");

        for row in ans {
            println!("{}", row.citer().format(""));
        }
    } else {
        println!("No");
    }
}
