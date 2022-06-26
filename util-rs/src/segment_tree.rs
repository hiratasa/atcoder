use cargo_snippet::snippet;

// NOTE: 区間長に依存する場合（たとえば一定の値を足す、など）
//  解決策1) 値を (value, len) にする
//  解決策2) fがlenも受け取れるようにする

#[snippet("segtree")]
#[snippet("lazysegtree")]
trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

#[snippet("segtree")]
#[derive(Debug)]
struct SegmentTree<M>
where
    M: Monoid,
{
    cap: usize,
    values: Vec<M::Item>,
}

#[snippet("segtree")]
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

    fn get(&self, pos: usize) -> M::Item {
        self.values[self.cap - 1 + pos].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        let idx = self.cap - 1 + pos;

        self.values[idx] = v;

        self.fix_all(idx);
    }

    fn query(&self, a: usize, b: usize) -> M::Item {
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

// M: Monoid of value
// Op: Monoid of lazy operation
#[snippet("lazysegtree")]
#[derive(Debug)]
struct LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    height: usize,
    cap: usize,
    values: Vec<M::Item>,
    lazy: Vec<Op::Item>,
}

#[snippet("lazysegtree")]
trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

#[snippet("lazysegtree")]
#[allow(dead_code)]
impl<M, Op> LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new(n: usize) -> Self {
        let cap = n.next_power_of_two();
        LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }

    fn with(vals: &Vec<M::Item>) -> Self {
        let n = vals.len();
        let cap = n.next_power_of_two();

        let mut values = Vec::with_capacity(2 * cap - 1);
        values.resize(cap - 1, M::id());
        values.extend(vals.iter().cloned());
        values.resize(2 * cap - 1, M::id());

        let mut st = LazySegmentTree {
            height: cap.trailing_zeros() as usize + 1,
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
    fn apply(&mut self, idx: usize, p: &Op::Item) {
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

    fn get(&mut self, pos: usize) -> M::Item {
        let idx = self.cap - 1 + pos;

        self.push_all(idx);

        self.values[idx].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        let idx = self.cap - 1 + pos;

        self.push_all(idx);

        self.values[idx] = v;
        self.lazy[idx] = Op::id();

        self.fix_all(idx);
    }

    fn update(&mut self, a: usize, b: usize, p: Op::Item) {
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

    fn query(&mut self, a: usize, b: usize) -> M::Item {
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

#[snippet("segtree")]
#[snippet("lazysegtree")]
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

define_monoid!(Minimum, i64, 1 << 60, i64::min);
define_monoid!(AddValue, i64, 0, std::ops::Add::add);

impl Operator<i64> for AddValue {
    fn apply(op: &Self::Item, v: &i64) -> i64 {
        op + v
    }
}

#[allow(dead_code)]
type RMTTree = SegmentTree<Minimum>;

#[allow(dead_code)]
type RMTTreeWithAddition = LazySegmentTree<Minimum, AddValue>;

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_segtree() {
        let mut st = RMTTree::new(10);

        st.set(1, 2);
        st.set(3, 4);
        st.set(5, 8);

        assert_eq!(st.query(0, 10), 2);
        assert_eq!(st.query(1, 4), 2);
        assert_eq!(st.query(2, 4), 4);
        assert_eq!(st.query(3, 3), Minimum::id());
        assert_eq!(st.query(4, 5), Minimum::id());
    }

    #[test]
    fn test_segtree_random() {
        use rand::distributions::Distribution;
        use rand::SeedableRng;

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let dist = rand::distributions::Uniform::new(0, 1000);
        let dist01 = rand::distributions::Uniform::new(0, 2);

        for i in 0..5 {
            // 2の冪乗のあたりを試す
            let n = 510 + i;
            let distidx = rand::distributions::Uniform::new(0, n);

            let a0 = std::iter::repeat_with(|| dist.sample(&mut rng))
                .take(n)
                .collect();

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
                    let distidx2 = rand::distributions::Uniform::new(l, n + 1);
                    let r = distidx2.sample(&mut rng);

                    assert_eq!(
                        st.query(l, r),
                        a[l..r].iter().copied().min().unwrap_or(Minimum::id()),
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

        assert_eq!(st.query(1, 4), 2);
        assert_eq!(st.query(2, 4), 4);
        assert_eq!(st.query(3, 3), Minimum::id());
        assert_eq!(st.query(4, 5), Minimum::id());

        st.update(1, 5, 2);

        // [inf, 4, inf, 6, ...]

        assert_eq!(st.query(1, 4), 4);
        assert_eq!(st.query(2, 4), 6);
    }

    #[test]
    fn test_lazy_segtree_random() {
        use rand::distributions::Distribution;
        use rand::SeedableRng;

        let mut rng = rand::rngs::SmallRng::from_entropy();

        let dist = rand::distributions::Uniform::new(0, 100000);
        let dist_op = rand::distributions::Uniform::new(0, 4);

        for i in 0..5 {
            // 2の冪乗のあたりを試す
            let n = 510 + i;
            let distidx = rand::distributions::Uniform::new(0, n + 1);

            let a0 = std::iter::repeat_with(|| dist.sample(&mut rng))
                .take(n)
                .collect();

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

                    st.update(l, r, x);
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
                        st.query(l, r),
                        a[l..r].iter().copied().min().unwrap_or(Minimum::id()),
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
                        st.get(pos),
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
            let mins = std::iter::once((i, Minimum::id()))
                .chain(vals.iter().copied().enumerate().skip(i).scan(
                    Minimum::id(),
                    |min, (j, v)| {
                        *min = std::cmp::min(*min, v);
                        // minimum in [i, j] = [i, j + 1)
                        Some((j + 1, *min))
                    },
                ))
                .collect::<Vec<_>>();
            for v in 0..20 {
                assert_eq!(
                    /* actual */ st.right_partition_point(i, |&u| u >= v),
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
}
