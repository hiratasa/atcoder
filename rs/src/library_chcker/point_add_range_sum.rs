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

    fn query(&self, a: usize, b: usize) -> M::Item {
        let mut idx0 = a + self.cap - 1;
        let mut idx1 = b + self.cap - 1;

        let mut left = M::id();
        let mut right = M::id();

        while idx0 < idx1 {
            if idx0 % 2 == 0 {
                left = M::op(&left, &self.values[idx0]);
                idx0 += 1;
            }

            if idx1 % 2 == 0 {
                right = M::op(&self.values[idx1 - 1], &right);
            }

            if idx0 == 0 {
                break;
            }

            idx0 = (idx0 - 1) / 2;
            idx1 = (idx1 - 1) / 2;
        }

        M::op(&left, &right)
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

define_monoid!(Sum, usize, 0, std::ops::Add::add);

type ST = SegmentTree<Sum>;

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

    let mut st = ST::with(&a);
    for _ in 0..nq[1] {
        let query = read();

        if query[0] == 0 {
            st.set(query[1], st.get(query[1]) + query[2]);
        } else {
            println!("{}", st.query(query[1], query[2]));
        }
    }
}
