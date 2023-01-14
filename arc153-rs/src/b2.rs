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
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

// 通常のtreapとimplicit treap兼用
struct TreapNode<K, M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    left: Option<Box<TreapNode<K, M, Op>>>,
    right: Option<Box<TreapNode<K, M, Op>>>,
    size: usize,
    priority: usize,
    key: K,
    reversed: bool,
    lazy: Op::Item,
    // lazy適用後の値
    acc: M::Item,
    // lazy適用後の値
    value: M::Item,
}

impl<K, M, Op> TreapNode<K, M, Op>
where
    K: Ord,
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new(priority: usize, key: K, value: M::Item) -> TreapNode<K, M, Op> {
        TreapNode {
            left: None,
            right: None,
            size: 1,
            reversed: false,
            priority,
            key,
            lazy: Op::id(),
            acc: value.clone(),
            value,
        }
    }

    fn fix(&mut self) {
        self.size = 1
            + self.left.as_deref().map_or(0, |l| l.size)
            + self.right.as_deref().map_or(0, |r| r.size);
        self.acc = M::op(
            &M::op(
                &Op::apply(
                    &self.lazy,
                    self.left.as_deref().map_or(&M::id(), |l| &l.acc),
                ),
                &self.value,
            ),
            &Op::apply(
                &self.lazy,
                self.right.as_deref().map_or(&M::id(), |l| &l.acc),
            ),
        );
    }

    fn push(&mut self) {
        if let Some(t) = self.left.as_deref_mut() {
            t.lazy = Op::op(&self.lazy, &t.lazy);
            t.acc = Op::apply(&self.lazy, &t.acc);
            t.value = Op::apply(&self.lazy, &t.value);

            if self.reversed {
                t.reversed ^= true;
            }
        }
        if let Some(t) = self.right.as_deref_mut() {
            t.lazy = Op::op(&self.lazy, &t.lazy);
            t.acc = Op::apply(&self.lazy, &t.acc);
            t.value = Op::apply(&self.lazy, &t.value);

            if self.reversed {
                t.reversed ^= true;
            }
        }

        if self.reversed {
            std::mem::swap(&mut self.left, &mut self.right);
        }

        self.lazy = Op::id();
        self.reversed = false;
    }

    // selfの要素がrhsの要素より小さいときのみ使用可
    fn merge(lhs: Option<Box<Self>>, rhs: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut lhs = if let Some(lhs) = lhs {
            lhs
        } else {
            return rhs;
        };

        let mut rhs = if let Some(rhs) = rhs {
            rhs
        } else {
            return Some(lhs);
        };

        if lhs.priority < rhs.priority {
            rhs.push();
            rhs.left = TreapNode::merge(Some(lhs), rhs.left);
            rhs.fix();
            Some(rhs)
        } else {
            lhs.push();
            lhs.right = TreapNode::merge(lhs.right, Some(rhs));
            lhs.fix();
            Some(lhs)
        }
    }

    // x以下とxより大きい部分に分ける
    fn split_upper_bound(t: Option<Box<Self>>, x: &K) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        if *x < t.key {
            let (l, r) = TreapNode::split_upper_bound(t.left, x);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_upper_bound(t.right, x);
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    // x未満とx以上の部分に分ける
    fn split_lower_bound(t: Option<Box<Self>>, x: &K) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        if *x <= t.key {
            let (l, r) = TreapNode::split_lower_bound(t.left, x);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_lower_bound(t.right, x);
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    // nth番目以降を分離
    fn split_at(t: Option<Box<Self>>, nth: usize) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        let left_size = TreapNode::size(&t.left);

        if nth <= left_size {
            let (l, r) = TreapNode::split_at(t.left, nth);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_at(t.right, nth - (left_size + 1));
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    fn insert(t: Option<Box<Self>>, priority: usize, key: K, value: M::Item) -> Box<Self> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return Box::new(TreapNode::new(priority, key, value));
        };

        if t.priority < priority {
            let (l, r) = TreapNode::split_upper_bound(Some(t), &key);
            let mut v = Box::new(TreapNode::new(priority, key, value));
            v.left = l;
            v.right = r;
            v.fix();
            v
        } else if key < t.key {
            t.push();
            t.left = Some(TreapNode::insert(t.left, priority, key, value));
            t.fix();
            t
        } else {
            t.push();
            t.right = Some(TreapNode::insert(t.right, priority, key, value));
            t.fix();
            t
        }
    }

    // keyの順序を無視してnth番目に挿入する
    // Implicit treap用
    fn insert_at(
        t: Option<Box<Self>>,
        nth: usize,
        priority: usize,
        key: K,
        value: M::Item,
    ) -> Box<Self> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return Box::new(TreapNode::new(priority, key, value));
        };

        t.push();
        let left_size = TreapNode::size(&t.left);
        if t.priority < priority {
            let (l, r) = TreapNode::split_at(Some(t), nth);
            let mut v = Box::new(TreapNode::new(priority, key, value));
            v.left = l;
            v.right = r;
            v.fix();
            v
        } else if nth <= left_size {
            t.left = Some(TreapNode::insert_at(t.left, nth, priority, key, value));
            t.fix();
            t
        } else {
            t.right = Some(TreapNode::insert_at(
                t.right,
                nth - (left_size + 1),
                priority,
                key,
                value,
            ));
            t.fix();
            t
        }
    }

    // 指定されたkeyを持つ要素が複数あったら一つだけ消す
    fn remove(t: Option<Box<Self>>, key: &K) -> Option<Box<Self>> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return None;
        };

        t.push();

        use std::cmp::Ordering;
        match t.key.cmp(&key) {
            Ordering::Less => {
                t.right = TreapNode::remove(t.right, key);
                t.fix();
                Some(t)
            }
            Ordering::Equal => TreapNode::merge(t.left, t.right),
            Ordering::Greater => {
                t.left = TreapNode::remove(t.left, key);
                t.fix();
                Some(t)
            }
        }
    }

    // nth番目の要素を消す
    fn remove_at(t: Option<Box<Self>>, nth: usize) -> (Option<Box<Self>>, Option<(K, M::Item)>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        if nth >= t.size {
            return (Some(t), None);
        }

        t.push();

        let (tl, v) = TreapNode::remove_at(t.left, nth);
        t.left = tl;
        if let Some(v) = v {
            t.fix();
            return (Some(t), Some(v));
        }

        let left_size = TreapNode::size(&t.left);

        if left_size == nth {
            let ret = (t.key, t.value);
            return (TreapNode::merge(t.left, t.right), Some(ret));
        }

        let (tr, v) = TreapNode::remove_at(t.right, nth - (left_size + 1));
        t.right = tr;
        t.fix();

        (Some(t), v)
    }

    fn size(t: &Option<Box<Self>>) -> usize {
        t.as_deref().map_or(0, |t| t.size)
    }

    fn at(t: &mut Option<Box<Self>>, nth: usize) -> Option<(&K, &M::Item)> {
        let t = t.as_deref_mut()?;

        if nth >= t.size {
            return None;
        }

        t.push();

        let left_size = TreapNode::size(&t.left);
        if let Some(x) = TreapNode::at(&mut t.left, nth) {
            return Some(x);
        }

        if left_size == nth {
            return Some((&t.key, &t.value));
        }

        TreapNode::at(&mut t.right, nth - (left_size + 1))
    }

    // TODO: implement without Box
    fn iter<'a>(
        t: &'a mut Option<Box<Self>>,
    ) -> Box<dyn 'a + Iterator<Item = (&'a K, &'a M::Item)>> {
        Box::new(t.iter_mut().flat_map(|t| {
            t.push();

            TreapNode::iter(&mut t.left)
                .chain(std::iter::once((&t.key, &t.value)))
                .chain(TreapNode::iter(&mut t.right))
        }))
    }
}

struct Treap<K, M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    // To share when split, use Rc.
    rng: std::rc::Rc<std::cell::RefCell<rand::rngs::SmallRng>>,
    root: Option<Box<TreapNode<K, M, Op>>>,
}

#[allow(dead_code)]
impl<K, M, Op> Treap<K, M, Op>
where
    K: Ord + Copy,
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new() -> Treap<K, M, Op> {
        Treap {
            rng: std::rc::Rc::new(std::cell::RefCell::new(rand::rngs::SmallRng::from_entropy())),
            root: None,
        }
    }

    fn merge(&mut self, rhs: Treap<K, M, Op>) {
        self.root = TreapNode::merge(std::mem::replace(&mut self.root, None), rhs.root);
    }

    fn split(&mut self, x: &K) -> Treap<K, M, Op> {
        let (l, r) = TreapNode::split_upper_bound(std::mem::replace(&mut self.root, None), x);
        self.root = l;

        Treap {
            rng: std::rc::Rc::clone(&self.rng),
            root: r,
        }
    }

    fn insert(&mut self, key: K) {
        self.insert_with_value(key, M::id());
    }

    fn insert_with_value(&mut self, key: K, value: M::Item) {
        self.root = Some(TreapNode::insert(
            std::mem::replace(&mut self.root, None),
            self.rng.borrow_mut().gen(),
            key,
            value,
        ));
    }

    fn remove(&mut self, key: &K) {
        self.root = TreapNode::remove(std::mem::replace(&mut self.root, None), key);
    }

    fn remove_at(&mut self, nth: usize) -> Option<(K, M::Item)> {
        let (t, v) = TreapNode::remove_at(std::mem::replace(&mut self.root, None), nth);
        self.root = t;
        v
    }

    fn size(&self) -> usize {
        TreapNode::size(&self.root)
    }

    // 内部的に変更走るのでmut
    fn at(&mut self, nth: usize) -> Option<(&K, &M::Item)> {
        TreapNode::at(&mut self.root, nth)
    }

    // 内部的に変更走るのでmut
    fn query(&mut self, lower: &K, upper: &K) -> M::Item {
        assert!(*lower <= *upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_lower_bound(root, lower);
        let (t1, t2) = TreapNode::split_lower_bound(t12, upper);

        let v = t1.as_deref().map_or(M::id(), |t1| t1.acc.clone());

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));

        v
    }

    fn update(&mut self, lower: &K, upper: &K, x: Op::Item) {
        assert!(*lower <= *upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_lower_bound(root, lower);
        let (mut t1, t2) = TreapNode::split_lower_bound(t12, upper);

        if let Some(t1) = t1.as_deref_mut() {
            t1.acc = Op::apply(&x, &t1.acc);
            t1.value = Op::apply(&x, &t1.value);
            t1.lazy = x;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }

    fn iter(&mut self) -> impl Iterator<Item = (&K, &M::Item)> {
        TreapNode::iter(&mut self.root)
    }
}

// keyを保持せずにindexで操作するtreap
struct ImplicitTreap<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    // To share when split, use Rc.
    rng: std::rc::Rc<std::cell::RefCell<rand::rngs::SmallRng>>,
    root: Option<Box<TreapNode<(), M, Op>>>,
}

#[allow(dead_code)]
impl<M, Op> ImplicitTreap<M, Op>
where
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new() -> ImplicitTreap<M, Op> {
        ImplicitTreap {
            rng: std::rc::Rc::new(std::cell::RefCell::new(rand::rngs::SmallRng::from_entropy())),
            root: None,
        }
    }

    fn merge(&mut self, rhs: ImplicitTreap<M, Op>) {
        self.root = TreapNode::merge(std::mem::replace(&mut self.root, None), rhs.root);
    }

    fn split_at(&mut self, nth: usize) -> ImplicitTreap<M, Op> {
        let (l, r) = TreapNode::split_at(std::mem::replace(&mut self.root, None), nth);
        self.root = l;

        ImplicitTreap {
            rng: std::rc::Rc::clone(&self.rng),
            root: r,
        }
    }

    fn insert_at(&mut self, nth: usize, value: M::Item) {
        self.root = Some(TreapNode::insert_at(
            std::mem::replace(&mut self.root, None),
            nth,
            self.rng.borrow_mut().gen(),
            (),
            value,
        ));
    }

    fn remove_at(&mut self, nth: usize) -> Option<M::Item> {
        let (t, v) = TreapNode::remove_at(std::mem::replace(&mut self.root, None), nth);
        self.root = t;
        v.map(|(_, v)| v)
    }

    fn size(&self) -> usize {
        TreapNode::size(&self.root)
    }

    // 内部的に変更走るのでmut
    fn at(&mut self, nth: usize) -> Option<&M::Item> {
        TreapNode::at(&mut self.root, nth).map(|(_, v)| v)
    }

    // 内部的に変更走るのでmut
    fn query(&mut self, lower: usize, upper: usize) -> M::Item {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (t1, t2) = TreapNode::split_at(t12, upper - lower);

        let v = t1.as_deref().map_or(M::id(), |t1| t1.acc.clone());

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));

        v
    }

    fn update(&mut self, lower: usize, upper: usize, x: Op::Item) {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (mut t1, t2) = TreapNode::split_at(t12, upper - lower);

        if let Some(t1) = t1.as_deref_mut() {
            t1.acc = Op::apply(&x, &t1.acc);
            t1.value = Op::apply(&x, &t1.value);
            t1.lazy = x;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }

    fn reverse(&mut self, lower: usize, upper: usize) {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (mut t1, t2) = TreapNode::split_at(t12, upper - lower);

        if let Some(t1) = t1.as_deref_mut() {
            t1.reversed ^= true;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }
}

struct NullMonoid;

impl Monoid for NullMonoid {
    type Item = ();

    fn id() -> Self::Item {
        ()
    }

    fn op(_lhs: &Self::Item, _rhs: &Self::Item) -> Self::Item {
        ()
    }
}

impl<T> Operator<T> for NullMonoid
where
    T: Clone,
{
    fn apply(_: &Self::Item, v: &T) -> T {
        v.clone()
    }
}

struct Sum;

impl Monoid for Sum {
    type Item = usize;

    fn id() -> Self::Item {
        0
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        lhs + rhs
    }
}

fn main() {
    let (h, w) = read_tuple!(usize, usize);
    let a = read_vec(h, || read_str());
    let q = read::<usize>();
    let ab = read_vec(q, || read_tuple!(usize, usize));

    // monoidは何でもよい（集約演算使わないので）
    let mut row_t = ImplicitTreap::<Sum, NullMonoid>::new();
    for i in 0..h {
        row_t.insert_at(i, i);
    }
    let mut row_t = ab.citer().fold(row_t, |mut t, (a, _)| {
        t.reverse(0, a);
        t.reverse(a, h);
        t
    });
    let rows = (0..h).map(|i| *row_t.at(i).unwrap()).collect::<Vec<_>>();

    let mut col_t = ImplicitTreap::<Sum, NullMonoid>::new();
    for i in 0..w {
        col_t.insert_at(i, i);
    }
    let mut col_t = ab.citer().fold(col_t, |mut t, (_, b)| {
        t.reverse(0, b);
        t.reverse(b, w);
        t
    });
    let cols = (0..w).map(|i| *col_t.at(i).unwrap()).collect::<Vec<_>>();

    for i in rows.citer() {
        println!("{}", cols.citer().map(|j| a[i][j]).join(""));
    }
}
