#[allow(unused_imports)]
use itertools::{izip, Itertools};
#[allow(unused_imports)]
use num::{One, Zero};
#[allow(unused_imports)]
use std::iter::once;

trait MatrixElement:
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
    + num::Zero
    + num::One
{
}

impl<T> MatrixElement for T where
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
        + num::Zero
        + num::One
{
}

#[allow(dead_code)]
fn calc_det<T>(a: &[Vec<T>]) -> T
where
    T: MatrixElement,
{
    let mut a = a.to_vec();

    let n = a.len();

    let mut det = T::one();
    for i in 0..n {
        let idx = match (i..n).find(|&idx| !a[idx][i].is_zero()) {
            Some(idx) => idx,
            _ => return T::zero(),
        };

        a.swap(i, idx);

        det *= a[i][i];
        let c = T::one() / a[i][i];

        for j in i..n {
            a[i][j] *= c;
        }

        for i2 in i + 1..n {
            let c = -a[i2][i];

            for j in i..n {
                a[i2][j] = a[i2][j] + c * a[i][j];
            }
        }
    }

    det
}

// PAP^(-1)の形の変換でHessenberg行列に変換
// O(N^3)
#[allow(dead_code)]
fn to_upper_hessenberg_matrix<T>(a: &mut [Vec<T>])
where
    T: MatrixElement,
{
    let n = a.len();

    for idx in 1..n {
        let idx2 = match (idx..n).find(|&i| !a[i][idx - 1].is_zero()) {
            Some(idx2) => idx2,
            _ => continue,
        };

        a.swap(idx, idx2);
        a.iter_mut().for_each(|row| {
            row.swap(idx, idx2);
        });

        let inv = T::one() / a[idx][idx - 1];
        for i in idx + 1..n {
            let c = -a[i][idx - 1] * inv;

            // 行idxのc倍を行iに足す
            for j in idx - 1..n {
                a[i][j] = a[i][j] + c * a[idx][j];
            }

            // 列iのc倍を列idxから引く
            for i2 in 0..n {
                a[i2][idx] = a[i2][idx] - c * a[i2][i];
            }
        }
    }
}

// det(A-xI)
// O(N^3)
#[allow(dead_code)]
fn characteristic_polynomial<T>(a: &[Vec<T>]) -> Vec<T>
where
    T: MatrixElement,
{
    let mut a = a.to_vec();

    to_upper_hessenberg_matrix(&mut a);

    let n = a.len();

    // 左上のi行i列部分の行列式(多項式)
    let mut dets = vec![vec![]; n + 1];
    dets[0] = vec![T::one()];
    for i in 0..n {
        dets[i + 1] = vec![T::zero(); i + 2];

        // (i,i)要素 * 残り
        for deg in 0..=i {
            dets[i + 1][deg] = dets[i + 1][deg] + a[i][i] * dets[i][deg];
        }
        for deg in 0..=i {
            dets[i + 1][deg + 1] = dets[i + 1][deg + 1] - dets[i][deg];
        }

        let mut p = T::one();
        for i2 in (0..i).rev() {
            // (i2+1,i2)要素 * ...
            p *= -a[i2 + 1][i2];

            // (i2,i)要素 * 残り
            for deg in 0..=i2 {
                dets[i + 1][deg] = dets[i + 1][deg] + p * a[i2][i] * dets[i2][deg];
            }
        }
    }

    std::mem::take(&mut dets[n])
}

// det(A+xB)をdet(A'+xI)/(d*x^k)の形に変換し、(d, k)を返す
// 恒等的に0の場合はNoneを返す
// O(N^3)
#[allow(dead_code)]
fn xb_to_xidentity<T>(a: &mut [Vec<T>], b: &mut [Vec<T>]) -> Option<(T, usize)>
where
    T: MatrixElement,
{
    let n = a.len();
    assert!(b.len() == n);

    let mut det = T::one();
    let mut deg = 0;
    for idx in 0..n {
        let idx2 = if let Some(idx2) = (idx..n).find(|&i| !b[i][idx].is_zero()) {
            idx2
        } else {
            // Bのidx番目の列が全て0なので、A+xB全体のidx番目の列にxを掛け、Aから項を持ってくる
            loop {
                deg += 1;
                if deg > n {
                    return None;
                }
                for i in 0..n {
                    assert!(b[i][idx].is_zero());
                    b[i][idx] = a[i][idx];
                    a[i][idx] = T::zero();
                }

                // idx行目より前を再度履き出す
                for idx2 in 0..idx {
                    let c = -b[idx2][idx];
                    for i in 0..n {
                        a[i][idx] = a[i][idx] + a[i][idx2] * c;
                        b[i][idx] = b[i][idx] + b[i][idx2] * c;
                    }
                }

                // 改めて非零の行を探す
                if let Some(idx2) = (idx..n).find(|&i| !b[i][idx].is_zero()) {
                    break idx2;
                }
                // 非零の行がなかった場合でも、再度の履き出しによりAのidx列目に0以外の値が出現している場合があるので、再度同様の手順を繰り返す
            }
        };
        a.swap(idx, idx2);
        b.swap(idx, idx2);

        let c = T::one() / b[idx][idx];
        det *= c;
        for j in 0..n {
            a[idx][j] *= c;
            b[idx][j] *= c;
        }

        for i in idx + 1..n {
            let c = -b[i][idx];
            for j in 0..n {
                a[i][j] = a[i][j] + a[idx][j] * c;
                b[i][j] = b[i][j] + b[idx][j] * c;
            }
        }

        for j in idx + 1..n {
            let c = -b[idx][j];
            for i in 0..n {
                a[i][j] = a[i][j] + a[i][idx] * c;
                b[i][j] = b[i][j] + b[i][idx] * c;
            }
        }
    }

    Some((det, deg))
}

// det(A+xB)
// O(N^3)
#[allow(dead_code)]
fn calc_det_a_xb<T>(a: &[Vec<T>], b: &[Vec<T>]) -> Vec<T>
where
    T: MatrixElement,
{
    let n = a.len();
    assert!(b.len() == n);

    let mut a = a.to_vec();
    let mut b = b.to_vec();

    // 適当な変形でdet(A'+xI)/(d*x^k) の形にする
    let Some((d, k)) = xb_to_xidentity(&mut a, &mut b) else {
        return vec![T::zero()];
    };

    // det(A'-xI)
    let mut poly = characteristic_polynomial(&a);

    // det(A'+xI)
    poly.iter_mut()
        .skip(1)
        .step_by(2)
        .for_each(|v| *v *= -T::one());

    // det(A+xB)
    poly.iter_mut().for_each(|v| {
        *v = *v / d;
    });
    poly.drain(0..k);

    poly
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::modulo::Mod998244353;

    type Mod = Mod998244353;

    #[test]
    fn test_calc_det2() {
        let a = vec![
            vec![Mod::new(2), Mod::new(1)],
            vec![Mod::new(4), Mod::new(3)],
        ];

        let expected = Mod::new(2);

        assert_eq!(expected, calc_det(&a));
    }

    #[test]
    fn test_to_upper_hessenberg_matrix() {
        let mut a = vec![
            vec![Mod::new(2), Mod::new(1), Mod::new(5)],
            vec![Mod::new(4), Mod::new(3), Mod::new(6)],
            vec![Mod::new(8), Mod::new(9), Mod::new(10)],
        ];

        let d = calc_det(&a);

        to_upper_hessenberg_matrix(&mut a);

        for i in 1..a.len() {
            for j in 0..i - 1 {
                assert!(
                    a[i][j].is_zero(),
                    "a[{i}][{j}] should be zero; {}",
                    a.iter().map(|row| row.iter().copied().join(",")).join(";")
                );
            }
        }

        assert_eq!(d, calc_det(&a));
    }

    #[test]
    fn test_characteristic_polynomial2() {
        let a = vec![
            vec![Mod::new(1), Mod::new(2)],
            vec![Mod::new(3), Mod::new(4)],
        ];

        assert_eq!(
            vec![-Mod::new(2), -Mod::new(5), Mod::new(1)],
            characteristic_polynomial(&a)
        );
    }

    #[test]
    fn test_characteristic_polynomial3() {
        let a = vec![
            vec![Mod::new(1), Mod::new(2), Mod::new(3)],
            vec![Mod::new(4), Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8), Mod::new(9)],
        ];

        assert_eq!(
            vec![Mod::new(0), Mod::new(18), Mod::new(15), -Mod::new(1)],
            characteristic_polynomial(&a)
        );
    }

    #[test]
    fn test_xb_to_xidentity() {
        let mut a = vec![
            vec![Mod::new(1), Mod::new(2)],
            vec![Mod::new(3), Mod::new(4)],
        ];
        let mut b = vec![
            vec![Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8)],
        ];

        let det = calc_det(&b);

        assert_eq!((1 / det, 0), xb_to_xidentity(&mut a, &mut b).unwrap());
        assert_eq!(
            vec![vec![Mod::one(), Mod::zero()], vec![Mod::zero(), Mod::one()]],
            b
        );
        assert_eq!(
            vec![
                vec![Mod::new(1) / Mod::new(5), Mod::new(4) / Mod::new(25)],
                vec![-Mod::new(4), Mod::new(9) / Mod::new(5)]
            ],
            a
        );
    }

    #[test]
    fn test_calc_det_a_xb_b_regular_deg2() {
        let a = vec![
            vec![Mod::new(1), Mod::new(2)],
            vec![Mod::new(3), Mod::new(4)],
        ];
        let b = vec![
            vec![Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8)],
        ];

        assert_eq!(
            vec![-Mod::new(2), -Mod::new(4), -Mod::new(2)],
            calc_det_a_xb(&a, &b)
        );
    }

    #[test]
    fn test_calc_det_a_xb_b_regular_deg3() {
        let a = vec![
            vec![Mod::new(2), Mod::new(2), Mod::new(3)],
            vec![Mod::new(4), Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(10), Mod::new(11), Mod::new(12)],
            vec![Mod::new(13), Mod::new(14), Mod::new(15)],
            vec![Mod::new(16), Mod::new(17), Mod::new(19)],
        ];

        assert_eq!(
            vec![-Mod::new(3), -Mod::new(4), Mod::new(5), -Mod::new(3)],
            calc_det_a_xb(&a, &b)
        );
    }

    #[test]
    fn test_calc_det_a_xb_b_nonregular_deg3_1() {
        let a = vec![
            vec![Mod::new(2), Mod::new(2), Mod::new(3)],
            vec![Mod::new(4), Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(10), Mod::new(0), Mod::new(12)],
            vec![Mod::new(13), Mod::new(0), Mod::new(15)],
            vec![Mod::new(16), Mod::new(0), Mod::new(19)],
        ];

        assert_eq!(
            vec![-Mod::new(3), -Mod::new(28), Mod::new(24)],
            calc_det_a_xb(&a, &b)
        );
    }

    #[test]
    fn test_calc_det_a_xb3_b_nonregular_deg3_2() {
        let a = vec![
            vec![Mod::new(2), Mod::new(2), Mod::new(3)],
            vec![Mod::new(4), Mod::new(5), Mod::new(6)],
            vec![Mod::new(7), Mod::new(8), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(1), Mod::new(2), Mod::new(3)],
            vec![Mod::new(2), Mod::new(4), Mod::new(6)],
            vec![Mod::new(3), Mod::new(6), Mod::new(9)],
        ];

        assert_eq!(vec![-Mod::new(3), -Mod::new(3)], calc_det_a_xb(&a, &b));
    }

    #[test]
    fn test_calc_det_a_xb3_b_nonregular_deg3_3() {
        let a = vec![
            vec![Mod::new(1), Mod::new(1), Mod::new(7)],
            vec![Mod::new(3), Mod::new(0), Mod::new(8)],
            vec![Mod::new(9), Mod::new(0), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(1), Mod::new(0), Mod::new(4)],
            vec![Mod::new(0), Mod::new(0), Mod::new(5)],
            vec![Mod::new(0), Mod::new(0), Mod::new(6)],
        ];

        assert_eq!(vec![Mod::new(45), Mod::new(27)], calc_det_a_xb(&a, &b));
    }

    #[test]
    fn test_calc_det_a_xb3_b_nonregular_deg3_equal0_1() {
        let a = vec![
            vec![Mod::new(2), Mod::new(0), Mod::new(3)],
            vec![Mod::new(4), Mod::new(0), Mod::new(6)],
            vec![Mod::new(7), Mod::new(0), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(10), Mod::new(0), Mod::new(12)],
            vec![Mod::new(13), Mod::new(0), Mod::new(15)],
            vec![Mod::new(16), Mod::new(0), Mod::new(19)],
        ];

        assert_eq!(vec![Mod::new(0)], calc_det_a_xb(&a, &b));
    }

    #[test]
    fn test_calc_det_a_xb3_b_nonregular_deg3_equal0_2() {
        let a = vec![
            vec![Mod::new(2), Mod::new(2), Mod::new(3)],
            vec![Mod::new(4), Mod::new(4), Mod::new(6)],
            vec![Mod::new(7), Mod::new(6), Mod::new(9)],
        ];
        let b = vec![
            vec![Mod::new(1), Mod::new(2), Mod::new(3)],
            vec![Mod::new(2), Mod::new(4), Mod::new(6)],
            vec![Mod::new(3), Mod::new(6), Mod::new(9)],
        ];

        assert_eq!(vec![Mod::new(0)], calc_det_a_xb(&a, &b));
    }
}
