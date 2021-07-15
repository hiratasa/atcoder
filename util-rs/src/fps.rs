use super::fft::convolution_mod;
use super::modulo::{Mod, Modulus};
use std::iter::*;

trait Convolution: std::marker::Sized {
    fn convolution(v: &[Self], u: &[Self]) -> Vec<Self>;
}

impl<M: Modulus> Convolution for Mod<M> {
    fn convolution(v: &[Self], u: &[Self]) -> Vec<Self> {
        convolution_mod(v, u)
    }
}

trait Inverse {
    fn inv(&self) -> Self;
}

impl<M: Modulus> Inverse for Mod<M> {
    fn inv(&self) -> Self {
        Mod::inv(*self)
    }
}

// 形式的冪級数
#[derive(Debug, Clone)]
struct FormalPowerSeries<T>(Vec<T>);

#[allow(dead_code)]
impl<T> FormalPowerSeries<T> {
    fn deg(&self) -> usize {
        self.0.len() - 1
    }

    fn shrink(&mut self, n: usize) {
        if n < self.0.len() {
            self.0.resize_with(n, || unreachable!());
        }
    }

    // apply x => -x
    fn apply_neg_x(&self) -> FormalPowerSeries<T>
    where
        T: std::ops::Neg<Output = T> + Copy,
    {
        FormalPowerSeries(
            self.0
                .iter()
                .enumerate()
                .map(|(i, &x)| if i % 2 == 0 { x } else { -x })
                .collect(),
        )
    }

    fn even(&self) -> FormalPowerSeries<T>
    where
        T: Copy,
    {
        FormalPowerSeries(self.0.iter().step_by(2).copied().collect())
    }

    fn odd(&self) -> FormalPowerSeries<T>
    where
        T: Copy,
    {
        FormalPowerSeries(self.0.iter().skip(1).step_by(2).copied().collect())
    }

    fn reverse(&mut self) {
        self.0.reverse();
    }

    fn reversed(&self) -> Self
    where
        T: Clone,
    {
        let mut r = self.clone();
        r.reverse();
        r
    }

    // mod x^n での逆元を求める
    fn inv(&self, n: usize) -> FormalPowerSeries<T>
    where
        T: Copy
            + Inverse
            + Convolution
            + std::ops::Add<T, Output = T>
            + std::ops::Mul<u32, Output = T>
            + std::ops::Neg<Output = T>,
    {
        let mut g = vec![self.0[0].inv()];
        if self.0.len() == 1 {
            return FormalPowerSeries(g);
        }

        while g.len() < n {
            let k = 2 * g.len();
            let mut h = T::convolution(&g, &g);
            h = T::convolution(&self.0[..std::cmp::min(self.0.len(), k)], &h);
            h.resize_with(k, || unreachable!());

            h.iter_mut().for_each(|x| *x = -*x);
            h.iter_mut().zip(g).for_each(|(x, y)| *x = *x + y * 2);

            g = h;
        }

        g.resize_with(n, || unreachable!());
        FormalPowerSeries(g)
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn add(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x + *y)
                .collect(),
        )
    }
}

impl<T: Copy + std::ops::Sub<Output = T>> std::ops::Sub for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x - *y)
                .collect(),
        )
    }
}

impl<T: Convolution> std::ops::Mul for &FormalPowerSeries<T> {
    type Output = FormalPowerSeries<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        FormalPowerSeries(T::convolution(&self.0, &rhs.0))
    }
}

// 多項式除算
// O(n logn) (nはselfの次数)
// これは形式的冪級数ではなく多項式クラスに定義すべき？
impl<
        T: Copy
            + Default
            + Inverse
            + Convolution
            + std::ops::Add<T, Output = T>
            + std::ops::Mul<u32, Output = T>
            + std::ops::Neg<Output = T>,
    > std::ops::Div for &FormalPowerSeries<T>
{
    type Output = FormalPowerSeries<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self.deg();
        let m = rhs.deg();

        if n < m {
            return FormalPowerSeries(vec![T::default()]);
        }

        // 商の次数
        let k = n - m;

        let rev_rhs = rhs.reversed();
        let x = rev_rhs.inv(k + 1);

        let mut q = &self.reversed() * &x;
        q.shrink(k + 1);
        q.reverse();

        q
    }
}

// 隣接k+1項間漸化式で定められる数列{a_n}のn項目を計算する
// a_{k} = sum[i=0 to k-1] c_{i} * a_{k-i-1}
// https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// http://q.c.titech.ac.jp/docs/progs/polynomial_division.html
#[allow(dead_code)]
fn bostan_mori<M: Modulus + Clone + Default>(a: &[Mod<M>], c: &[Mod<M>], n: usize) -> Mod<M> {
    let k = c.len();

    let q = FormalPowerSeries(
        once(Mod::new(1))
            .chain(c.iter().copied().map(|x| -x))
            .collect(),
    );

    // a.len() < k のとき、残りの項は負の番号の項をゼロとして漸化式で定められるとみなす
    let p = if a.len() < k {
        let q2 = FormalPowerSeries(
            once(Mod::new(1))
                .chain(c.iter().copied().map(|x| -x))
                .take(a.len())
                .collect(),
        );
        let mut p = &FormalPowerSeries(a.to_vec()) * &q2;
        p.shrink(a.len());
        p
    } else {
        let g = FormalPowerSeries(a[0..k].to_vec());
        let mut p = &g * &q;
        p.shrink(k);
        p
    };

    successors(Some((n, p, q)), |(m, pp, qq)| {
        if *m == 0 {
            return None;
        }

        let qq2 = qq.apply_neg_x();
        let u = pp * &qq2;

        Some((
            m / 2,
            if m % 2 == 0 { u.even() } else { u.odd() },
            (qq * &qq2).even(),
        ))
    })
    .last()
    .map(|(_, pp, qq)| pp.0[0] / qq.0[0])
    .unwrap()
}
