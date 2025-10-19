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
use itertools::{Itertools, chain, iproduct, iterate, izip, repeat_n};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{Rng, SeedableRng, rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom};
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
    }
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
fn println_opt<T: Copy + std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
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

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
struct UnionFind {
    g: Vec<UnionFindNode>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;
        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
        }
    }
    fn root(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let p = match self.g[v] {
            Root { size: _ } => return v,
            Child { parent: p } => p,
        };
        let r = self.root(p);
        self.g[v] = Child { parent: r };
        r
    }
    fn unite(&mut self, v: usize, u: usize) -> bool {
        use UnionFindNode::*;
        let rv = self.root(v);
        let ru = self.root(u);
        if rv == ru {
            return false;
        }
        let size_rv = self.size(rv);
        let size_ru = self.size(ru);
        let (rsmall, rlarge) = if size_rv < size_ru {
            (rv, ru)
        } else {
            (ru, rv)
        };
        self.g[rsmall] = Child { parent: rlarge };
        self.g[rlarge] = Root {
            size: size_rv + size_ru,
        };
        true
    }
    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let rv = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _ } => unreachable!(),
        }
    }
}

fn main() {
    let (n, m, l) = read_tuple!(usize, usize, usize);
    let ab = read_vec(l, || read_tuple!(usize, usize));

    let mut uf = ab.citer().fold(UnionFind::new(n), |mut uf, (a, b)| {
        uf.unite(a - 1, b - 1);
        uf
    });

    let num_edges = ab.citer().fold(vec![0; n], |mut num_edges, (a, _b)| {
        num_edges[uf.root(a - 1)] += 1;
        num_edges
    });

    let k = (0..n)
        .map(|i| {
            if uf.root(i) == i && num_edges[i] == uf.size(i) - 1 {
                num_edges[i]
            } else {
                0
            }
        })
        .sum::<usize>();

    let cycles = (0..n)
        .filter_map(|i| {
            if uf.root(i) == i && num_edges[i] == uf.size(i) {
                Some(num_edges[i])
            } else {
                None
            }
        })
        .fold(vec![0; n + 1], |mut cycles, l| {
            cycles[l] += 1;
            cycles
        });

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct State {
        k: usize,
        cycles: Vec<usize>,
    }

    impl State {
        fn finished(&self) -> bool {
            self.k == 0 && self.cycles[2..].citer().all(|x| x == 0)
        }
    }

    let mut costs = FxHashMap::default();
    let mut q = BinaryHeap::new();

    let state0 = State { k, cycles };

    costs.insert(state0.clone(), (0, 0));
    q.push((Reverse((0, 0)), state0.clone()));

    let add = |(c, d): (usize, usize), e: usize| {
        assert!(e <= m);

        if d + e > m {
            if e == m { (c + 2, 0) } else { (c + 1, e) }
        } else if d + e == m {
            (c + 1, 0)
        } else {
            (c, d + e)
        }
    };

    let insert_if_can = |costs: &mut FxHashMap<State, (usize, usize)>,
                         q: &mut BinaryHeap<(Reverse<(usize, usize)>, State)>,
                         next_state: &State,
                         cost: (usize, usize),
                         add_cost: usize| {
        let next_cost = add(cost, add_cost);

        if next_cost
            < costs
                .get(&next_state)
                .copied()
                .unwrap_or((usize::MAX, usize::MAX))
        {
            costs.insert(next_state.clone(), next_cost);
            q.push((Reverse(next_cost), next_state.clone()));
        }
    };

    while let Some((Reverse(cost), state)) = q.pop() {
        if cost > *costs.get(&state).unwrap() {
            continue;
        }

        if state.finished() {
            let ans = (cost.0 + (cost.1 > 0) as usize) * 10;
            println!("{}", ans);
            return;
        }

        if state.k > 0 {
            let mut next_state = state.clone();
            next_state.k -= 1;

            insert_if_can(&mut costs, &mut q, &next_state, cost, 1);
        }

        state.cycles[2..]
            .citer()
            .positions(|x| x > 0)
            .for_each(|c| {
                let c = c + 2;
                if l < n {
                    // 荷物を一つ空いている席にうつしてcycleをばらす
                    let mut next_state = state.clone();
                    next_state.k += c;
                    next_state.cycles[c] -= 1;
                    insert_if_can(&mut costs, &mut q, &next_state, cost, 1);
                }

                for i in 2..=min(m, c) {
                    // cycleの一部or全てを解消
                    let mut next_state = state.clone();
                    next_state.cycles[c] -= 1;
                    next_state.cycles[c - (i - 1)] += 1;
                    insert_if_can(&mut costs, &mut q, &next_state, cost, i);
                }
            })
    }

    println!("-1");
}
