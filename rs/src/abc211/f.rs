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

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

//  1-indexedで、
//    iの最後に立っているビットをB（=i&-i)として、
//    values_iは [i - (i&-i) + 1, i] の区間の和を保持
//    (といいつつ0-indexにアクセスする箇所で直してる)
// M must be commutative.
struct BIT<M>
where
    M: Monoid,
{
    len: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Monoid,
{
    fn new(len: usize) -> BIT<M> {
        BIT {
            len,
            values: vec![M::id(); len],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let mut bit = Self::new(vals.len());

        for (i, v) in vals.iter().enumerate() {
            bit.add(i, v.clone());
        }

        bit
    }

    // [0, i)の和
    fn sum(&self, i: usize) -> M::Item {
        let mut s = M::id();
        let mut idx = i as i64;

        // values[1] ~ values[i] の和
        // (bは1-indexedなのでこれでOK)
        while idx > 0 {
            s = M::op(&s, &self.values[(idx - 1) as usize]);
            idx -= idx & -idx;
        }

        return s;
    }

    fn add(&mut self, i: usize, a: M::Item) {
        // 1-indexedに直す
        let mut idx = i as i64 + 1;

        while idx as usize <= self.len {
            self.values[(idx - 1) as usize] = M::op(&self.values[(idx - 1) as usize], &a);
            idx += idx & -idx;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

// 可換群
trait Group: Monoid {
    fn inv(value: &Self::Item) -> Self::Item;
}

#[allow(dead_code)]
impl<M> BIT<M>
where
    M: Group,
{
    // [i, j) の和
    fn sum_between(&self, i: usize, j: usize) -> M::Item {
        M::op(&self.sum(j), &M::inv(&self.sum(i)))
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        struct $name;

        impl Monoid for $name {
            type Item = $t;

            fn id() -> Self::Item {
                $id
            }

            fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
                ($op)(*lhs, *rhs)
            }
        }
    };
}

macro_rules! define_group {
    ($name: ident, $t: ty, $id: expr, $op: expr, $inv: expr) => {
        define_monoid!($name, $t, $id, $op);

        impl Group for $name {
            fn inv(value: &Self::Item) -> Self::Item {
                ($inv)(*value)
            }
        }
    };
}

define_group!(Sum, i64, 0, std::ops::Add::add, std::ops::Neg::neg);

fn main() {
    let n: usize = read();
    let rects = (0..n)
        .map(|_| {
            let m: usize = read();
            let xy = read_row::<usize>()
                .into_iter()
                .tuples()
                .collect::<Vec<(_, _)>>();

            xy
        })
        .collect::<Vec<_>>();
    let q: usize = read();
    let xy = read_vec(q, || read_tuple!(usize, usize));

    let rect_points = rects
        .iter()
        .map(|points| {
            points
                .citer()
                .sorted()
                .group_by(|t| t.0)
                .into_iter()
                .map(|(x, it)| (x, it.map(|t| t.1).collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let t = rect_points
        .iter()
        .map(|points| {
            let yss = points
                .iter()
                .flat_map(|(_x, ys)| ys.citer())
                .sorted()
                .dedup()
                .collect::<Vec<_>>();
            let y_idxs = yss
                .citer()
                .enumerate()
                .map(|t| (t.1, t.0))
                .collect::<FxHashMap<_, _>>();
            points
                .iter()
                .scan(BIT::<Sum>::new(yss.len() + 1), |bit, (_x, ys)| {
                    let t = ys
                        .citer()
                        .map(|y| y_idxs[&y])
                        .enumerate()
                        .map(|(i, y_idx)| {
                            let c0 = bit.sum(y_idx);
                            let c1 = bit.sum(y_idx + 1);

                            if i % 2 == 0 {
                                if c0 == 0 && c1 == 0 {
                                    bit.add(y_idx, 1);
                                    1
                                } else if c0 == 0 && c1 == 1 {
                                    bit.add(y_idx, -1);
                                    -1
                                } else if c0 == 1 && c1 == 0 {
                                    bit.add(y_idx, 1);
                                    1
                                } else if c0 == 1 && c1 == 1 {
                                    bit.add(y_idx, -1);
                                    -1
                                } else {
                                    unreachable!()
                                }
                            } else {
                                if c0 == 0 && c1 == -1 {
                                    bit.add(y_idx, 1);
                                    1
                                } else if c0 == 0 && c1 == 0 {
                                    bit.add(y_idx, 1);
                                    1
                                } else if c0 == 1 && c1 == 1 {
                                    bit.add(y_idx, -1);
                                    -1
                                } else if c0 == 1 && c1 == 2 {
                                    bit.add(y_idx, -1);
                                    -1
                                } else {
                                    unreachable!()
                                }
                            }
                        })
                        .collect::<Vec<_>>();
                    Some(t)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let all_points = izip!(rect_points, t,)
        .flat_map(|(points, tt)| {
            izip!(points, tt).map(|((x, ys), ts)| (x, izip!(ys, ts).collect::<Vec<_>>()))
        })
        .sorted_by_key(|(x, _)| *x)
        .group_by(|(x, _)| *x)
        .into_iter()
        .map(|(x, it)| (x, it.flat_map(|(_, v)| v).sorted().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    // eprintln!("{:?}", all_points);

    let queries = xy
        .citer()
        .enumerate()
        .sorted_by_key(|&(_i, (x, _y))| x)
        .group_by(|&(_i, (x, _y))| x)
        .into_iter()
        .map(|(x, it)| (x, it.collect::<Vec<_>>()))
        .collect::<BTreeMap<_, _>>();
    let (_, ans, _) = all_points.iter().fold(
        (BIT::<Sum>::new(100010), vec![0; q], 0),
        |(mut bit, mut ans, prev_x), (x, points)| {
            let x = *x;

            queries.range(prev_x..x).for_each(|(_xx, v)| {
                v.citer().for_each(|(i_query, (_x, y))| {
                    ans[i_query] = bit.sum(y + 1);
                });
            });

            points.citer().for_each(|(y, z)| {
                bit.add(y, z);
            });

            (bit, ans, x)
        },
    );

    for a in ans {
        println!("{}", a);
    }
}
