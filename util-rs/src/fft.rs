use super::modulo::{pow_mod, Mod, Modulus};
use num::complex::Complex64;

// Calculate fft: g[i] = sum[j=0 to n] f[j] * es[i * j]
// - length must be power of two
// - es[i] = es[0]^i
// - es[0]^n == 1
// esを順次掛け算で計算していくと誤差が大きくなるので（型に応じた方法で）適切に計算して外から渡す
// 計算量 O(n logn)
fn fft<
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num::One,
>(
    f: &mut Vec<T>,
    es: &[T],
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

    for i in 0..d {
        let b = 1 << i;
        let c = n >> (i + 1); // b * c == n/2

        for j in 0..c {
            for k in 0..b {
                let p = es[k * c];
                let t1 = f[j * 2 * b + k];
                let t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
            }
        }
    }
}

#[allow(dead_code)]
fn fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();
    fft(
        f,
        &(0..n)
            .map(|i| {
                Complex64::from_polar(&1.0, &(2.0 * i as f64 * std::f64::consts::PI / (n as f64)))
            })
            .collect::<Vec<_>>(),
    );
}

#[allow(dead_code)]
fn inv_fft_complex(f: &mut Vec<Complex64>) {
    let n = f.len();

    fft(
        f,
        &(0..n)
            .map(|i| {
                Complex64::from_polar(&1.0, &(-2.0 * i as f64 * std::f64::consts::PI / (n as f64)))
            })
            .collect::<Vec<_>>(),
    );
    for x in f {
        *x /= n as f64;
    }
}

#[allow(dead_code)]
fn convolution<T: Copy + num::ToPrimitive + num::FromPrimitive>(p: &Vec<T>, q: &Vec<T>) -> Vec<T> {
    let n0 = p.len();
    let n1 = q.len();

    let n = (n0 + n1 - 1).next_power_of_two();

    // T => f64 => Complex64
    let mut pf = p
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .chain(std::iter::repeat(Complex64::new(0.0, 0.0)))
        .take(n)
        .collect::<Vec<_>>();
    let mut qf = q
        .iter()
        .map(|x| x.to_f64().unwrap().into())
        .chain(std::iter::repeat(Complex64::new(0.0, 0.0)))
        .take(n)
        .collect::<Vec<_>>();

    fft_complex(&mut pf);
    fft_complex(&mut qf);

    for (x, y) in pf.iter_mut().zip(&qf) {
        *x *= *y;
    }

    inv_fft_complex(&mut pf);

    pf.iter()
        .map(|x| T::from_f64(x.re.round()).unwrap())
        .take(n0 + n1 - 1)
        .collect()
}

// 素数mに対して、原始根を求める
// g^k != 1 (1<=k<m-1), g^(m-1) = 1
#[allow(dead_code)]
fn primitive_root(m: usize) -> usize {
    match m {
        2 => return 1,
        167772161 => return 3,
        469762049 => return 3,
        754974721 => return 11,
        998244353 => return 3,
        _ => {}
    };

    // m - 1の素因数分解
    let primes = (2..)
        .try_fold((vec![], m - 1), |(mut primes, x), i| {
            if i * i > x {
                if x > 1 {
                    primes.push(x);
                }
                Err(primes)
            } else if x % i > 0 {
                Ok((primes, x))
            } else {
                primes.push(i);
                let x = itertools::iterate(x, |x| x / i)
                    .find(|&x| x % i > 0)
                    .unwrap();
                Ok((primes, x))
            }
        })
        .unwrap_err();

    (2..)
        .find(|&g| primes.iter().all(|&p| pow_mod(g, (m - 1) / p, m) != 1))
        .unwrap()
}

#[allow(dead_code)]
fn fft_mod<M: Modulus>(f: &mut Vec<Mod<M>>) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!((M::modulus() - 1) % n == 0);
    let g = primitive_root(M::modulus());
    let c = pow_mod(g, (M::modulus() - 1) / n, M::modulus());
    fft(f, &(0..n).map(|i| Mod::new(c).pow(i)).collect::<Vec<_>>());
}

#[allow(dead_code)]
fn inv_fft_mod<M: Modulus>(f: &mut Vec<Mod<M>>) {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert!((M::modulus() - 1) % n == 0);
    let g = primitive_root(M::modulus());
    // let c = pow_mod(g, (modulus() - 1) / n, modulus()).inv();
    let c = pow_mod(g, (M::modulus() - 1) / n * (n - 1), M::modulus());
    fft(f, &(0..n).map(|i| Mod::new(c).pow(i)).collect::<Vec<_>>());
    for x in f {
        *x /= n;
    }
}

#[allow(dead_code)]
fn convolution_mod<M: Modulus>(p: &Vec<Mod<M>>, q: &Vec<Mod<M>>) -> Vec<Mod<M>> {
    let n0 = p.len();
    let n1 = q.len();

    let n = (n0 + n1 - 1).next_power_of_two();

    let mut pf = p
        .iter()
        .copied()
        .chain(std::iter::repeat(Mod::new(0)))
        .take(n)
        .collect::<Vec<_>>();
    let mut qf = q
        .iter()
        .copied()
        .chain(std::iter::repeat(Mod::new(0)))
        .take(n)
        .collect::<Vec<_>>();

    fft_mod(&mut pf);
    fft_mod(&mut qf);

    for (x, y) in pf.iter_mut().zip(&qf) {
        *x *= *y;
    }

    inv_fft_mod(&mut pf);

    pf.resize(n0 + n1 - 1, Mod::new(0));
    pf
}

// fn convolution_crt(p: &Vec<usize>, q: &Vec<usize>) -> Vec<usize> {
//     define_static_mod!(469762049, Modulus469762049, Mod469762049);
//     define_static_mod!(998244353, Modulus998244353, Mod998244353);

//     type Mod1 = Mod469762049;
//     type Mod2 = Mod998244353;

//     let M1 = Mod1::modulus() as i64;
//     let M2 = Mod2::modulus() as i64;

//     let p1 = p.iter().copied().map(|x| Mod1::new(x)).collect();
//     let q1 = q.iter().copied().map(|x| Mod1::new(x)).collect();
//     let conv1 = convolution_mod(&p1, &q1);

//     let p2 = p.iter().copied().map(|x| Mod2::new(x)).collect();
//     let q2 = q.iter().copied().map(|x| Mod2::new(x)).collect();
//     let conv2 = convolution_mod(&p2, &q2);

//     // u * M1 + v * M2 = 1
//     let (u, v, _1) = extgcd(M1, M2);

//     conv1
//         .into_iter()
//         .zip(conv2)
//         .map(|(r1, r2)| {
//             (r1.0 as i64 * v % M1 * M2 + r2.0 as i64 * u % M2 * M1).rem_euclid(M1 * M2) as usize
//         })
//         .collect()
// }

#[test]
fn test_fft_complex() {
    let a: Vec<Complex64> = vec![1.0.into(), 3.0.into(), 5.0.into(), 2.0.into()];

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
    let a: Vec<usize> = vec![1, 3, 5, 2];
    let b: Vec<usize> = vec![2, 9, 4, 10];

    assert_eq!(convolution(&a, &b), vec![2, 15, 41, 71, 68, 58, 20]);
}

#[test]
fn test_primitive_root() {
    let a = [
        // (167772161, 3),
        // (469762049, 3),
        // (754974721, 11),
        // (998244353, 3),
        (1000000007, 5),
    ];

    for &(m, g) in &a {
        assert_eq!(primitive_root(m), g);
    }
}

#[test]
fn test_convolution_mod() {
    type Mod = super::modulo::Mod998244353;
    let a: Vec<Mod> = vec![Mod::new(100000), Mod::new(200000), Mod::new(300000)];
    let b: Vec<Mod> = vec![Mod::new(400000), Mod::new(500000), Mod::new(600000)];

    assert_eq!(
        convolution_mod(&a, &b),
        vec![
            Mod::new(40000000000),
            Mod::new(130000000000),
            Mod::new(280000000000),
            Mod::new(270000000000),
            Mod::new(180000000000),
        ]
    );
}
