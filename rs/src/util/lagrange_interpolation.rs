pub trait FieldValue:
    Copy
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
    + num::Zero
    + num::One
{
}

impl<T> FieldValue for T where
    T: Copy
        + std::fmt::Debug
        + std::ops::Add<Output = Self>
        + std::ops::AddAssign
        + std::ops::Sub<Output = Self>
        + std::ops::SubAssign
        + std::ops::Mul<Output = Self>
        + std::ops::MulAssign
        + std::ops::Div<Output = Self>
        + std::ops::DivAssign
        + std::ops::Neg<Output = Self>
        + std::iter::Sum
        + num::Zero
        + num::One
{
}

// ラグランジュ補間
// O(N^2) （ちゃんとやれば O(N (logN)^2) にできるらしい）
#[allow(dead_code)]
pub fn lagrange_interpolation<T: FieldValue>(xy: &[(T, T)]) -> Vec<T> {
    // Π(x - x_i)
    let f = xy
        .iter()
        .map(|&(x, _)| x)
        .fold(vec![T::one()], |mut f, x0| {
            // f * (x - x_i)
            f.push(T::zero());

            let n = f.len();
            for i in (0..n - 1).rev() {
                f[i + 1] = f[i + 1] + f[i];
                f[i] *= -x0;
            }

            f
        });

    let n = xy.len();
    assert!(f.len() == n + 1);

    let mut poly = vec![T::zero(); n];
    xy.iter().for_each(|&(x0, y0)| {
        // g = f / (x-x0)
        let mut g = f.clone();
        if x0.is_zero() {
            g.remove(0);
        } else {
            let inv_x0 = T::one() / x0;
            for i in 0..n {
                let c = g[i] * inv_x0;
                g[i] = g[i] * inv_x0;
                g[i + 1] += c;
            }
            assert!(g[n].is_zero());
        }

        // v = g(x0)
        let v = g
            .iter()
            .scan(T::one(), |p, c| Some(std::mem::replace(p, *p * x0) * *c))
            .sum::<T>();
        assert!(!v.is_zero());

        let c = y0 / v;
        poly.iter_mut().zip(g.iter()).for_each(|(to, from)| {
            *to += *from * c;
        });
    });

    poly
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::modulo::*;
    type Mod = Mod998244353;

    #[test]
    fn test_lagrange_interpolation() {
        let f = vec![Mod::new(1), Mod::new(2), Mod::new(3)];

        let xy = [
            (Mod::new(0), Mod::new(1)),
            (Mod::new(1), Mod::new(6)),
            (Mod::new(2), Mod::new(17)),
        ];

        assert_eq!(f, lagrange_interpolation(&xy));
    }

    #[test]
    fn test_lagrange_interpolation_random() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..10000 {
            let n = rng.gen_range(2..=5);
            let f = (0..n)
                .map(|_| Mod::new(rng.gen_range(0..10)))
                .collect::<Vec<_>>();

            let mut xy = vec![];
            while xy.len() < n {
                let x = Mod::new(rng.gen_range(0..50));
                if xy.iter().any(|&(xx, _)| x == xx) {
                    continue;
                }

                let y = f
                    .iter()
                    .scan(Mod::new(1), |p, &c| Some(std::mem::replace(p, *p * x) * c))
                    .sum::<Mod>();

                xy.push((x, y));
            }

            assert_eq!(f, lagrange_interpolation(&xy), "f={:?}, xy={:?}", f, xy);
        }
    }
}
