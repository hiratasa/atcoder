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

fn range<T>(r: impl std::ops::RangeBounds<T>, min: T, n: T) -> (T, T)
where
    T: std::ops::Add<Output = T> + Copy + num::One,
{
    let start = match r.start_bound() {
        std::ops::Bound::Excluded(&i) => i + T::one(),
        std::ops::Bound::Included(&i) => i,
        std::ops::Bound::Unbounded => min,
    };
    let end = match r.end_bound() {
        std::ops::Bound::Excluded(&i) => i,
        std::ops::Bound::Included(&i) => i + T::one(),
        std::ops::Bound::Unbounded => n,
    };
    (start, end)
}
trait Monoid {
    fn id() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    cap: usize,
    values: Vec<M>,
}
#[allow(dead_code)]
impl<M> SegmentTree<M>
where
    M: Monoid + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTree {
            cap,
            values: vec![M::id(); 2 * cap - 1],
        }
    }
    fn with<T>(vals: &[T]) -> Self
    where
        T: Into<M> + Clone,
    {
        let n = vals.len();
        let cap = n.next_power_of_two();
        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned().map(|x| x.into()));
        values.resize(2 * cap - 1, M::id());
        let mut st = SegmentTree { cap, values };
        for idx in (0..cap - 1).rev() {
            st.fix(idx);
        }
        st
    }
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
        }
    }
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }
    fn get(&self, pos: usize) -> M {
        self.values[self.cap - 1 + pos].clone()
    }
    fn set<T>(&mut self, pos: usize, v: T)
    where
        T: Into<M>,
    {
        let idx = self.cap - 1 + pos;
        self.values[idx] = v.into();
        self.fix_all(idx);
    }
    fn query(&self, r: impl std::ops::RangeBounds<usize>) -> M {
        let (a, b) = range(r, 0, self.cap);
        let mut left = M::id();
        let mut right = M::id();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx]);
                left_idx += 1;
            }
            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1], &right);
                right_idx -= 1;
            }
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
        M::op(&left, &right)
    }
    fn right_partition_point<F>(&self, a: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M) -> bool,
    {
        assert!(a <= self.cap);
        if !f(&M::id()) {
            Some(a)
        } else if a == self.cap {
            None
        } else {
            let mut b = a;
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);
            while f(&val_next) {
                val = val_next;
                b += len;
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;
                if idx == 0 {
                    return None;
                }
                val_next = M::op(&val, &self.values[idx]);
            }
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
            Some(b + 1)
        }
    }
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> usize
    where
        F: FnMut(&M) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            b
        } else if b == 0 {
            0
        } else {
            let mut a = b;
            let mut idx = (a + self.cap - 1) >> (!(a + self.cap - 1)).trailing_zeros();
            let mut len = 1 << (!(a + self.cap - 1)).trailing_zeros();
            if idx == 0 {
                len = self.cap;
            } else {
                idx -= 1;
            }
            let mut val = M::id();
            let mut val_next = M::op(&self.values[idx], &val);
            while f(&val_next) {
                val = val_next;
                a -= len;
                if idx == 0 || (idx + 1).is_power_of_two() {
                    return 0;
                }
                len <<= (!idx).trailing_zeros();
                idx >>= (!idx).trailing_zeros();
                idx -= 1;
                val_next = M::op(&self.values[idx], &val);
            }
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
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct $name($t);
        impl Monoid for $name {
            fn id() -> Self {
                Self($id)
            }
            fn op(&self, rhs: &Self) -> Self {
                Self(($op)(self.0, rhs.0))
            }
        }
        impl From<$t> for $name {
            fn from(x: $t) -> $name {
                Self(x)
            }
        }
    };
}

// 座標圧縮 + Segment木
#[derive(Debug)]
struct SparseSegmentTree<M>
where
    M: Monoid,
{
    pos_idxs: BTreeMap<i64, usize>,
    segtree: SegmentTree<M>,
}

#[allow(dead_code)]
impl<M> SparseSegmentTree<M>
where
    M: Monoid + Clone,
{
    fn new(poss: impl IntoIterator<Item = i64>) -> SparseSegmentTree<M> {
        let mut poss = poss.into_iter().collect::<Vec<_>>();
        poss.sort();
        poss.dedup();
        let pos_idxs = poss
            .iter()
            .enumerate()
            .map(|(i, &j)| (j, i))
            .collect::<BTreeMap<_, _>>();
        let n = pos_idxs.len();

        SparseSegmentTree {
            pos_idxs,
            segtree: SegmentTree::new(n),
        }
    }

    fn get(&self, pos: i64) -> M {
        if let Some(&pos_idx) = self.pos_idxs.get(&pos) {
            self.segtree.get(pos_idx)
        } else {
            M::id()
        }
    }

    fn contains(&self, pos: i64) -> bool {
        self.pos_idxs.contains_key(&pos)
    }

    fn set<T>(&mut self, pos: i64, v: T)
    where
        T: Into<M>,
    {
        let pos_idx = *self.pos_idxs.get(&pos).unwrap();
        self.segtree.set(pos_idx, v);
    }

    fn query(&self, r: impl std::ops::RangeBounds<i64>) -> M {
        let (a, b) = range(r, std::i64::MIN, std::i64::MAX);

        let n = self.pos_idxs.len();
        let pos_idx0 = self
            .pos_idxs
            .range(a as i64..)
            .next()
            .map_or(n, |(_, &pos_idx)| pos_idx);
        let pos_idx1 = self
            .pos_idxs
            .range(b as i64..)
            .next()
            .map_or(n, |(_, &pos_idx)| pos_idx);

        self.segtree.query(pos_idx0..pos_idx1)
    }
}

// 二次元Segment木
// あらかじめ指定されたN個の要素だけ値を持つ
// (queryの引数はそれ以外でもよい)
#[derive(Debug)]
struct SparseSegmentTree2D<M>
where
    M: Monoid,
{
    cap: usize,
    pos_idxs: BTreeMap<i64, usize>,
    values: Vec<SparseSegmentTree<M>>,
}

#[allow(dead_code)]
impl<M> SparseSegmentTree2D<M>
where
    M: Monoid + Clone,
{
    fn new(poss: impl IntoIterator<Item = (i64, i64)>) -> Self {
        let mut poss = poss.into_iter().collect::<Vec<_>>();
        poss.sort();
        poss.dedup();
        let mut poss0 = poss.iter().map(|&(i, _)| i).collect::<Vec<_>>();
        poss0.dedup();
        let pos_idxs = poss0
            .iter()
            .enumerate()
            .map(|(pos_idx, &i)| (i, pos_idx))
            .collect::<BTreeMap<_, _>>();
        let cap = pos_idxs.len().next_power_of_two();

        let poss = &poss;
        let poss0 = &poss0;

        let height = cap.trailing_zeros() as usize + 1;
        let values = (0..height)
            .flat_map(|depth| {
                (0..1 << depth)
                    .scan(0, move |org_idx, nth| {
                        let w = cap >> depth;

                        let pos_idx_right = (nth + 1) * w;

                        let i_right = poss0.get(pos_idx_right).copied().unwrap_or(std::i64::MAX);

                        let org_idx0 = *org_idx;
                        while *org_idx < poss.len() && poss[*org_idx].0 < i_right {
                            *org_idx += 1;
                        }

                        Some((org_idx0, *org_idx))
                    })
                    .map(|(org_idx_left, org_idx_right)| {
                        SparseSegmentTree::new(
                            poss[org_idx_left..org_idx_right]
                                .iter()
                                .copied()
                                .map(|(_, j)| j),
                        )
                    })
            })
            .collect::<Vec<_>>();

        SparseSegmentTree2D {
            cap,
            pos_idxs,
            values,
        }
    }

    // internal
    // 子の値を反映する
    fn fix(&mut self, idx: usize, j: i64) {
        if !self.values[idx].contains(j) {
            return;
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            let x = M::op(
                &self.values[left_idx].get(j),
                &self.values[right_idx].get(j),
            );
            self.values[idx].set(j, x);
        }
    }

    // internal
    // idxの全ての祖先でfixする
    fn fix_all(&mut self, mut idx: usize, j: i64) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx, j);
        }
    }

    fn get(&self, i: i64, j: i64) -> M {
        if let Some(pos_idx) = self.pos_idxs.get(&i) {
            self.values[self.cap - 1 + pos_idx].get(j)
        } else {
            M::id()
        }
    }

    fn set<T>(&mut self, i: i64, j: i64, v: T)
    where
        T: Into<M>,
    {
        let pos_idx = self.pos_idxs.get(&i).unwrap();
        let idx = self.cap - 1 + pos_idx;

        self.values[idx].set(j, v);

        self.fix_all(idx, j);
    }

    fn query<IRange, JRange>(&self, i_range: IRange, j_range: JRange) -> M
    where
        IRange: std::ops::RangeBounds<i64>,
        JRange: std::ops::RangeBounds<i64> + Clone,
    {
        let (a, b) = range(i_range, std::i64::MIN, std::i64::MAX);

        let n = self.pos_idxs.len();
        let left_pos_idx = self
            .pos_idxs
            .range(a..)
            .next()
            .map_or(n, |(_, &pos_idx)| pos_idx);
        let right_pos_idx = self
            .pos_idxs
            .range(b..)
            .next()
            .map_or(n, |(_, &pos_idx)| pos_idx);

        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = left_pos_idx + self.cap - 1;
        let mut right_idx = right_pos_idx + self.cap - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.values[left_idx].query(j_range.clone()));
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.values[right_idx - 1].query(j_range.clone()), &right);
                right_idx -= 1;
            }

            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }

        M::op(&left, &right)
    }
}

define_monoid!(MinPair, (i64, usize), (1 << 60, usize::MAX), std::cmp::min);
define_monoid!(
    MaxPair,
    (i64, usize),
    (-(1 << 60), usize::MAX),
    std::cmp::max
);
define_monoid!(Max, usize, 0, std::cmp::max);

fn dfs(
    lr: &[(i64, i64)],
    i: usize,
    vs: &mut Vec<usize>,
    left_st: &mut SegmentTree<MinPair>,
    right_st: &mut SegmentTree<MinPair>,
) {
    let (_l, r) = lr[i];

    // mark visited
    left_st.set(i, MinPair::id());
    right_st.set(i, MinPair::id());

    // j < i
    // r_i - (i - j) >= l_j
    loop {
        let x = left_st.query(0..i);
        if x == MinPair::id() {
            break;
        }

        let (_, j) = x.0;

        if r - ((i - j) as i64) < lr[j].0 {
            break;
        }

        dfs(lr, j, vs, left_st, right_st);
    }

    // j > i
    // r_i - (j - i) >= l_j
    loop {
        let x = right_st.query(i + 1..);
        if x == MinPair::id() {
            break;
        }

        let (_, j) = x.0;

        if r - ((j - i) as i64) < lr[j].0 {
            break;
        }

        dfs(lr, j, vs, left_st, right_st);
    }

    vs.push(i);
}

fn rev_dfs(
    lr: &[(i64, i64)],
    i: usize,
    vs: &mut Vec<usize>,
    left_st: &mut SegmentTree<MaxPair>,
    right_st: &mut SegmentTree<MaxPair>,
) {
    let (l, _r) = lr[i];

    // mark visited
    left_st.set(i, MaxPair::id());
    right_st.set(i, MaxPair::id());

    // i < j
    // r_j - (j - i) >= l_i
    loop {
        let x = left_st.query(i + 1..);
        if x == MaxPair::id() {
            break;
        }

        let (_, j) = x.0;

        if lr[j].1 - ((j - i) as i64) < l {
            break;
        }

        rev_dfs(lr, j, vs, left_st, right_st);
    }

    // j < i
    // r_j - (i - j) >= l_i
    loop {
        let x = right_st.query(0..i);
        if x == MaxPair::id() {
            break;
        }

        let (_, j) = x.0;

        if lr[j].1 - ((i - j) as i64) < l {
            break;
        }

        rev_dfs(lr, j, vs, left_st, right_st);
    }

    vs.push(i);
}

fn main() {
    let n = read::<usize>();
    let l = read_row::<i64>();
    let r = read_row::<i64>();

    let lr = izip!(l, r)
        .chain(once((-(1 << 50), -(1 << 50))))
        .chain(once((1 << 50, 1 << 50)))
        .collect::<Vec<_>>();
    let mut l_minus_min_st = SegmentTree::<MinPair>::with(
        &lr.citer()
            .enumerate()
            .map(|(i, (l, _r))| MinPair((l - i as i64, i)))
            .collect::<Vec<_>>(),
    );
    let mut l_plus_min_st = SegmentTree::<MinPair>::with(
        &lr.citer()
            .enumerate()
            .map(|(i, (l, _r))| MinPair((l + i as i64, i)))
            .collect::<Vec<_>>(),
    );

    let mut vs = vec![];
    dfs(&lr, n + 1, &mut vs, &mut l_minus_min_st, &mut l_plus_min_st);
    assert!(vs.len() == n + 2);

    let mut r_minus_max_st = SegmentTree::<MaxPair>::with(
        &lr.citer()
            .enumerate()
            .map(|(i, (_l, r))| MaxPair((r - i as i64, i)))
            .collect::<Vec<_>>(),
    );
    let mut r_plus_max_st = SegmentTree::<MaxPair>::with(
        &lr.citer()
            .enumerate()
            .map(|(i, (_l, r))| MaxPair((r + i as i64, i)))
            .collect::<Vec<_>>(),
    );

    let mut component_idxs = vec![0; n + 2];
    let mut components = vec![];
    for i in vs.into_iter().rev() {
        if r_minus_max_st.get(i) == MaxPair::id() {
            // visited
            continue;
        }

        let mut vs = vec![];
        rev_dfs(&lr, i, &mut vs, &mut r_minus_max_st, &mut r_plus_max_st);

        for &j in &vs {
            component_idxs[j] = components.len();
        }
        components.push(vs);
    }

    let st2d = SparseSegmentTree2D::<Max>::new(
        lr.citer()
            .enumerate()
            .map(|(i, (l, _r))| (l - i as i64, l + i as i64)),
    );

    let ans = components
        .iter()
        .rev()
        .scan(st2d, |st2d, component| {
            let x = component
                .citer()
                .map(|i| {
                    let r = lr[i].1;
                    let x = st2d.query(..=(r - i as i64), ..=(r + i as i64));
                    if x == Max::id() { 0 } else { x.0 }
                })
                .max()
                .unwrap();

            let y = x + component.len();

            component.citer().for_each(|i| {
                let l = lr[i].0;
                st2d.set(
                    l - i as i64,
                    l + i as i64,
                    max(y, st2d.get(l - i as i64, l + i as i64).0),
                );
            });

            Some(y)
        })
        .last()
        .unwrap();

    // ダミーの分を除く
    let ans = ans - 2;

    println!("{}", ans);
}
