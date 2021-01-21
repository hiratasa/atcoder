trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
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
        if left_idx < self.values.len() {
            self.values[idx] = M::op(
                &self.get_node_value(left_idx),
                &self.get_node_value(right_idx),
            );
        }
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
        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        // Opが非可換の場合用に, これより前にupdateされたものを適用させておく
        for i in (1..self.height).rev() {
            self.resolve(((left_idx + 1) >> i) - 1);
            self.resolve(((right_idx + 1) >> i) - 1);
        }

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                self.lazy[left_idx] = Op::op(&p, &self.lazy[left_idx]);
            }

            if right_idx % 2 == 0 {
                self.lazy[right_idx - 1] = Op::op(&p, &self.lazy[right_idx - 1]);
            }

            // 偶数の場合は一つ右隣の親になる
            left_idx = left_idx >> 1;
            right_idx = (right_idx - 1) >> 1;
        }

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;
        for _ in 0..self.height - 1 {
            left_idx = (left_idx - 1) >> 1;
            self.fix_value(left_idx);

            right_idx = (right_idx - 1) >> 1;
            self.fix_value(right_idx);
        }
    }

    fn query(&mut self, a: usize, b: usize) -> M::Item {
        let mut left = M::id();
        let mut right = M::id();

        let mut left_idx = a + self.cap - 1;
        let mut right_idx = b + self.cap - 1;

        let c0 = std::cmp::min(
            // trailing_ones
            (!left_idx).trailing_zeros(),
            (right_idx + 1).trailing_zeros(),
        ) as usize;

        for i in (c0 + 1..self.height).rev() {
            self.resolve(((left_idx + 1) >> i) - 1);
            self.resolve(((right_idx + 1) >> i) - 1);
        }

        left_idx = left_idx >> c0;
        right_idx = ((right_idx + 1) >> c0) - 1;

        while left_idx < right_idx {
            if left_idx % 2 == 0 {
                left = M::op(&left, &self.get_node_value(left_idx));
                left_idx += 1;
            }

            if right_idx % 2 == 0 {
                right = M::op(&self.get_node_value(right_idx - 1), &right);
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
}

const M: usize = 998244353;

#[derive(Debug)]
struct Sum;

impl Monoid for Sum {
    // (value, length)
    type Item = (usize, usize);

    fn id() -> Self::Item {
        (0, 0)
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        ((lhs.0 + rhs.0) % M, lhs.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Affine;

impl Monoid for Affine {
    type Item = (usize, usize);

    fn id() -> Self::Item {
        (1, 0)
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        // rhsからかける
        // lhs.0 * (rhs.0 * a + rhs.1) + lhs.1
        // = lhs.0 * rhs.0 * a + lhs.0 * rhs.1 + lhs.1
        (lhs.0 * rhs.0 % M, (lhs.0 * rhs.1 + lhs.1) % M)
    }
}

impl Operator<<Sum as Monoid>::Item> for Affine {
    fn apply(op: &Self::Item, v: &<Sum as Monoid>::Item) -> <Sum as Monoid>::Item {
        ((op.0 * v.0 + op.1 * v.1) % M, v.1)
    }
}

type ST = LazySegmentTree<Sum, Affine>;

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut read = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line.trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let nq = read();
    let a = read();

    let mut st = ST::with(&a.into_iter().map(|aa| (aa, 1)).collect());

    for _ in 0..nq[1] {
        let query = read();

        // eprintln!("{:?}", st);
        if query[0] == 0 {
            st.update(query[1], query[2], (query[3], query[4]));
        } else {
            let (s, _) = st.query(query[1], query[2]);
            println!("{}", s);
        }
    }
}
