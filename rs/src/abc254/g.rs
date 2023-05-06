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

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    cap: usize,
    values: Vec<M::Item>,
}

#[allow(dead_code)]
impl<M> SegmentTree<M>
where
    M: Monoid,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTree {
            cap,
            values: vec![M::id(); 2 * cap - 1],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let n = vals.len();
        let cap = n.next_power_of_two();

        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned());
        values.resize(2 * cap - 1, M::id());

        let mut st = SegmentTree { cap, values };
        for idx in (0..cap - 1).rev() {
            st.fix_value(idx);
        }
        st
    }

    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
    }

    fn get(&self, pos: usize) -> M::Item {
        self.values[self.cap - 1 + pos].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        let mut idx = self.cap - 1 + pos;

        self.values[idx] = v;

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn query(&self, a: usize, b: usize) -> M::Item {
        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        let c0 = std::cmp::min(
            // trailing_ones
            (!left_idx).trailing_zeros(),
            (right_idx + 1).trailing_zeros(),
        );
        left_idx = left_idx >> c0;
        right_idx = ((right_idx + 1) >> c0) - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }

            let c = std::cmp::min(
                // trailing_ones
                (!left_idx).trailing_zeros(),
                (right_idx + 1).trailing_zeros(),
            );
            left_idx = left_idx >> c;
            right_idx = ((right_idx + 1) >> c) - 1;
        }

        M::op(&left, &right)
    }

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
    // (存在しないときにnを返してしまうとquery(a,n)がfalseのときと区別がつかないのでNoneを返す)
    fn right_partition_point<F>(&self, a: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(a <= self.cap);
        if !f(&M::id()) {
            Some(a)
        } else if a == self.cap {
            None
        } else {
            let mut b = a;
            // [b, b+2^k) が保持されている最初の箇所に移動
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);

            // チェックする範囲を拡大しながらf()がtrueになる限りbを右に伸ばしていく
            while f(&val_next) {
                val = val_next;
                b += len;

                // [b, b+2^k) が保持されている最初の箇所に移動
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;

                // 最後に計算したidxが右端だった場合
                if idx == 0 {
                    return None;
                }
                val_next = M::op(&val, &self.values[idx]);
            }

            // 範囲を縮小しながらbを右に伸ばしていく
            idx = 2 * idx + 1;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&val, &self.values[idx]);
                if f(&val_next) {
                    val = val_next;
                    b += len;
                    idx += 1;
                }
                len >>= 1;
                idx = 2 * idx + 1;
            }

            // [a, b)区間でfがtrue => 求めるbはその次
            Some(b + 1)
        }
    }

    // f(query(a, b)) == false となるaが存在すればその最大のもの+1を返す
    // 存在しない場合は0を返す
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> usize
    where
        F: FnMut(&M::Item) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            b
        } else if b == 0 {
            0
        } else {
            let mut a = b;
            // [a-2^k, a) が保持されている最初の箇所に移動
            let mut idx = (a + self.cap - 1) >> (!(a + self.cap - 1)).trailing_zeros();
            let mut len = 1 << (!(a + self.cap - 1)).trailing_zeros();
            if idx == 0 {
                // このケースになるのはb=self.capのときだけ
                len = self.cap;
            } else {
                idx -= 1;
            }

            let mut val = M::id();
            let mut val_next = M::op(&self.values[idx], &val);

            // チェックする範囲を拡大しながらf()がtrueになる限りaを左に伸ばしていく
            while f(&val_next) {
                val = val_next;
                a -= len;

                // 最後に計算したidxが左端だった場合
                if idx == 0 || (idx + 1).is_power_of_two() {
                    return 0;
                }

                // [a-2^k, a) が保持されている最初の箇所に移動
                len <<= (!idx).trailing_zeros();
                idx >>= (!idx).trailing_zeros();
                idx -= 1;

                val_next = M::op(&self.values[idx], &val);
            }

            // 範囲を縮小しながらaを左に伸ばしていく
            idx = 2 * idx + 2;
            len >>= 1;
            while idx < self.values.len() {
                val_next = M::op(&self.values[idx], &val);
                if f(&val_next) {
                    val = val_next;
                    a -= len;
                    idx -= 1;
                }
                len >>= 1;
                idx = 2 * idx + 2;
            }

            a
        }
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone, Debug)]
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

define_monoid!(Max, (usize, usize), (0, 0), std::cmp::max);

fn solve0(
    n: usize,
    abc: &[(usize, usize, usize)],
    x: usize,
    y: usize,
    z: usize,
    w: usize,
) -> Option<usize> {
    let mut costs = FxHashMap::default();
    let mut q = VecDeque::new();

    costs.insert((x - 1, y), 0);
    q.push_back((0, (x - 1, y)));

    while let Some((cost, (k, f))) = q.pop_front() {
        if (k, f) == (z - 1, w) {
            return Some(cost);
        }

        for i in 0..n {
            let nc = cost + 1;
            if nc < costs.get(&(i, f)).copied().unwrap_or(usize::MAX) {
                costs.insert((i, f), nc);
                q.push_back((nc, (i, f)));
            }
        }

        abc.citer()
            .filter(|&(a, b, c)| a - 1 == k && b <= f && f <= c)
            .for_each(|(_, b, c)| {
                it![f - 1, f + 1]
                    .filter(|&ff| b <= ff && ff <= c)
                    .for_each(|ff| {
                        let nc = cost + 1;
                        if nc < costs.get(&(k, ff)).copied().unwrap_or(usize::MAX) {
                            costs.insert((k, ff), nc);
                            q.push_back((nc, (k, ff)));
                        }
                    });
            });
    }

    None
}

fn main() {
    let (n, m, q) = read_tuple!(usize, usize, usize);
    let abc = read_vec(m, || read_tuple!(usize, usize, usize));
    let query = read_vec(q, || read_tuple!(usize, usize, usize, usize));

    // merge
    let (elevators, _) = abc
        .citer()
        .sorted()
        .group_by(|(a, _, _)| *a)
        .into_iter()
        .map(|(a, it)| {
            (
                a,
                it.map(|(_, b, c)| (b, c))
                    .coalesce(|(b0, c0), (b1, c1)| {
                        assert!(b0 <= b1);

                        if c0 >= b1 {
                            Ok((b0, max(c0, c1)))
                        } else {
                            Err(((b0, c0), (b1, c1)))
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .fold((vec![vec![]; n], 0), |(mut elevators, idx), (a, v)| {
            let l = v.len();
            elevators[a - 1] = izip!(idx.., v).collect::<Vec<_>>();
            (elevators, idx + l)
        });

    let elevators_flat = elevators.iter().flatten().copied().collect::<Vec<_>>();
    let ys = elevators
        .iter()
        .flat_map(|row| row.citer().flat_map(|(_, (b, c))| it![b, c]))
        .chain(query.citer().flat_map(|(_, y, _, w)| it![y, w]))
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let idxs = ys
        .citer()
        .enumerate()
        .map(|(idx, y)| (y, idx))
        .collect::<FxHashMap<_, _>>();
    let k = ys.len();

    let st = elevators
        .iter()
        .fold(SegmentTree::<Max>::new(k), |st, row| {
            row.citer().fold(st, |mut st, (idx, (b, c))| {
                let tmp = st.get(idxs[&b]);
                st.set(idxs[&b], max(tmp, (idxs[&c], idx)));
                st
            })
        });

    const K: usize = 20;
    let mut nexts = vec![vec![0; m]; K];
    nexts[0] = elevators
        .iter()
        .flatten()
        .map(|&(idx, (_b, c))| {
            let (d_idx, next) = st.query(0, idxs[&c] + 1);
            let d = ys[d_idx];

            assert!(d >= c);
            (idx, next)
        })
        .fold(vec![0; m], |mut nexts, (idx, next)| {
            nexts[idx] = next;
            nexts
        });

    for i in 1..K {
        for j in 0..m {
            nexts[i][j] = nexts[i - 1][nexts[i - 1][j]];
        }
    }

    query
        .citer()
        .map(|(x, y, z, w)| {
            let x = x - 1;
            let z = z - 1;

            if y == w {
                return Some((x != z) as usize);
            }

            let (x, y, z, w) = if y > w { (z, w, x, y) } else { (x, y, z, w) };

            iproduct!(it![false, true], it![false, true])
                .filter_map(|(get_on_direct, get_off_direct)| {
                    let (start, get_on_cost) = if get_on_direct {
                        let idx = elevators[x]
                            .binary_search_by(|&(_, (b, _c))| b.cmp(&y).then(Ordering::Less))
                            .unwrap_err();
                        if idx == 0 || (elevators[x][idx - 1].1).1 <= y {
                            return None;
                        }

                        if (elevators[x][idx - 1].1).1 >= w && x == z {
                            return Some(w - y);
                        }

                        (elevators[x][idx - 1].0, 0)
                    } else {
                        let (c_idx, idx) = st.query(0, &idxs[&y] + 1);
                        let c = ys[c_idx];

                        if c <= y {
                            return None;
                        }

                        (idx, 1)
                    };

                    let goal_floor = if get_off_direct {
                        let idx = elevators[z]
                            .binary_search_by(|&(_, (_b, c))| c.cmp(&w).then(Ordering::Greater))
                            .unwrap_err();
                        if idx == elevators[z].len() || (elevators[z][idx].1).0 >= w {
                            return None;
                        }

                        max((elevators[z][idx].1).0, y)
                    } else {
                        w
                    };

                    if y >= goal_floor {
                        return Some(w - y + 1);
                    }

                    if (elevators_flat[start].1).1 >= goal_floor {
                        return Some(w - y + get_on_cost + 1);
                    }

                    let mut current = start;
                    let mut t = 0;
                    for i in (0..K).rev() {
                        let next = nexts[i][current];
                        let next_floor = (elevators_flat[next].1).1;
                        if next_floor < goal_floor {
                            current = next;
                            t += 1 << i;
                        }
                    }

                    assert!((elevators_flat[current].1).1 < goal_floor);

                    if nexts[0][current] == current {
                        // no next
                        None
                    } else {
                        Some(w - y + get_on_cost + t + 2)
                    }
                })
                .min()
        })
        .for_each(|ans| {
            if let Some(ans) = ans {
                println!("{}", ans);
            } else {
                println!("-1");
            }
        });
}
