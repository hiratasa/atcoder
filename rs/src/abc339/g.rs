use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        abc: [(usize, usize, usize); q]
    };

    let mut st = SparseSegmentTree2D::<Sum>::new(a.iter().copied().enumerate());
    for (i, x) in a.iter().copied().enumerate() {
        st.set(i, x, x);
    }

    let mut key = 0;
    for (a, b, c) in abc {
        let l = (a ^ key) - 1;
        let r = b ^ key;
        let x = c ^ key;

        let ans = st.query(l..r, 0..=x).0;

        println!("{ans}");

        key = ans;
    }
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
                .map(|l| (l.saturating_sub(k - 1) + k - 1) / k)
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
                    return Some(0);
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
macro_rules! define_monoid {
    ($ name : ident , $ t : ty , $ id : expr , $ op : expr ) => {
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

define_monoid!(Sum, usize, 0, std::ops::Add::add);
