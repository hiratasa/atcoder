// NOTE: 区間長に依存する場合（たとえば一定の値を足す、など）
//  解決策1) 値を (value, len) にする
//  解決策2) fがlenも受け取れるようにする

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

    fn query(&self, mut a: usize, mut b: usize) -> M::Item {
        let mut left = M::id();
        {
            let mut idx = ((a + self.cap) >> (a + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (a + self.cap).trailing_zeros();
            while a + len <= b {
                left = M::op(&left, &self.values[idx]);
                a += len;
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;
            }
        }

        let mut right = M::id();
        {
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            while a + len <= b {
                right = M::op(&self.values[idx - 1], &right);
                b -= len;
                len <<= idx.trailing_zeros();
                idx = (idx >> idx.trailing_zeros()) - 1;
            }
        }

        M::op(&left, &right)
    }

    // f(query(a, b)) == false となるbが存在すればその最小のものを返す
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
            let mut idx = ((b + self.cap) >> (b + self.cap).trailing_zeros()) - 1;
            let mut len = 1 << (b + self.cap).trailing_zeros();
            let mut val = M::id();
            let mut val_next = M::op(&val, &self.values[idx]);

            while f(&val_next) {
                val = val_next;
                b += len;
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;

                // 最後に計算したidxが右端だった場合
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

            // [a, b)区間でfがtrue => 求めるbはその次
            Some(b + 1)
        }
    }
}

// M: Monoid of value
// Op: Monoid of lazy operation
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

trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

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
            st.fix_value(idx);
        }

        st
    }

    // internal
    fn get_node_value(&mut self, idx: usize) -> M::Item {
        Op::apply(&self.lazy[idx], &self.values[idx])
    }

    // internal
    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(
            &self.get_node_value(left_idx),
            &self.get_node_value(right_idx),
        );
    }

    // internal
    fn resolve(&mut self, parent_idx: usize) {
        let left_idx = 2 * (parent_idx + 1) - 1;
        let right_idx = 2 * (parent_idx + 1);

        if left_idx < self.values.len() {
            self.lazy[left_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[left_idx]);
            self.lazy[right_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[right_idx]);
            self.lazy[parent_idx] = Op::id();
            self.fix_value(parent_idx);
        } else {
            self.values[parent_idx] = Op::apply(&self.lazy[parent_idx], &self.values[parent_idx]);
            self.lazy[parent_idx] = Op::id();
        }
    }

    // internal
    fn resolve_all(&mut self, pos: usize) {
        let idx = self.cap - 1 + pos;
        for i in (0..self.height).rev() {
            self.resolve(((idx + 1) >> i) - 1);
        }
    }

    fn get(&mut self, pos: usize) -> M::Item {
        self.resolve_all(pos);

        let idx = self.cap - 1 + pos;
        self.values[idx].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        self.resolve_all(pos);

        let mut idx = self.cap - 1 + pos;
        self.values[idx] = v;
        self.lazy[idx] = Op::id();

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn update(&mut self, a: usize, b: usize, p: Op::Item) {
        let mut a = a;
        {
            let c = (a + self.cap).trailing_zeros() as usize;
            let mut idx = ((a + self.cap) >> c) - 1;

            // Opが非可換の場合用に, これより前にupdateされたものを適用させておく
            for i in (1..self.height - c).rev() {
                self.resolve(((idx + 1) >> i) - 1);
            }

            let mut len = 1 << c;
            for _ in c..self.height {
                if idx % 2 == 0 && a + len <= b {
                    self.lazy[idx] = Op::op(&p, &self.lazy[idx]);
                    a += len;
                }
                // 偶数の場合は一つ右隣の親になる
                idx = idx >> 1;
                if idx == 0 {
                    break;
                }
                self.fix_value(idx - 1);
                len <<= 1;
            }
        }

        let mut b = b;
        {
            let c = (b + self.cap).trailing_zeros() as usize;
            let mut idx = ((b + self.cap) >> c) - 1;

            for i in (1..self.height - c).rev() {
                self.resolve(((idx + 1) >> i) - 1);
            }

            let mut len = 1 << c;
            // 最上段に足す必要がある場合はaのほうで計算済みなのでその手前までで良い
            for _ in c..self.height - 1 {
                if idx % 2 == 0 && a + len <= b {
                    self.lazy[idx - 1] = Op::op(&p, &self.lazy[idx - 1]);
                    b -= len;
                }
                idx = (idx - 1) >> 1;
                self.fix_value(idx);
                len <<= 1;
            }
        }
    }

    fn query(&mut self, a: usize, b: usize) -> M::Item {
        let mut a = a;
        let mut left = M::id();
        {
            let c = (a + self.cap).trailing_zeros() as usize;
            let mut idx = ((a + self.cap) >> c) - 1;
            for i in (1..self.height - c).rev() {
                self.resolve(((idx + 1) >> i) - 1);
            }

            let mut len = 1 << c;
            while a + len <= b {
                left = M::op(&left, &self.get_node_value(idx));
                a += len;
                len <<= (idx + 2).trailing_zeros();
                idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;
            }
        }

        if a == b {
            return left;
        }

        let mut b = b;
        let mut right = M::id();
        {
            assert!(0 < b);
            let c = (b + self.cap).trailing_zeros() as usize;
            let mut idx = ((b + self.cap) >> c) - 1;
            for i in (1..self.height - c).rev() {
                self.resolve((idx >> i) - 1);
            }

            let mut len = 1 << c;
            while a + len <= b {
                right = M::op(&self.get_node_value(idx - 1), &right);
                b -= len;
                len <<= idx.trailing_zeros();
                idx = (idx >> idx.trailing_zeros()) - 1;
            }
        }

        M::op(&left, &right)
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

        let n = 5;

        let dist = rand::distributions::Uniform::new(0, 1000);
        let distidx = rand::distributions::Uniform::new(0, n);
        let dist01 = rand::distributions::Uniform::new(0, 2);

        for _ in 0..5 {
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

        let n = 5;

        let dist = rand::distributions::Uniform::new(0, 1000);
        let distidx = rand::distributions::Uniform::new(0, n);
        let dist01 = rand::distributions::Uniform::new(0, 2);

        for _ in 0..5 {
            let a0 = std::iter::repeat_with(|| dist.sample(&mut rng))
                .take(n)
                .collect();

            let mut st = RMTTreeWithAddition::with(&a0);
            let mut logs = vec![];

            let q = 1000;
            let mut a = a0.clone();
            for _ in 0..q {
                let t = dist01.sample(&mut rng);

                if t == 0 {
                    // update
                    let l = distidx.sample(&mut rng);
                    let distidx2 = rand::distributions::Uniform::new(l, n + 1);
                    let r = distidx2.sample(&mut rng);
                    let x = dist.sample(&mut rng);

                    st.update(l, r, x);
                    for i in l..r {
                        a[i] += x;
                    }

                    logs.push(format!("Add {} for [{}, {});", x, l, r));
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
