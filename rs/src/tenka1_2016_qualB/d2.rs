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

trait Monoid {
    fn id() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
#[derive(Debug)]
struct DualSegmentTree<Op>
where
    Op: Monoid,
{
    height: usize,
    n: usize,
    cap: usize,
    lazy: Vec<Op>,
}
#[allow(dead_code)]
impl<Op> DualSegmentTree<Op>
where
    Op: Monoid + Clone,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        DualSegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }
    fn with<T>(vals: &[T]) -> Self
    where
        T: Into<Op> + Clone,
    {
        let n = vals.len();
        let cap = n.next_power_of_two();
        let mut lazy = Vec::with_capacity(2 * cap - 1);
        lazy.resize(cap - 1, Op::id());
        lazy.extend(vals.iter().cloned().map(|x| x.into()));
        lazy.resize(2 * cap - 1, Op::id());
        DualSegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            n,
            cap,
            lazy,
        }
    }
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
    }
    fn push(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);
        if left_idx < self.lazy.len() {
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
    fn get(&mut self, pos: usize) -> Op {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.lazy[idx].clone()
    }
    fn get_all(&mut self) -> &[Op] {
        for idx in 0..self.cap - 1 {
            self.push(idx);
        }
        &self.lazy[self.cap - 1..self.cap - 1 + self.n]
    }
    fn set<T>(&mut self, pos: usize, p: T)
    where
        T: Into<Op>,
    {
        let idx = self.cap - 1 + pos;
        self.push_all(idx);
        self.lazy[idx] = p.into();
    }
    fn update<T>(&mut self, a: usize, b: usize, p: T)
    where
        T: Into<Op>,
    {
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

define_monoid!(
    AddMin,
    (i32, i32),
    (0, 0),
    |(x0, y0): (i32, i32), (x1, y1): (i32, i32)| { (x0 + x1, min(y1, x1 + y0)) }
);

fn main() {
    let n = read::<usize>();
    let a = read_row::<i64>();
    let qa = read::<usize>();
    let lrx = read_vec(qa, || read_tuple!(usize, usize, i32));
    let qb = read::<usize>();
    let stk = read_vec(qb, || read_tuple!(usize, usize, usize));

    const D: usize = 220;

    let starts = stk.citer().enumerate().fold(
        vec![vec![]; qa + 1],
        |mut starts, (i_query, (s, _t, _k))| {
            starts[s - 1].push(i_query);
            starts
        },
    );

    let (ans, _, _) = lrx.chunks(D).enumerate().fold(
        (
            vec![i64::MAX; qb],
            a,
            BinaryHeap::<(Reverse<usize>, usize)>::new(),
        ),
        |(mut ans, a, mut block_qidxs), (i_block, chunk)| {
            let mut each_qidxs = BinaryHeap::<(Reverse<usize>, usize)>::new();
            while matches!(block_qidxs.peek(), Some(&(Reverse(t), _)) if t < (i_block + 1) * D) {
                each_qidxs.push(block_qidxs.pop().unwrap());
            }

            let mut st = chunk.citer().enumerate().fold(
                DualSegmentTree::<AddMin>::new(n),
                |mut st, (i, (l, r, x))| {
                    let i_query = i_block * D + i;

                    let l = l - 1;

                    for &qidx in &starts[i_query] {
                        let t = stk[qidx].1;
                        each_qidxs.push((Reverse(t), qidx));
                    }

                    for &(_, qidx) in each_qidxs.iter() {
                        let k = stk[qidx].2 - 1;
                        if stk[qidx].0 - 1 == i_query || (i_query > 0 && lrx[i_query - 1].2 < 0) {
                            ans[qidx] = min(ans[qidx], a[k] + (st.get(k).0).0 as i64);
                        }
                    }

                    while matches!(each_qidxs.peek(), Some(&(Reverse(t), _)) if t == i_query) {
                        each_qidxs.pop();
                    }

                    st.update(l, r, (x, min(0, x)));

                    st
                },
            );

            let ops = st.get_all();

            // 列の一番最後でうまくいかんのでやる
            for &(_, qidx) in each_qidxs.iter() {
                let k = stk[qidx].2 - 1;
                ans[qidx] = min(ans[qidx], a[k] + (ops[k].0).0 as i64);
            }

            for &(_, qidx) in block_qidxs.iter() {
                let k = stk[qidx].2 - 1;
                ans[qidx] = min(ans[qidx], a[k] + (ops[k].0).1 as i64);
            }
            let new_a = a
                .citer()
                .enumerate()
                .map(|(i, x)| x + (ops[i].0).0 as i64)
                .collect::<Vec<_>>();

            block_qidxs.extend(each_qidxs.into_iter());

            (ans, new_a, block_qidxs)
        },
    );

    for x in ans {
        println!("{}", x);
    }
}
