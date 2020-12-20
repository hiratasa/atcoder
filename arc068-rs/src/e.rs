#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

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
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

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

impl<M, Op> LazySegmentTree<M, Op>
where
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn get(&mut self, pos: usize) -> M::Item {
        self.resolve(pos);

        let idx = self.cap - 1 + pos;
        self.values[idx].clone()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

impl Operator<usize> for Sum {
    fn apply(op: &Self::Item, v: &usize) -> usize {
        op + v
    }
}

#[allow(dead_code)]
type ST = LazySegmentTree<Sum, Sum>;

fn main() {
    let (n, m) = read_cols!(usize, usize);

    let lr = {
        let mut lr: Vec<_> = (0..n)
            .map(|_| read_cols!(usize, usize))
            .map(|(l, r)| (r - l, l, r))
            .collect();
        lr.sort();
        lr
    };

    let ans = (1..=m).scan((lr.iter().copied(), ST::new(m + 2)), |(it, st), d| {
        for (_, l, r) in it.clone().take_while(|&(dd, _, _)| dd < d) {
            it.next();
            st.update(l, r + 1, 1);
        }

        Some(
            (1..)
                .map(|i| i * d)
                .take_while(|&i| i <= m)
                .map(|i| st.get(i))
                .sum::<usize>()
                + it.len(),
        )
    });

    for a in ans {
        println!("{}", a);
    }
}
