// Calculate fft: g[i] = sum[j=0 to n] f[j] * c^(i * j)
// - length must be power of two
// - c^n == 1
// 計算量 O(n logn)
fn fft<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num::One,
>(
    f: &mut Vec<T>,
    c: T,
) {
    let n = f.len();

    if n == 1 {
        return;
    }

    assert!(n.is_power_of_two());

    let d = n.trailing_zeros() as usize;

    for i in 0..n {
        let j = i.reverse_bits() >> (std::mem::size_of::<usize>() * 8 - d);

        if i < j {
            f.swap(i, j);
        }
    }

    let cs = std::iter::successors(Some(c), |cc| Some(*cc * *cc))
        .take(d)
        .collect::<Vec<_>>();
    for i in (0..d).rev() {
        let b = n >> (1 + i);
        let c = 1 << i; // b * c == n/2
        let z = cs[i];

        for j in 0..c {
            let mut p = T::one();

            for k in 0..b {
                let t1 = f[j * 2 * b + k];
                let t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
                p = p * z;
            }
        }
    }
}

use num::complex::Complex64;

#[allow(dead_code)]
fn fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();
    fft(
        f,
        Complex64::from_polar(&1.0, &(2.0 * std::f64::consts::PI / (n as f64))),
    );
}

#[allow(dead_code)]
fn inv_fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();

    fft(
        f,
        Complex64::from_polar(&1.0, &(-2.0 * std::f64::consts::PI / (n as f64))),
    );
    for x in f {
        *x /= n as f64;
    }
}

#[allow(dead_code)]
fn convolution<T: Copy + num::ToPrimitive + num::FromPrimitive>(p: &Vec<T>, q: &Vec<T>) -> Vec<T> {
    assert!(p.len() == q.len());
    // assert!(posmax(p) + poxmax(q) < p.len())

    // T => f64 => Complex64
    let mut pf = p
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .collect::<Vec<_>>();
    let mut qf = q
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .collect::<Vec<_>>();

    fft_complex(&mut pf);
    fft_complex(&mut qf);

    let mut r = pf
        .iter()
        .zip(qf.iter())
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>();

    inv_fft_complex(&mut r);

    r.iter()
        .map(|x| T::from_f64(x.re.round()).unwrap())
        .collect()
}

#[test]
fn test_fft_complex() {
    let a = vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(3.0, 0.0),
        Complex64::new(5.0, 0.0),
        Complex64::new(2.0, 0.0),
    ];

    let mut b = a.clone();
    fft_complex(&mut b);

    let e = Complex64::from_polar(&1.0, &(2.0 * std::f64::consts::PI / 4.0));
    let expected_vec = vec![
        a[0] + a[1] + a[2] + a[3],
        a[0] + e * a[1] + e.powi(2) * a[2] + e.powi(3) * a[3],
        a[0] + e.powi(2) * a[1] + e.powi(4) * a[2] + e.powi(6) * a[3],
        a[0] + e.powi(3) * a[1] + e.powi(6) * a[2] + e.powi(9) * a[3],
    ];

    for (i, (actual, expected)) in b.iter().zip(&expected_vec).enumerate() {
        assert!((actual - expected).norm() < 1e-8, "{}-th element", i);
    }

    inv_fft_complex(&mut b);

    for (i, (actual, expected)) in b.iter().zip(&a).enumerate() {
        assert!(
            (actual - expected).norm() < 1e-8,
            "{}-th element not recovered",
            i
        );
    }
}

#[test]
fn test_convolution() {
    let a: Vec<usize> = vec![1, 3, 5, 2, 0, 0, 0, 0];
    let b: Vec<usize> = vec![2, 9, 4, 10, 0, 0, 0, 0];

    assert_eq!(convolution(&a, &b), vec![2, 15, 41, 71, 68, 58, 20, 0]);
}
