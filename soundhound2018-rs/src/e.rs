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

fn range(r: impl std::ops::RangeBounds<usize>, n: usize) -> (usize, usize) {
    let start = match r.start_bound() {
        std::ops::Bound::Excluded(&i) => i + 1,
        std::ops::Bound::Included(&i) => i,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match r.end_bound() {
        std::ops::Bound::Excluded(&i) => i,
        std::ops::Bound::Included(&i) => i + 1,
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
    n: usize,
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
            n,
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
        let mut st = SegmentTree { n, cap, values };
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
        let (a, b) = range(r, self.n);
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
#[derive(Debug)]
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    height: usize,
    n: usize,
    cap: usize,
    values: Vec<M>,
    lazy: Vec<Op>,
}
trait Operator<T>: Monoid {
    fn apply(&self, v: &T) -> T;
}
#[allow(dead_code)]
impl<M, Op> LazySegmentTree<M, Op>
where
    M: Monoid + Clone,
    Op: Monoid + Operator<M> + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
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
        let mut st = LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            values,
            lazy: vec![Op::id(); 2 * cap - 1],
        };
        for idx in (0..cap - 1).rev() {
            st.fix(idx);
        }
        st
    }
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = Op::apply(
                &self.lazy[idx],
                &M::op(&self.values[left_idx], &self.values[right_idx]),
            );
        }
    }
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
        self.values[idx] = Op::apply(p, &self.values[idx]);
    }
    fn push(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);
        if left_idx < self.values.len() {
            let l = self.lazy[parent_idx].clone();
            self.apply(left_idx, &l);
            self.apply(right_idx, &l);
            self.lazy[parent_idx] = Op::id();
        }
    }
    fn push_all(&mut self, idx: usize) {
        for i in (1..(idx + 2).next_power_of_two().trailing_zeros()).rev() {
            self.push(((idx + 1) >> i) - 1);
        }
    }
    fn get(&mut self, pos: usize) -> M {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.values[idx].clone()
    }
    fn set<T>(&mut self, pos: usize, v: T)
    where
        T: Into<M>,
    {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.values[idx] = v.into();
        self.lazy[idx] = Op::id();
        self.fix_all(idx);
    }
    fn update<T>(&mut self, r: impl std::ops::RangeBounds<usize>, p: T)
    where
        T: Into<Op>,
    {
        let (a, b) = range(r, self.n);
        let p = p.into();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);
        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.apply(left_idx, &p);
            }
            if right_idx % 2 == 0 {
                self.apply(right_idx - 1, &p);
            }
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
        self.fix_all(a + self.cap - 1);
        self.fix_all(b + self.cap - 1);
    }
    fn query(&mut self, r: impl std::ops::RangeBounds<usize>) -> M {
        let (a, b) = range(r, self.n);
        let mut left = M::id();
        let mut right = M::id();
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);
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
}
macro_rules! define_monoid {
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
        #[derive(Clone, Copy, Debug)]
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

define_monoid!(Min, i64, i64::MAX, std::cmp::min);
define_monoid!(Sum, i64, 0, std::ops::Add::add);

impl Operator<Min> for Sum {
    fn apply(&self, v: &Min) -> Min {
        Min(v.0 + self.0)
    }
}

fn main() {
    let (n, q) = read_tuple!(usize, usize);
    let s = read_str();
    let query = read_col::<usize>(q);

    let opens = (1..=n)
        .filter(|&i| s[i - 1] == '(')
        .collect::<BTreeSet<_>>();
    let closes = (1..=n)
        .filter(|&i| s[i - 1] == ')')
        .collect::<BTreeSet<_>>();
    let st = LazySegmentTree::<Min, Sum>::with(&vec![0; n + 2]);
    let st_num = SegmentTree::<Sum>::new(n + 2);

    let (pos, score, st, st_num) = (0..n)
        .try_fold(
            ((0, n + 1), 0, st, st_num),
            |(pos, score, mut st, mut st_num), _| {
                let next_o = if let Some(&next_o) = opens.range(pos.0 + 1..).next() {
                    next_o
                } else {
                    return Err((pos, score, st, st_num));
                };
                let next_c = if let Some(&next_c) = closes.range(..pos.1).next_back() {
                    next_c
                } else {
                    return Err((pos, score, st, st_num));
                };

                st.update(next_o.., 1);
                st.update(next_c.., -1);

                let score_diff = -(st_num.query(..=next_o).0 + 1)
                    - (st.get(n).0 - st.get(next_o).0)
                    + (st_num.query(..=next_c).0 + 1)
                    - (st.get(n).0 - st.get(next_c).0);

                if st.query(..).0 < 0 || score_diff < 0 {
                    // rollback
                    st.update(next_o.., -1);
                    st.update(next_c.., 1);

                    Err((pos, score, st, st_num))
                } else {
                    st_num.set(next_o, 1);
                    st_num.set(next_c, 1);
                    Ok(((next_o, next_c), score + score_diff, st, st_num))
                }
            },
        )
        .map_or_else(|t| t, |t| t);

    // eprintln!("{}; score={}", s.citer().join(""), score);

    query
        .citer()
        .scan(
            (pos, score, opens, closes, st, st_num),
            |(pos, score, opens, closes, st, st_num), k| {
                if closes.contains(&k) {
                    // ')' => '('
                    closes.remove(&k);
                    opens.insert(k);
                    let (o, c) = *pos;
                    if c <= k {
                        *score -= st_num.query(..=k).0;
                        *score += st.get(n).0 - st.get(k).0;
                        st.update(k.., 1);
                        st_num.set(k, 0);
                    }
                    if k <= o {
                        *score -= st_num.query(..=k).0 + 1;
                        *score -= st.get(n).0 - st.get(k).0;
                        st.update(k.., 1);
                        st_num.set(k, 1);
                    }
                } else {
                    // '(' => ')'
                    opens.remove(&k);
                    closes.insert(k);
                    let (o, c) = *pos;
                    if k <= o {
                        *score += st_num.query(..=k).0;
                        *score += st.get(n).0 - st.get(k).0;
                        st.update(k.., -1);
                        st_num.set(k, 0);
                    }
                    if c <= k {
                        *score += st_num.query(..=k).0 + 1;
                        *score -= st.get(n).0 - st.get(k).0;
                        st.update(k.., -1);
                        st_num.set(k, 1);
                    }
                }

                // 開きと閉じの個数が揃ってない場合の補正
                while st.get(n).0 < 0 {
                    let c = *closes.range(pos.1..).next().unwrap();

                    *score -= st_num.query(..=c).0;
                    *score += st.get(n).0 - st.get(c).0;
                    st.update(c.., 1);
                    st_num.set(c, 0);
                    *pos = (pos.0, c + 1);
                }
                while st.get(n).0 > 0 {
                    let o = *opens.range(..=pos.0).next_back().unwrap();

                    *score += st_num.query(..=o).0;
                    *score += st.get(n).0 - st.get(o).0;
                    st.update(o.., -1);
                    st_num.set(o, 0);
                    *pos = (o - 1, pos.1);
                }

                // 対応付いた括弧列になっていない場合の補正
                while st.query(..).0 < 0 {
                    let o = *opens.range(..=pos.0).next_back().unwrap();
                    let c = *closes.range(pos.1..).next().unwrap();

                    *score -= st_num.query(..=c).0;
                    *score += st.get(n).0 - st.get(c).0;
                    *score += st_num.query(..=o).0;
                    *score += st.get(n).0 - st.get(o).0;
                    st.update(o.., -1);
                    st.update(c.., 1);
                    st_num.set(o, 0);
                    st_num.set(c, 0);

                    *pos = (o - 1, c + 1);
                }

                // 短くしたほうがスコアが良い場合
                loop {
                    let o = if let Some(&o) = opens.range(..=pos.0).next_back() {
                        o
                    } else {
                        break;
                    };
                    let c = if let Some(&c) = closes.range(pos.1..).next() {
                        c
                    } else {
                        break;
                    };

                    st.update(o.., -1);
                    st.update(c.., 1);

                    let score_diff = -st_num.query(..=c).0
                        + (st.get(n).0 - st.get(c).0)
                        + st_num.query(..=o).0
                        + (st.get(n).0 - st.get(o).0);

                    if score_diff < 0 {
                        // rollback
                        st.update(o.., 1);
                        st.update(c.., -1);
                        break;
                    }
                    *score += score_diff;
                    st_num.set(o, 0);
                    st_num.set(c, 0);

                    *pos = (o - 1, c + 1);
                }

                // eprintln!("pos={:?}; score={}; st[n]={}", pos, *score, st.get(n).0);

                // 長くしたほうがスコアが良い場合
                loop {
                    let o = opens.range(..=pos.0).next_back().copied().unwrap_or(0);
                    let c = closes.range(pos.1..).next().copied().unwrap_or(n + 1);

                    let next_o = opens.range(o + 1..).next().copied();
                    let next_c = closes.range(..c).next_back().copied();

                    let ok = {
                        if let Some(next_o) = next_o {
                            if let Some(next_c) = next_c {
                                st.update(next_o.., 1);
                                st.update(next_c.., -1);

                                let score_diff = (st_num.query(..=next_c).0 + 1)
                                    - (st.get(n).0 - st.get(next_c).0)
                                    - (st_num.query(..=next_o).0 + 1)
                                    - (st.get(n).0 - st.get(next_o).0);

                                if st.query(..).0 >= 0 && score_diff > 0 {
                                    *score += score_diff;
                                    st_num.set(next_o, 1);
                                    st_num.set(next_c, 1);
                                    *pos = (next_o, next_c);
                                    true
                                } else {
                                    // rollback
                                    st.update(next_o.., -1);
                                    st.update(next_c.., 1);
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    if !ok {
                        break;
                    }
                }

                Some(*score)
            },
        )
        .for_each(|ans| {
            println!("{}", ans);
        })
}
