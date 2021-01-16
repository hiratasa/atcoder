// NOTE: 区間長に依存する場合（たとえば一定の値を足す、など）
//  解決策1) 値を (value, len) にする
//  解決策2) fがlenも受け取れるようにする

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

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
        let mut c = (a + self.cap).trailing_zeros();
        let mut idx = ((a + self.cap) >> c) - 1;
        let mut left = M::id();

        while a + (1 << c) <= b {
            left = M::op(&left, &self.values[idx]);
            a += 1 << c;
            c += (idx + 2).trailing_zeros();
            idx = ((idx + 2) >> (idx + 2).trailing_zeros()) - 1;
        }

        let mut d = (b + self.cap).trailing_zeros();
        let mut idx2 = ((b + self.cap) >> d) - 1;
        let mut right = M::id();
        while a + (1 << d) <= b {
            right = M::op(&self.values[idx2 - 1], &right);
            b -= 1 << d;
            d += idx2.trailing_zeros();
            idx2 = (idx2 >> idx2.trailing_zeros()) - 1;
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
            self.right_partition_point_impl(a, &mut f, 0, 0, self.cap, &M::id())
                .ok()
        }
    }

    fn right_partition_point_impl<F>(
        &self,
        a: usize,
        f: &mut F,
        idx: usize,
        l: usize,
        r: usize,
        carry: &M::Item,
    ) -> Result<usize, M::Item>
    where
        F: FnMut(&M::Item) -> bool,
    {
        // precondition
        assert!(a < r);
        assert!(f(carry));
        // assert!(l < a || carry == query(a, l))

        // postcondition
        // when return Ok, f(query(a, ok_value - 1)) && !f(query(a, ok_value))
        // when return Err, err_value == query(a, r) && f(err_value)

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        let mid = (l + r) / 2;

        if a <= l {
            let v = M::op(carry, &self.values[idx]);
            if f(&v) {
                Err(v.clone())
            } else {
                if l + 1 == r {
                    // leaf
                    Ok(r)
                } else {
                    match self.right_partition_point_impl(a, f, left_idx, l, mid, carry) {
                        Ok(found) => Ok(found),
                        // In this branch, expected to be always return Ok
                        Err(q) => self.right_partition_point_impl(a, f, right_idx, mid, r, &q),
                    }
                }
            }
        } else if a < mid {
            match self.right_partition_point_impl(a, f, left_idx, l, mid, &M::id()) {
                Ok(found) => Ok(found),
                Err(q) => self.right_partition_point_impl(a, f, right_idx, mid, r, &q),
            }
        } else if a < r {
            self.right_partition_point_impl(a, f, right_idx, mid, r, &M::id())
        } else {
            unreachable!()
        }
    }
}

// M: Monoid of value
// Op: Monoid of lazy operation
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
            height: cap.trailing_zeros() as usize,
            cap,
            values: vec![M::id(); 2 * cap - 1],
            lazy: vec![Op::id(); 2 * cap - 1],
        }
    }

    fn get_node_value(&mut self, idx: usize) -> M::Item {
        Op::apply(&self.lazy[idx], &self.values[idx])
    }

    fn fix_value(&mut self, idx: usize) {
        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);
        self.values[idx] = M::op(
            &self.get_node_value(left_idx),
            &self.get_node_value(right_idx),
        );
    }

    fn resolve(&mut self, pos: usize) {
        let idx = self.cap - 1 + pos;
        for i in (1..self.height).rev() {
            let parent_idx = ((idx + 1) >> i) - 1;

            let left_idx = 2 * (parent_idx + 1) - 1;
            let right_idx = 2 * (parent_idx + 1);

            self.lazy[left_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[left_idx]);
            self.lazy[right_idx] = Op::op(&self.lazy[parent_idx], &self.lazy[right_idx]);
            self.lazy[parent_idx] = Op::id();

            self.fix_value(parent_idx);
        }

        self.values[idx] = Op::apply(&self.lazy[idx], &self.values[idx]);
        self.lazy[idx] = Op::id();
    }

    fn get(&mut self, pos: usize) -> M::Item {
        self.resolve(pos);

        let idx = self.cap - 1 + pos;
        self.values[idx].clone()
    }

    fn set(&mut self, pos: usize, v: M::Item) {
        self.resolve(pos);

        let mut idx = self.cap - 1 + pos;
        self.values[idx] = v;
        self.lazy[idx] = Op::id();

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.fix_value(idx);
        }
    }

    fn update(&mut self, a: usize, b: usize, p: Op::Item) {
        self.update_impl(a, b, p, 0, 0, self.cap);
    }

    fn update_impl(&mut self, a: usize, b: usize, p: Op::Item, idx: usize, l: usize, r: usize) {
        if a >= r || b <= l {
            // no overlap
            return;
        }

        if a <= l && r <= b {
            self.lazy[idx] = Op::op(&p, &self.lazy[idx]);
            return;
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);

        // モノイドOpが可換でない場合、pの適用前にlazy[idx]の適用が必要
        self.lazy[left_idx] = Op::op(&self.lazy[idx], &self.lazy[left_idx]);
        self.lazy[right_idx] = Op::op(&self.lazy[idx], &self.lazy[right_idx]);
        self.lazy[idx] = Op::id();

        self.update_impl(a, b, p.clone(), left_idx, l, (l + r) / 2);
        self.update_impl(a, b, p.clone(), right_idx, (l + r) / 2, r);

        self.fix_value(idx);
    }

    fn query(&mut self, a: usize, b: usize) -> M::Item {
        self.query_impl(a, b, 0, 0, self.cap)
    }

    fn query_impl(&mut self, a: usize, b: usize, idx: usize, l: usize, r: usize) -> M::Item {
        if a >= r || b <= l {
            // no overlap
            return M::id();
        }

        if a <= l && r <= b {
            return self.get_node_value(idx);
        }

        let left_idx = 2 * (idx + 1) - 1;
        let right_idx = 2 * (idx + 1);

        let left_v = self.query_impl(a, b, left_idx, l, (l + r) / 2);
        let right_v = self.query_impl(a, b, right_idx, (l + r) / 2, r);

        Op::apply(&self.lazy[idx], &M::op(&left_v, &right_v))
    }
}

macro_rules! define_monoid {
    ($name: ident, $t: ty, $id: expr, $op: expr) => {
        #[derive(Clone)]
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
    fn test_lazy_segtree() {
        let mut st = RMTTreeWithAddition::new(10);

        st.set(1, 2);
        st.set(3, 4);

        assert_eq!(st.query(1, 4), 2);
        assert_eq!(st.query(2, 4), 4);
        assert_eq!(st.query(3, 3), Minimum::id());
        assert_eq!(st.query(4, 5), Minimum::id());

        st.update(1, 5, 2);

        assert_eq!(st.query(1, 4), 4);
        assert_eq!(st.query(2, 4), 6);
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
