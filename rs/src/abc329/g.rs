#[allow(unused_imports)]
use std::{cmp::*, collections::*, f64, i64, io, iter::*, mem::*, str::*, usize};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, repeat_n, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rand::{rngs::SmallRng, seq::IteratorRandom, seq::SliceRandom, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

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
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        if $n > 0 {
            bs.buffer_mut()[0] = $x as u64;
        }
        bs
    }};
}

#[allow(dead_code)]
fn println_opt<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

mod detail {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug)]
    pub struct Edge<W = ()>
    where
        W: Copy,
    {
        pub from: usize,
        pub to: usize,
        pub label: W,
    }
    #[allow(dead_code)]
    impl<W> Edge<W>
    where
        W: Copy,
    {
        pub fn new(from: usize, to: usize) -> Self
        where
            W: Default,
        {
            Self {
                from,
                to,
                label: W::default(),
            }
        }
        pub fn new_with_label(from: usize, to: usize, label: W) -> Self {
            Self { from, to, label }
        }
        pub fn rev(&self) -> Self {
            Self {
                from: self.to,
                to: self.from,
                ..*self
            }
        }
        pub fn offset1(&self) -> Self {
            Self {
                from: self.from - 1,
                to: self.to - 1,
                ..*self
            }
        }
    }
    type Weight = usize;
    pub type UnweightedEdge = Edge<()>;
    pub type WeightedEdge = Edge<Weight>;
    impl std::convert::From<(usize, usize)> for UnweightedEdge {
        fn from(t: (usize, usize)) -> Self {
            UnweightedEdge::new(t.0, t.1)
        }
    }
    impl std::convert::From<&(usize, usize)> for UnweightedEdge {
        fn from(t: &(usize, usize)) -> Self {
            Edge::from(*t)
        }
    }
    impl std::convert::From<(usize, usize, Weight)> for WeightedEdge {
        fn from(t: (usize, usize, Weight)) -> Self {
            Edge::new_with_label(t.0, t.1, t.2)
        }
    }
    impl std::convert::From<&(usize, usize, Weight)> for WeightedEdge {
        fn from(t: &(usize, usize, Weight)) -> Self {
            Edge::from(*t)
        }
    }
    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    pub struct Graph<W = ()>
    where
        W: Copy,
    {
        pub out_edges: Vec<Vec<Edge<W>>>,
        pub in_edges: Vec<Vec<Edge<W>>>,
    }
    #[allow(dead_code)]
    pub type UnweightedGraph = Graph<()>;
    #[allow(dead_code)]
    pub type WeightedGraph = Graph<Weight>;
    #[allow(dead_code)]
    impl<W: Copy> Graph<W> {
        pub fn new(n: usize) -> Self {
            Self {
                out_edges: vec![vec![]; n],
                in_edges: vec![vec![]; n],
            }
        }
        pub fn from_edges_directed<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            let mut g = Graph::new(n);
            for edge in edges {
                let e = edge.into();
                g.add_edge(e);
            }
            g
        }
        pub fn from_edges1_directed<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges_directed(n, edges.into_iter().map(|e| e.into()).map(|e| e.offset1()))
        }
        pub fn from_edges_undirected<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges_directed(
                n,
                edges
                    .into_iter()
                    .map(|e| e.into())
                    .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
            )
        }
        pub fn from_edges1_undirected<T, I>(n: usize, edges: I) -> Self
        where
            I: IntoIterator<Item = T>,
            T: std::convert::Into<Edge<W>>,
        {
            Graph::from_edges1_directed(
                n,
                edges
                    .into_iter()
                    .map(|e| e.into())
                    .flat_map(|e| std::iter::once(e).chain(std::iter::once(e.rev()))),
            )
        }
        pub fn size(&self) -> usize {
            self.out_edges.len()
        }
        pub fn add_edge<T>(&mut self, e: T)
        where
            Edge<W>: std::convert::From<T>,
        {
            let edge = Edge::from(e);
            self.out_edges[edge.from].push(edge);
            self.in_edges[edge.to].push(edge);
        }
        pub fn adjs<'a>(&'a self, v: usize) -> impl 'a + DoubleEndedIterator<Item = usize> {
            self.out_edges[v].iter().map(|e| e.to)
        }
        pub fn children<'a>(
            &'a self,
            v: usize,
            p: usize,
        ) -> impl 'a + DoubleEndedIterator<Item = usize> {
            self.adjs(v).filter(move |&u| u != p)
        }
        pub fn children_edge<'a>(
            &'a self,
            v: usize,
            p: usize,
        ) -> impl 'a + DoubleEndedIterator<Item = Edge<W>> {
            self.out_edges[v].iter().copied().filter(move |e| e.to != p)
        }
    }
}

type Graph = detail::UnweightedGraph;

use num::{One, Zero};
#[allow(dead_code)]
pub fn pow_mod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut y = 1;
    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }
        x = x * x % m;
        p >>= 1;
    }
    y
}
pub trait Modulus: Copy + Eq {
    fn modulus() -> usize;
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StaticModulus<const M: usize>();
impl<const M: usize> Modulus for StaticModulus<M> {
    fn modulus() -> usize {
        M
    }
}
macro_rules! define_static_mod {
    ($ m : expr , $ mod : ident ) => {
        #[allow(dead_code)]
        pub type $mod = Mod<StaticModulus<$m>>;
    };
}
define_static_mod!(2013265921, Mod2013265921);
define_static_mod!(1811939329, Mod1811939329);
define_static_mod!(469762049, Mod469762049);
define_static_mod!(998244353, Mod998244353);
define_static_mod!(1224736769, Mod1224736769);
define_static_mod!(1000000007, Mod1000000007);
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mod<M>(pub usize, std::marker::PhantomData<fn() -> M>);
#[allow(dead_code)]
impl<M: Modulus> Mod<M> {
    pub fn modulus() -> usize {
        M::modulus()
    }
    pub fn new(n: usize) -> Self {
        Mod(n % M::modulus(), std::marker::PhantomData)
    }
    pub fn raw(n: usize) -> Self {
        Mod(n, std::marker::PhantomData)
    }
    pub fn pow(self, p: usize) -> Self {
        Mod::new(pow_mod(self.0, p, M::modulus()))
    }
    pub fn inv(self) -> Self {
        let (_zero, g, _u, v) = std::iter::successors(
            Some((self.0 as i64, M::modulus() as i64, 1, 0)),
            |&(a, b, u, v)| {
                if a == 0 {
                    None
                } else {
                    Some((b % a, a, -u * (b / a) + v, u))
                }
            },
        )
        .last()
        .unwrap();
        assert_eq!(
            g,
            1,
            "gcd({}, {}) must be 1 but {}.",
            self.0,
            M::modulus(),
            g
        );
        Mod::new((v + M::modulus() as i64) as usize)
    }
}
impl<M> std::fmt::Display for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<M> std::fmt::Debug for Mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T, M: Modulus> std::convert::From<T> for Mod<M>
where
    usize: std::convert::TryFrom<T>,
    T: num::traits::Unsigned,
{
    fn from(v: T) -> Self {
        Mod::new(usize::try_from(v).ok().unwrap())
    }
}
impl<M: Modulus> std::str::FromStr for Mod<M> {
    type Err = <usize as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s).map(|n| Mod::new(n))
    }
}
impl<M: Modulus> std::ops::Neg for Mod<M> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            Mod::raw(0)
        } else {
            Mod::raw(M::modulus() - self.0)
        }
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: Mod<M>) -> Self {
        let r = self.0 + rhs.0;
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            assert!(r - M::modulus() < M::modulus());
            Mod::raw(r - M::modulus())
        }
    }
}
impl<M: Modulus> std::ops::Add<usize> for Mod<M> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Add<Mod<M>> for usize {
    type Output = Mod<M>;
    fn add(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) + rhs.0
    }
}
impl<T, M: Modulus> std::ops::AddAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Add<T, Output = Mod<M>>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: Mod<M>) -> Self {
        let r = self.0.wrapping_sub(rhs.0);
        if r < M::modulus() {
            Mod::raw(r)
        } else {
            Mod::raw(r.wrapping_add(M::modulus()))
        }
    }
}
impl<M: Modulus> std::ops::Sub<usize> for Mod<M> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Mod::new(rhs)
    }
}
impl<M: Modulus> std::ops::Sub<Mod<M>> for usize {
    type Output = Mod<M>;
    fn sub(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new(self) - rhs
    }
}
impl<T, M: Modulus> std::ops::SubAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Sub<T, Output = Mod<M>>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}
impl<M: Modulus> std::ops::Mul<Mod<M>> for Mod<M> {
    type Output = Self;
    fn mul(self, rhs: Mod<M>) -> Self {
        Mod::new(self.0 * rhs.0)
    }
}
impl<M: Modulus> std::ops::Mul<usize> for Mod<M> {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Mod::new(self.0 * (rhs % M::modulus()))
    }
}
impl<M: Modulus> std::ops::Mul<Mod<M>> for usize {
    type Output = Mod<M>;
    fn mul(self, rhs: Mod<M>) -> Mod<M> {
        Mod::new((self % M::modulus()) * rhs.0)
    }
}
impl<T, M: Modulus> std::ops::MulAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Mul<T, Output = Mod<M>>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<M: Modulus> std::ops::Div<Mod<M>> for Mod<M> {
    type Output = Self;
    fn div(self, rhs: Mod<M>) -> Self {
        assert!(!rhs.is_zero());
        if self.0 == 0 {
            self
        } else {
            self * rhs.inv()
        }
    }
}
impl<M: Modulus> std::ops::Div<usize> for Mod<M> {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        assert_ne!(rhs, 0);
        if self.0 == 0 {
            self
        } else {
            self * Mod::new(rhs).inv()
        }
    }
}
impl<M: Modulus> std::ops::Div<Mod<M>> for usize {
    type Output = Mod<M>;
    fn div(self, rhs: Mod<M>) -> Mod<M> {
        assert!(!rhs.is_zero());
        if self == 0 {
            Mod::new(self)
        } else {
            self * rhs.inv()
        }
    }
}
impl<T, M: Modulus> std::ops::DivAssign<T> for Mod<M>
where
    Mod<M>: std::ops::Div<T, Output = Mod<M>>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
impl<M: Modulus> std::iter::Product for Mod<M> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::one(), |p, a| p * a)
    }
}
impl<M: Modulus> std::iter::Sum for Mod<M> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Mod::zero(), |p, a| p + a)
    }
}
impl<M: Modulus> num::Zero for Mod<M> {
    fn zero() -> Self {
        Mod::new(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl<M: Modulus> num::One for Mod<M> {
    fn one() -> Self {
        Mod::new(1)
    }
    fn is_one(&self) -> bool {
        self.0 == 1
    }
}
impl<M: Modulus> rand::distr::Distribution<Mod<M>> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mod<M> {
        Mod::new(rng.random_range(0..M::modulus()))
    }
}

fn calc_depth(g: &Graph, v: usize, d: usize, depths: &mut [usize]) {
    depths[v] = d;

    g.adjs(v).for_each(|u| calc_depth(g, u, d + 1, depths));
}

fn solve0(g: &Graph, k: usize, st: &[(usize, usize)]) -> Mod998244353 {
    fn dfs(g: &Graph, v: usize, tours: &mut Vec<Vec<(usize, usize)>>) {
        let l = g.out_edges[v].len();

        for tt in tours.iter_mut() {
            tt.push((v, 0));
        }

        *tours = g
            .adjs(v)
            .permutations(l)
            .flat_map(|perm| {
                let mut t = tours.clone();

                for (i, u) in perm.citer().enumerate() {
                    dfs(g, u, &mut t);
                    for tt in t.iter_mut() {
                        tt.push((v, i + 1));
                    }
                }

                t
            })
            .collect::<Vec<_>>();
    }

    fn check(
        g: &Graph,
        k: usize,
        tour: &[(usize, usize)],
        st: &[(usize, usize)],
        starts: &[Vec<usize>],
        terminates: &[Vec<usize>],
        lca: impl Fn(usize, usize) -> (usize, Option<usize>),
    ) -> bool {
        let m = st.len();
        tour.citer()
            .enumerate()
            .try_fold(
                (vec![false; m], vec![false; m], 0usize, 0usize),
                |(mut current, mut finished, mut num, mut num_finished),
                 (i_v, (v, nth_visited))| {
                    for idx in terminates[v].citer() {
                        if current[idx] {
                            assert!(!finished[idx]);
                            current[idx] = false;
                            finished[idx] = true;
                            num -= 1;
                            num_finished += 1;
                        }
                    }
                    for idx in starts[v].citer() {
                        if nth_visited == g.out_edges[v].len() {
                            if !finished[idx] {
                                assert!(!current[idx]);
                                current[idx] = true;
                                num += 1;
                            }
                        } else {
                            let (l, target_ch) = lca(st[idx].0, st[idx].1);
                            if l == v {
                                let target_ch = target_ch.unwrap();
                                if i_v + 1 < tour.len() && tour[i_v + 1].0 == target_ch {
                                    assert!(!current[idx]);
                                    current[idx] = true;
                                    num += 1;
                                }
                            }
                        }
                    }

                    if num > k {
                        return None;
                    }
                    Some((current, finished, num, num_finished))
                },
            )
            .filter(|(_current, _finished, num, num_finished)| *num == 0 && *num_finished == m)
            .is_some()
    }

    let n = g.size();

    const K: usize = 20;
    let mut parents = vec![vec![0; n]; K];
    for i in 1..n {
        parents[0][i] = g.in_edges[i][0].from;
    }
    for i in 0..K - 1 {
        for j in 0..n {
            parents[i + 1][j] = parents[i][parents[i][j]];
        }
    }

    let mut depths = vec![0; n];
    calc_depth(&g, 0, 0, &mut depths);

    let lca = |v: usize, u: usize| {
        let (mut v, mut u, swapped) = if depths[v] < depths[u] {
            (v, u, false)
        } else {
            (u, v, true)
        };

        for i in (0..K).rev() {
            if depths[v] < depths[parents[i][u]] {
                u = parents[i][u];
            }
        }

        let uu = if depths[v] < depths[u] {
            let uu = u;
            u = parents[0][u];
            Some(uu)
        } else {
            None
        };
        assert!(depths[v] == depths[u]);

        if v == u {
            return (v, uu);
        }

        for i in (0..K).rev() {
            if parents[i][v] != parents[i][u] {
                v = parents[i][v];
                u = parents[i][u];
            }
        }

        if swapped {
            (parents[0][v], Some(v))
        } else {
            (parents[0][v], Some(u))
        }
    };

    let mut tours = vec![vec![]];
    dfs(&g, 0, &mut tours);

    let starts = st
        .citer()
        .enumerate()
        .fold(vec![vec![]; n], |mut starts, (i, (s, _))| {
            starts[s].push(i);
            starts
        });
    let terminates = st
        .citer()
        .enumerate()
        .fold(vec![vec![]; n], |mut terminates, (i, (_, t))| {
            terminates[t].push(i);
            terminates
        });

    Mod::new(
        tours
            .iter()
            .filter(|tour| check(g, k, tour, &st, &starts, &terminates, &lca))
            .count(),
    )
}

fn solve(
    g: &Graph,
    v: usize,
    pre_ins: &[i64],
    ins: &[i64],
    outs: &[i64],
    post_outs: &[i64],
    children_order: &[Option<usize>],
    k: usize,
) -> (Vec<Mod998244353>, i64) {
    let mut chs = g
        .adjs(v)
        .map(|u| solve(g, u, pre_ins, ins, outs, post_outs, children_order, k))
        .collect::<Vec<_>>();

    match chs.len() {
        0 => {
            let kk = max(0i64, max(pre_ins[v], pre_ins[v] + ins[v] + outs[v])) as usize;

            let mut r = vec![Mod::zero(); kk + 1];
            r[kk] = Mod::one();

            r.truncate(k + 1);

            (r, pre_ins[v] + ins[v] + outs[v] + post_outs[v])
        }
        1 => {
            let (r0, t0) = take(&mut chs[0]);
            let ll = pre_ins[v] + ins[v] + t0 + outs[v];

            assert!(r0.len() == k + 1 || t0 <= r0.len() as i64 - 1);

            let mut r = vec![
                Mod::zero();
                max(
                    max(0, pre_ins[v]),
                    max(pre_ins[v] + ins[v] + (r0.len() - 1) as i64, ll)
                ) as usize
                    + 1
            ];
            for (i, x) in r0.citer().enumerate() {
                let idx = max(max(0, pre_ins[v]), max(pre_ins[v] + ins[v] + i as i64, ll)) as usize;
                r[idx] = r[idx] + x;
            }

            r.truncate(k + 1);

            (r, ll + post_outs[v])
        }
        2 => {
            let (mut r0, t0) = take(&mut chs[0]);
            let (mut r1, t1) = take(&mut chs[1]);
            let child0 = g.out_edges[v][0].to;
            let child1 = g.out_edges[v][1].to;

            let order = children_order[v];
            let ll = pre_ins[v] + ins[v] + t0 + t1 + outs[v];

            let mut last = Mod::zero();
            for x in r0.iter_mut() {
                *x += last;
                last = *x;
            }
            r0.insert(0, Mod::zero());

            let mut last = Mod::zero();
            for x in r1.iter_mut() {
                *x += last;
                last = *x;
            }
            r1.insert(0, Mod::zero());

            assert!(
                matches!(order, None) || matches!(order, Some(c) if c == child0 || c == child1)
            );

            let mut r01 = vec![];

            if order.is_none() || order == Some(child0) {
                let l = max(
                    max(0, pre_ins[v]),
                    max(
                        max(
                            pre_ins[v] + ins[v] + r0.len() as i64 - 1,
                            pre_ins[v] + ins[v] + t0 + r1.len() as i64 - 1,
                        ),
                        ll,
                    ),
                ) as usize;
                r01.resize(l + 1, Mod::zero());

                for i in 0..=l {
                    let ii = i as i64;
                    if ii < pre_ins[v] || ii < ll {
                        continue;
                    }

                    let j0 = min(r0.len() - 1, max(0, ii + 1 - pre_ins[v] - ins[v]) as usize);
                    let j1 = min(
                        r1.len() - 1,
                        max(0, ii + 1 - t0 - pre_ins[v] - ins[v]) as usize,
                    );

                    r01[i] = r0[j0] * r1[j1];
                }

                for i in (1..=l).rev() {
                    r01[i] = r01[i] - r01[i - 1];
                }
            }

            let mut r10 = vec![];
            if order.is_none() || order == Some(child1) {
                let l = max(
                    max(0, pre_ins[v]),
                    max(
                        max(
                            pre_ins[v] + ins[v] + r1.len() as i64 - 1,
                            pre_ins[v] + ins[v] + t1 + r0.len() as i64 - 1,
                        ),
                        ll,
                    ),
                ) as usize;
                r10.resize(l + 1, Mod::zero());

                for i in 0..=l {
                    let ii = i as i64;
                    if ii < pre_ins[v] || ii < ll {
                        continue;
                    }

                    let j1 = min(r1.len() - 1, max(0, ii + 1 - pre_ins[v] - ins[v]) as usize);
                    let j0 = min(
                        r0.len() - 1,
                        max(0, ii + 1 - t1 - pre_ins[v] - ins[v]) as usize,
                    );

                    r10[i] = r10[i] + r1[j1] * r0[j0];
                }

                for i in (1..=l).rev() {
                    r10[i] = r10[i] - r10[i - 1];
                }
            }

            let mut r = vec![Mod::zero(); min(k + 1, max(r01.len(), r10.len()))];
            let l = r.len();
            for i in 0..l {
                r[i] = r01.get(i).copied().unwrap_or_default()
                    + r10.get(i).copied().unwrap_or_default();
            }

            (r, ll + post_outs[v])
        }
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        p: [Usize1; n - 1],
        st: [(Usize1, Usize1); m],
    };

    let g = Graph::from_edges_directed(n, p.citer().enumerate().map(|(i, pp)| (pp, i + 1)));

    const K: usize = 20;
    let mut parents = vec![vec![0; n]; K];
    for (i, pp) in p.citer().enumerate() {
        parents[0][i + 1] = pp;
    }
    for i in 0..K - 1 {
        for j in 0..n {
            parents[i + 1][j] = parents[i][parents[i][j]];
        }
    }

    let mut depths = vec![0; n];
    calc_depth(&g, 0, 0, &mut depths);

    let lca = |v: usize, u: usize| {
        let (mut v, mut u, swapped) = if depths[v] < depths[u] {
            (v, u, false)
        } else {
            (u, v, true)
        };

        for i in (0..K).rev() {
            if depths[v] < depths[parents[i][u]] {
                u = parents[i][u];
            }
        }
        let ch = if depths[v] < depths[u] {
            let ch = u;
            u = parents[0][u];
            Some(ch)
        } else {
            None
        };
        assert!(depths[v] == depths[u]);

        if v == u {
            return (v, ch);
        }

        for i in (0..K).rev() {
            if parents[i][v] != parents[i][u] {
                v = parents[i][v];
                u = parents[i][u];
            }
        }

        if !swapped {
            (parents[0][v], Some(v))
        } else {
            (parents[0][v], Some(u))
        }
    };

    let mut pre_ins = vec![0i64; n];
    let mut ins = vec![0i64; n];
    let mut outs = vec![0i64; n];
    let mut post_outs = vec![0i64; n];

    let mut children_order = vec![None; n];

    for (s, t) in st.citer() {
        let (l, extra) = lca(s, t);

        if l == s {
            let ch = extra.unwrap();
            pre_ins[ch] += 1;
            ins[t] -= 1;
        } else if l == t {
            let ch = extra.unwrap();
            outs[s] += 1;
            post_outs[ch] -= 1;
        } else {
            outs[s] += 1;
            ins[t] -= 1;

            let s_parent = extra.unwrap();

            match children_order[l] {
                None => {
                    children_order[l] = Some(s_parent);
                }
                Some(vv) if vv == s_parent => {
                    // NOP
                }
                _ => {
                    println!("0");
                    return;
                }
            }
        }
    }

    let (r, x) = solve(&g, 0, &pre_ins, &ins, &outs, &post_outs, &children_order, k);
    assert_eq!(x, 0);

    let ans = r.citer().sum::<Mod998244353>();

    println!("{ans}");
}
