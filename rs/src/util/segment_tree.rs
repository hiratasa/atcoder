use cargo_snippet::snippet;

// NOTE: 区間長に依存する場合（たとえば一定の値を足す、など）
//  解決策1) 値を (value, len) にする
//  解決策2) fがlenも受け取れるようにする

#[snippet("segtree")]
#[snippet("dualsegtree")]
#[snippet("lazysegtree")]
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

#[snippet("segtree")]
#[snippet("dualsegtree")]
#[snippet("lazysegtree")]
fn segtree_table<M: Monoid + std::fmt::Debug, Op: Monoid + std::fmt::Debug>(
    values: &[M],
    lazy: &[Op],
) -> String {
    let cap = (values.len() + 1) / 2;
    assert!(cap.is_power_of_two());
    let height = cap.trailing_zeros() as usize + 1;
    let enable_lazy = !lazy.is_empty();

    let rows = (0..height)
        .map(|i| {
            ((1usize << i) - 1..(1 << (i + 1)) - 1)
                .map(|idx| {
                    if enable_lazy {
                        format!("{:?}; {:?}", values[idx], lazy[idx])
                    } else {
                        format!("{:?}", values[idx])
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = rows
        .iter()
        .enumerate()
        .map(|(i, cols)| {
            let k = cap >> i;

            cols.iter()
                .map(|s| s.len())
                .map(|l| {
                    // w * k + (k - 1) >= l
                    (l.saturating_sub(k - 1) + k - 1) / k
                })
                .max()
                .unwrap()
        })
        .chain(once(2))
        .max()
        .unwrap();

    use std::iter::once;
    once(
        (0..cap)
            .map(|i| format!("|{:>width$}", i))
            .chain(once("|\n".to_string()))
            .collect::<String>(),
    )
    .chain(once(
        once("|")
            .chain((0..cap * (width + 1) - 1).map(|_| "+"))
            .chain(once("|\n"))
            .collect::<String>(),
    ))
    .chain(rows.iter().enumerate().map(|(i, cols)| {
        let k = cap >> i;

        let w = width * k + (k - 1);
        cols.iter()
            .map(|t| format!("|{:^w$}", t))
            .collect::<String>()
            + "|\n"
    }))
    .collect()
}

#[snippet("segtree")]
#[snippet("dualsegtree")]
#[snippet("lazysegtree")]
trait Monoid {
    fn id() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}

#[snippet("segtree")]
#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    cap: usize,
    values: Vec<M>,
}

#[snippet("segtree")]
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

    // internal
    // 子の値を反映する
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = M::op(&self.values[left_idx], &self.values[right_idx]);
        }
    }

    // internal
    // idxの全ての祖先でfixする
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

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
    // (存在しないときにnを返してしまうとquery(a,n)がfalseのときと区別がつかないのでNoneを返す)
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

    // f(query(a, b)) == true となるaが存在すればその最小のものを返す
    // 存在しない場合はNoneを返す
    fn left_partition_point<F>(&self, b: usize, mut f: F) -> Option<usize>
    where
        F: FnMut(&M) -> bool,
    {
        assert!(b <= self.cap);
        if !f(&M::id()) {
            None
        } else if b == 0 {
            Some(0)
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
                    return Some(0);
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

            Some(a)
        }
    }

    fn dump_table(&self)
    where
        M: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<M, M>(&self.values, &[]));
    }
}

// 演算だけのったセグメント木
// Op: Monoid of operation
#[snippet("dualsegtree")]
#[derive(Debug)]
struct DualSegmentTree<Op>
where
    Op: Monoid,
{
    #[allow(dead_code)]
    height: usize,
    cap: usize,
    n: usize,
    lazy: Vec<Op>,
}

#[snippet("dualsegtree")]
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

    // internal
    // pをidx全体に適用する
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
    }

    // internal
    // lazyを子に伝搬する
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

    // internal
    // idxの全ての祖先でpushする
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

    fn set<T>(&mut self, pos: usize, p: T)
    where
        T: Into<Op>,
    {
        let idx = self.cap - 1 + pos;

        self.push_all(idx);

        self.lazy[idx] = p.into();
    }

    fn update<T>(&mut self, r: impl std::ops::RangeBounds<usize>, p: T)
    where
        T: Into<Op>,
    {
        let (a, b) = range(r, self.n);

        let p = p.into();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        // Opが非可換の場合、[l, r)とその他の区間にまたがるlazyをpushしておく必要がある
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.apply(left_idx, &p);
            }

            if right_idx % 2 == 0 {
                self.apply(right_idx - 1, &p);
            }

            // 偶数の場合は一つ右隣の親になる
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }
    }

    fn dump_table(&self)
    where
        Op: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<Op, Op>(&self.lazy, &[]));
    }
}

// M: Monoid of value
// Op: Monoid of lazy operation
#[snippet("lazysegtree")]
#[derive(Debug)]
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    #[allow(dead_code)]
    height: usize,
    n: usize,
    cap: usize,
    values: Vec<M>,
    lazy: Vec<Op>,
}

#[snippet("lazysegtree")]
trait Operator<T>: Monoid {
    fn apply(&self, v: &T) -> T;
}

#[snippet("lazysegtree")]
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

    // internal
    // 子の値を反映する
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

    // internal
    // idxの全ての祖先でfixする
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }

    // internal
    // pをidx全体に適用する
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
        self.values[idx] = Op::apply(p, &self.values[idx]);
    }

    // internal
    // lazyを子に伝搬する
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

    // internal
    // idxの全ての祖先でpushする
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

        // Opが非可換の場合、[l, r)とその他の区間にまたがるlazyをpushしておく必要がある
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.apply(left_idx, &p);
            }

            if right_idx % 2 == 0 {
                self.apply(right_idx - 1, &p);
            }

            // 偶数の場合は一つ右隣の親になる
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

    fn dump_table(&self)
    where
        M: std::fmt::Debug,
        Op: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<M, Op>(&self.values, &self.lazy));
    }
}

#[snippet("segtree")]
#[snippet("dualsegtree")]
#[snippet("lazysegtree")]
macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
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

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                if *self == Self::id() {
                    write!(f, "id.")
                } else {
                    self.0.fmt(f)
                }
            }
        }
    };
}

use std::collections::BTreeMap;

// 座標圧縮 + Segment木
#[derive(Debug)]
struct SparseSegmentTree<M>
where
    M: Monoid,
{
    pos_idxs: BTreeMap<usize, usize>,
    segtree: SegmentTree<M>,
}

#[allow(dead_code)]
impl<M> SparseSegmentTree<M>
where
    M: Monoid + Clone,
{
    fn new(poss: impl IntoIterator<Item = usize>) -> SparseSegmentTree<M> {
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

    fn get(&self, pos: usize) -> M {
        if let Some(&pos_idx) = self.pos_idxs.get(&pos) {
            self.segtree.get(pos_idx)
        } else {
            M::id()
        }
    }

    fn contains(&self, pos: usize) -> bool {
        self.pos_idxs.contains_key(&pos)
    }

    fn set<T>(&mut self, pos: usize, v: T)
    where
        T: Into<M>,
    {
        let pos_idx = *self.pos_idxs.get(&pos).unwrap();
        self.segtree.set(pos_idx, v);
    }

    fn query(&self, r: impl std::ops::RangeBounds<usize>) -> M {
        let (a, b) = range(r, std::usize::MAX);

        let n = self.pos_idxs.len();
        let pos_idx0 = self
            .pos_idxs
            .range(a..)
            .next()
            .map_or(n, |(_, &pos_idx)| pos_idx);
        let pos_idx1 = self
            .pos_idxs
            .range(b..)
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
    pos_idxs: BTreeMap<usize, usize>,
    values: Vec<SparseSegmentTree<M>>,
}

#[allow(dead_code)]
impl<M> SparseSegmentTree2D<M>
where
    M: Monoid + Clone,
{
    fn new(poss: impl IntoIterator<Item = (usize, usize)>) -> Self {
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

                        let i_right = poss0.get(pos_idx_right).copied().unwrap_or(std::usize::MAX);

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
    fn fix(&mut self, idx: usize, j: usize) {
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
    fn fix_all(&mut self, mut idx: usize, j: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx, j);
        }
    }

    fn get(&self, i: usize, j: usize) -> M {
        if let Some(pos_idx) = self.pos_idxs.get(&i) {
            self.values[self.cap - 1 + pos_idx].get(j)
        } else {
            M::id()
        }
    }

    fn set<T>(&mut self, i: usize, j: usize, v: T)
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
        IRange: std::ops::RangeBounds<usize>,
        JRange: std::ops::RangeBounds<usize> + Clone,
    {
        let (a, b) = range(i_range, std::usize::MAX);

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

define_monoid!(Minimum, i64, i64::MAX, i64::min);
define_monoid!(AddValue, i64, 0, std::ops::Add::add);

impl Operator<Minimum> for AddValue {
    fn apply(&self, v: &Minimum) -> Minimum {
        Minimum(self.0.saturating_add(v.0))
    }
}

#[allow(dead_code)]
type RMTTree = SegmentTree<Minimum>;

#[allow(dead_code)]
type RMTTreeWithAddition = LazySegmentTree<Minimum, AddValue>;

// 失敗しうる作用. 葉に対しては必ず成功すること. また、id()は必ず成功すること
trait TryOperator<T>: Monoid {
    fn try_apply(&self, v: &T) -> Option<T>;
}

// M: Monoid of value
// Op: Monoid of lazy operation
#[derive(Debug)]
struct SegmentTreeBeats<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    #[allow(dead_code)]
    height: usize,
    n: usize,
    cap: usize,
    values: Vec<M>,
    lazy: Vec<Op>,
}

#[allow(dead_code)]
impl<M, Op> SegmentTreeBeats<M, Op>
where
    M: Monoid + Clone,
    Op: Monoid + TryOperator<M> + Clone + PartialEq,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        SegmentTreeBeats {
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

        let mut st = SegmentTreeBeats {
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

    // internal
    // 子の値を反映する
    fn fix(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        if left_idx < self.values.len() {
            self.values[idx] = Op::try_apply(
                &self.lazy[idx],
                &M::op(&self.values[left_idx], &self.values[right_idx]),
            )
            .unwrap();
        }
    }

    // internal
    // idxの全ての祖先でfixする
    fn fix_all(&mut self, mut idx: usize) {
        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix(idx);
        }
    }

    // internal
    // pをidx全体に適用する
    fn apply(&mut self, idx: usize, p: &Op) {
        self.lazy[idx] = Op::op(p, &self.lazy[idx]);
        if let Some(val) = Op::try_apply(p, &self.values[idx]) {
            self.values[idx] = val;
        } else {
            assert!(2 * (idx + 1) < self.values.len());
            self.push(idx);
            self.fix(idx);
        }
    }

    // internal
    // lazyを子に伝搬する
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

    // internal
    // idxの全ての祖先でpushする
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

        // Opが非可換の場合、[l, r)とその他の区間にまたがるlazyをpushしておく必要がある
        self.push_all(((left_idx + 1) >> (left_idx + 1).trailing_zeros()) - 1);
        self.push_all(((right_idx + 1) >> (right_idx + 1).trailing_zeros()) - 1);

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.apply(left_idx, &p);
            }

            if right_idx % 2 == 0 {
                self.apply(right_idx - 1, &p);
            }

            // 偶数の場合は一つ右隣の親になる
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

    fn dump_table(&self)
    where
        M: std::fmt::Debug,
        Op: std::fmt::Debug,
    {
        eprintln!("{}", segtree_table::<M, Op>(&self.values, &self.lazy));
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_segtree() {
        let mut st = RMTTree::new(10);

        st.set(1, 2);
        st.set(3, 4);
        st.set(5, 8);

        assert_eq!(st.query(0..10).0, 2);
        assert_eq!(st.query(1..4).0, 2);
        assert_eq!(st.query(2..4).0, 4);
        assert_eq!(st.query(3..3).0, Minimum::id().0);
        assert_eq!(st.query(4..5).0, Minimum::id().0);
    }

    #[test]
    fn test_segtree_random() {
        use rand::SeedableRng;
        use rand::distr::Distribution;

        let mut rng = rand::rngs::SmallRng::from_os_rng();

        let dist = rand::distr::Uniform::new(0, 1000).unwrap();
        let dist01 = rand::distr::Uniform::new(0, 2).unwrap();

        for i in 0..5 {
            // 2の冪乗のあたりを試す
            let n = 510 + i;
            let distidx = rand::distr::Uniform::new(0, n).unwrap();

            let a0 = std::iter::repeat_with(|| dist.sample(&mut rng))
                .take(n)
                .collect::<Vec<_>>();

            let mut st = RMTTree::with(&a0);
            let mut logs = vec![];

            let q = 1000;
            let mut a = a0.clone();
            for _ in 0..q {
                let t = dist01.sample(&mut rng);

                if t == 0 {
                    // update
                    let pos = distidx.sample(&mut rng);
                    let x = dist.sample(&mut rng);

                    st.set(pos, x);
                    a[pos] = x;

                    logs.push(format!("a[{}] = {};", pos, x));
                } else {
                    // query
                    let l = distidx.sample(&mut rng);
                    let distidx2 = rand::distr::Uniform::new(l, n + 1).unwrap();
                    let r = distidx2.sample(&mut rng);

                    assert_eq!(
                        st.query(l..r).0,
                        a[l..r].iter().copied().min().unwrap_or(Minimum::id().0),
                        "a0: {:?}, a: {:?}, l: {}, r: {}, ops: {}",
                        a0,
                        a,
                        l,
                        r,
                        logs.join(" ")
                    );
                }
            }
        }
    }

    #[test]
    fn test_lazy_segtree() {
        let mut st = RMTTreeWithAddition::new(10);

        st.set(1, 2);
        st.set(3, 4);

        // [inf, 2, inf, 4, ...]

        assert_eq!(st.query(1..4).0, 2);
        assert_eq!(st.query(2..4).0, 4);
        assert_eq!(st.query(3..3).0, Minimum::id().0);
        assert_eq!(st.query(4..5).0, Minimum::id().0);

        st.update(1..5, 2);

        // [inf, 4, inf, 6, ...]

        assert_eq!(st.query(1..4).0, 4);
        assert_eq!(st.query(2..4).0, 6);
    }

    #[test]
    fn test_lazy_segtree_random() {
        use rand::SeedableRng;
        use rand::distr::Distribution;

        let mut rng = rand::rngs::SmallRng::from_os_rng();

        let dist = rand::distr::Uniform::new(0, 100000).unwrap();
        let dist_op = rand::distr::Uniform::new(0, 4).unwrap();

        for i in 0..5 {
            // 2の冪乗のあたりを試す
            let n = 510 + i;
            let distidx = rand::distr::Uniform::new(0, n + 1).unwrap();

            let a0 = std::iter::repeat_with(|| dist.sample(&mut rng))
                .take(n)
                .collect::<Vec<_>>();

            let mut st = RMTTreeWithAddition::with(&a0);
            let mut logs = vec![];

            let q = 1000;
            let mut a = a0.clone();
            for _ in 0..q {
                let t = dist_op.sample(&mut rng);

                if t == 0 {
                    // update
                    let l = distidx.sample(&mut rng);
                    let r = distidx.sample(&mut rng);
                    let (l, r) = if l <= r { (l, r) } else { (r, l) };
                    let x = dist.sample(&mut rng);

                    st.update(l..r, x);
                    for i in l..r {
                        a[i] += x;
                    }

                    logs.push(format!("Add {} for [{}, {});", x, l, r));
                } else if t == 1 {
                    // query
                    let l = distidx.sample(&mut rng);
                    let r = distidx.sample(&mut rng);
                    let (l, r) = if l <= r { (l, r) } else { (r, l) };

                    assert_eq!(
                        st.query(l..r).0,
                        a[l..r].iter().copied().min().unwrap_or(Minimum::id().0),
                        "a0: {:?}, a: {:?}, l: {}, r: {}, ops: {}",
                        a0,
                        a,
                        l,
                        r,
                        logs.join(" ")
                    );
                } else if t == 2 {
                    // set
                    let pos = std::cmp::min(distidx.sample(&mut rng), n - 1);
                    let x = dist.sample(&mut rng);

                    st.set(pos, x);
                    a[pos] = x;

                    logs.push(format!("Set {} for {};", x, pos));
                } else if t == 3 {
                    // get
                    let pos = std::cmp::min(distidx.sample(&mut rng), n - 1);

                    assert_eq!(
                        st.get(pos).0,
                        a[pos],
                        "a0: {:?}, a: {:?}, pos: {}, ops: {}",
                        a0,
                        a,
                        pos,
                        logs.join(" ")
                    );
                }
            }
        }
    }

    #[test]
    fn test_right_partition_point() {
        let vals = vec![10, 13, 7, 8, 15, 2, 4];
        let st = RMTTree::with(&vals);

        for i in 0..vals.len() {
            let mins = std::iter::once((i, Minimum::id().0))
                .chain(vals.iter().copied().enumerate().skip(i).scan(
                    Minimum::id().0,
                    |min, (j, v)| {
                        *min = std::cmp::min(*min, v);
                        // minimum in [i, j] = [i, j + 1)
                        Some((j + 1, *min))
                    },
                ))
                .collect::<Vec<_>>();
            for v in 0..20 {
                assert_eq!(
                    /* actual */ st.right_partition_point(i, |&u| u.0 >= v),
                    /* expected */
                    mins.iter().copied().find(|&(_, u)| u < v).map(|(j, _)| j),
                    " l={}, v={}, mins={:?}",
                    i,
                    v,
                    mins
                );
            }
        }
    }

    #[test]
    fn test_left_partition_point() {
        let vals = vec![10, 13, 7, 8, 15, 2, 4];
        let st = RMTTree::with(&vals);

        for i in 0..vals.len() {
            let mins = std::iter::once((i, Minimum::id().0))
                .chain(vals[..i].iter().copied().enumerate().rev().scan(
                    Minimum::id().0,
                    |min, (j, v)| {
                        *min = std::cmp::min(*min, v);
                        Some((j, *min))
                    },
                ))
                .collect::<Vec<_>>();
            for v in 0..20 {
                assert_eq!(
                    /* actual */ st.left_partition_point(i, |&u| u.0 >= v),
                    /* expected */
                    mins.iter().copied().rfind(|&(_, u)| u >= v).map(|(j, _)| j),
                    " l={}, v={}, mins={:?}",
                    i,
                    v,
                    mins
                );
            }
        }
    }

    #[test]
    fn test_sparse_segtree_random() {
        use rand::Rng;
        use rand::SeedableRng;
        use rand::distr::Distribution;
        use rand::seq::IteratorRandom;

        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        let n = 100;
        let m = 20;

        let dist_pos = rand::distr::Uniform::new(0, n).unwrap();
        let dist_val = rand::distr::Uniform::new(0, 100000).unwrap();

        let poss = (0..n).choose_multiple(&mut rng, m);
        let pos_set = poss
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();

        let mut st = SparseSegmentTree::<Minimum>::new(poss);
        let mut arr = vec![Minimum::id(); n];

        for _ in 0..1000 {
            let pos = dist_pos.sample(&mut rng);

            // 1点取得
            assert_eq!(st.get(pos).0, arr[pos].0);

            // 区間取得1
            assert_eq!(
                st.query(pos..).0,
                arr[pos..]
                    .iter()
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, y))
                    .0
            );

            // 区間取得2
            assert_eq!(
                st.query(..pos).0,
                arr[..pos]
                    .iter()
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, y))
                    .0
            );

            // 区間取得3
            let pos2 = rng.random_range(pos..=n);
            assert_eq!(
                st.query(pos..pos2).0,
                arr[pos..pos2]
                    .iter()
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, y))
                    .0
            );

            // 1点更新
            if pos_set.contains(&pos) {
                let val = dist_val.sample(&mut rng);

                st.set(pos, val);
                arr[pos] = Minimum(val);
            }
        }
    }

    #[test]
    fn test_sparse_segtree_2d_random() {
        use rand::Rng;
        use rand::SeedableRng;
        use rand::distr::Distribution;
        use rand::seq::IteratorRandom;

        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        let n = 10;
        let m = 20;

        let dist_pos = rand::distr::Uniform::new(0, n).unwrap();
        let dist_val = rand::distr::Uniform::new(0, 100000).unwrap();

        let poss0 = (0..n).choose_multiple(&mut rng, m);
        let poss1 = (0..n).choose_multiple(&mut rng, m);
        let poss = poss0.into_iter().zip(poss1).collect::<Vec<_>>();
        let pos_set = poss
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();

        let mut st = SparseSegmentTree2D::<Minimum>::new(poss);
        let mut values = std::collections::HashMap::<(usize, usize), Minimum>::new();

        for _ in 0..1000 {
            let pos = (dist_pos.sample(&mut rng), dist_pos.sample(&mut rng));

            // 1点取得
            assert_eq!(
                st.get(pos.0, pos.1).0,
                values.get(&pos).copied().unwrap_or(Minimum::id()).0
            );

            // 区間取得1
            assert_eq!(
                st.query(pos.0.., pos.1..).0,
                values
                    .iter()
                    .filter_map(|(&key, &value)| {
                        if pos.0 <= key.0 && pos.1 <= key.1 {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, &y))
                    .0
            );

            // 区間取得2
            assert_eq!(
                st.query(..pos.0, ..pos.1).0,
                values
                    .iter()
                    .filter_map(|(&key, &value)| {
                        if key.0 < pos.0 && key.1 < pos.1 {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, &y))
                    .0
            );

            // 区間取得3
            let pos2 = (rng.random_range(pos.0..=n), rng.random_range(pos.1..=n));
            assert_eq!(
                st.query(pos.0..pos2.0, pos.1..pos2.1).0,
                values
                    .iter()
                    .filter_map(|(&key, &value)| {
                        if pos.0 <= key.0 && key.0 < pos2.0 && pos.1 <= key.1 && key.1 < pos2.1 {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .fold(Minimum::id(), |x, y| Minimum::op(&x, &y))
                    .0
            );

            // 1点更新
            if pos_set.contains(&pos) {
                let val = dist_val.sample(&mut rng);

                st.set(pos.0, pos.1, val);
                values.insert(pos, Minimum(val));
            }
        }
    }

    #[test]
    fn test_segtree_beats() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        use std::cmp::*;

        #[derive(PartialEq, Eq)]
        struct RevOption<T>(Option<T>);

        impl<T> PartialOrd for RevOption<T>
        where
            T: PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match (&self.0, &other.0) {
                    (None, None) => Some(Ordering::Equal),
                    (None, Some(_)) => Some(Ordering::Greater),
                    (Some(_), None) => Some(Ordering::Less),
                    (Some(x), Some(y)) => x.partial_cmp(y),
                }
            }
        }

        impl<T> Ord for RevOption<T>
        where
            T: Ord,
        {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct Item {
            max_val: i64,
            max2_val: Option<i64>,
            num_max: i64,
            min_val: i64,
            min2_val: Option<i64>,
            num_min: i64,
            num: i64,
            sum: i64,
        }

        impl Item {
            fn new(val: i64) -> Self {
                Item {
                    max_val: val,
                    max2_val: None,
                    num_max: 1,
                    min_val: val,
                    min2_val: None,
                    num_min: 1,
                    num: 1,
                    sum: val,
                }
            }
        }

        impl Monoid for Item {
            fn id() -> Self {
                Self {
                    max_val: i64::MIN,
                    max2_val: None,
                    num_max: 0,
                    min_val: i64::MAX,
                    min2_val: None,
                    num_min: 0,
                    num: 0,
                    sum: 0,
                }
            }

            fn op(&self, rhs: &Self) -> Self {
                Self {
                    max_val: max(self.max_val, rhs.max_val),
                    max2_val: match self.max_val.cmp(&rhs.max_val) {
                        Ordering::Equal => max(self.max2_val, rhs.max2_val),
                        Ordering::Greater => max(self.max2_val, Some(rhs.max_val)),
                        Ordering::Less => max(Some(self.max_val), rhs.max2_val),
                    },
                    num_max: match self.max_val.cmp(&rhs.max_val) {
                        Ordering::Equal => self.num_max + rhs.num_max,
                        Ordering::Greater => self.num_max,
                        Ordering::Less => rhs.num_max,
                    },
                    min_val: min(self.min_val, rhs.min_val),
                    min2_val: match self.min_val.cmp(&rhs.min_val) {
                        Ordering::Equal => min(RevOption(self.min2_val), RevOption(rhs.min2_val)).0,
                        Ordering::Greater => {
                            min(RevOption(Some(self.min_val)), RevOption(rhs.min2_val)).0
                        }
                        Ordering::Less => {
                            min(RevOption(self.min2_val), RevOption(Some(rhs.min_val))).0
                        }
                    },
                    num_min: match self.min_val.cmp(&rhs.min_val) {
                        Ordering::Equal => self.num_min + rhs.num_min,
                        Ordering::Greater => rhs.num_min,
                        Ordering::Less => self.num_min,
                    },
                    num: self.num + rhs.num,
                    sum: self.sum + rhs.sum,
                }
            }
        }

        impl From<i64> for Item {
            fn from(value: i64) -> Self {
                Self::new(value)
            }
        }

        // add => chmin => chmax
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Update {
            add: i64,
            chmin: i64,
            chmax: i64,
        }

        impl Update {
            fn add(val: i64) -> Update {
                Update {
                    add: val,
                    chmin: i64::MAX,
                    chmax: i64::MIN,
                }
            }

            fn chmin(val: i64) -> Update {
                Update {
                    add: 0,
                    chmin: val,
                    chmax: i64::MIN,
                }
            }

            fn chmax(val: i64) -> Update {
                Update {
                    add: 0,
                    chmin: i64::MAX,
                    chmax: val,
                }
            }
        }

        impl Monoid for Update {
            fn id() -> Self {
                Update {
                    add: 0,
                    chmin: i64::MAX,
                    chmax: i64::MIN,
                }
            }

            fn op(&self, rhs: &Self) -> Self {
                Self {
                    add: self.add + rhs.add,
                    chmin: min(self.chmin, rhs.chmin.saturating_add(self.add)),
                    chmax: max(
                        self.chmax,
                        min(self.chmin, rhs.chmax.saturating_add(self.add)),
                    ),
                }
            }
        }

        impl TryOperator<Item> for Update {
            fn try_apply(&self, v: &Item) -> Option<Item> {
                let mut v = *v;

                // add
                v.max_val = v.max_val.saturating_add(self.add);
                v.max2_val = v.max2_val.map(|x| x.saturating_add(self.add));
                v.min_val = v.min_val.saturating_add(self.add);
                v.min2_val = v.min2_val.map(|x| x.saturating_add(self.add));
                v.sum += v.num * self.add;

                // chmin
                if matches!(v.max2_val, Some(x) if x >= self.chmin) {
                    return None;
                }
                if self.chmin <= v.min_val {
                    v.max_val = self.chmin;
                    v.max2_val = None;
                    v.num_max = v.num;
                    v.min_val = self.chmin;
                    v.min2_val = None;
                    v.num_min = v.num;
                    v.sum = v.num * self.chmin;
                } else {
                    v.sum -= (v.max_val.saturating_sub(min(v.max_val, self.chmin))) * v.num_max;
                    v.max_val = min(v.max_val, self.chmin);
                    v.min2_val = v.min2_val.map(|x| min(x, self.chmin));
                }

                // chmax
                if matches!(v.min2_val, Some(x) if x <= self.chmax) {
                    return None;
                }
                if self.chmax >= v.max_val {
                    v.max_val = self.chmax;
                    v.max2_val = None;
                    v.num_max = v.num;
                    v.min_val = self.chmax;
                    v.min2_val = None;
                    v.num_min = v.num;
                    v.sum = v.num * self.chmax;
                } else {
                    v.sum += (max(v.min_val, self.chmax).saturating_sub(v.min_val)) * v.num_min;
                    v.max2_val = v.max2_val.map(|x| max(x, self.chmax));
                    v.min_val = max(v.min_val, self.chmax);
                }

                Some(v)
            }
        }

        let mut rng = SmallRng::seed_from_u64(42);
        let n = 100000;
        const M: i64 = 10000;
        let mut v = std::iter::repeat_with(|| rng.random_range(-M..M))
            .take(n)
            .collect::<Vec<_>>();
        let mut st = SegmentTreeBeats::<Item, Update>::with(&v);

        for _ in 0..1000 {
            match rng.random_range(0..6) {
                // get
                0 => {
                    let idx = rng.random_range(0..n);
                    assert_eq!(st.get(idx), Item::new(v[idx]), "st={:?};\nv={:?}", st, v);
                }
                // set
                1 => {
                    let idx = rng.random_range(0..n);
                    let val = rng.random_range(0..M);
                    st.set(idx, val);
                    v[idx] = val;
                }
                // chmin
                2 => {
                    let idx0 = rng.random_range(0..n);
                    let idx1 = rng.random_range(0..n);
                    let (l, r) = (min(idx0, idx1), max(idx0, idx1) + 1);
                    let val = rng.random_range(0..M);
                    st.update(l..r, Update::chmin(val));
                    for i in l..r {
                        v[i] = min(v[i], val);
                    }
                }
                // chmax
                3 => {
                    let idx0 = rng.random_range(0..n);
                    let idx1 = rng.random_range(0..n);
                    let (l, r) = (min(idx0, idx1), max(idx0, idx1) + 1);
                    let val = rng.random_range(0..M);
                    st.update(l..r, Update::chmax(val));
                    for i in l..r {
                        v[i] = max(v[i], val);
                    }
                }
                // add
                4 => {
                    let idx0 = rng.random_range(0..n);
                    let idx1 = rng.random_range(0..n);
                    let (l, r) = (min(idx0, idx1), max(idx0, idx1) + 1);
                    let val = rng.random_range(0..M);
                    st.update(l..r, Update::add(val));
                    for i in l..r {
                        v[i] += val;
                    }
                }
                // sum
                5 => {
                    let idx0 = rng.random_range(0..n);
                    let idx1 = rng.random_range(0..n);
                    let (l, r) = (min(idx0, idx1), max(idx0, idx1) + 1);

                    assert_eq!(st.query(l..r).sum, v[l..r].iter().copied().sum::<i64>());
                }
                _ => unreachable!(),
            }
        }
    }
}
