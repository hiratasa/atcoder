// NOTE: 右辺がgcdでないものを解くときの手順
//  x*a+y*b=c
//  1. extgcd(a, b) して(x, y, g)の組を求める
//  2. c % g != 0 なら解なし
//  3. kを任意の整数として ((c/g)*x + kb/g, (c/g)*y - ka/g) が解
//    ここで kb/g はc/gを掛けた後に足してよいことに注意
//    特にxの0以上の最小の解は ((c/g)*x).rem_euclid(b/g)

// a * x + b * y = gcd(a, b)
// |x| <= b, |y| <= a when ab != 0
// xにb/gを足してyに-a/gを足したものも解
// 返り値は(x, y, g)
// |x| <= b, |y| <= a であるが、これは（gが1でない限り）絶対値最小の解を保証しないことに注意
// 特にxの0以上の最小の解は x.rem_euclid(b/g)
// 再帰版
// #[allow(dead_code)]
// fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
//     if a == 0 {
//         (0, 1, b)
//     } else {
//         // r.0 * (b%a) + r.1 * a = r.2 (=gcd(a, b))
//         // <-> r.0 * (b - (b/a)*a) + r.1 * a = gcd(a, b)
//         // <-> (r.1 - r.0 * (b/a)) * a + r.0 * b = gcd(a, b)
//         // |r.1 - r.0 * (b/a)| <= b%a + a * (b/a) = b
//         // |r.0| <= a
//         let r = extgcd(b % a, a);
//         (r.1 - b / a * r.0, r.0, r.2)
//     }
// }

// (a b) (x y)^T = gcd(a, b) を解きたい
// - (a_0 b_0) = (a b)
// - (a_(i+1), b_(i+1)) = (b_i%a_i, a_i)
// ここで変換行列は、 (a_(i+1), b_(i+1)) = (a_i, b_i) D_i として、
//  D_i = (-(b_i/a_i) 1)
//        (         1 0)
// (a_n, b_n) = (0 g) となるまで繰り返すと、
//  (0 g) = (a b) D_0 D_1 ... D_(n-1)
// 元の方程式 (a b) (x y)^T = 1 は、(x y)^T = D_0 ... D_(n-1) (z 1)^T を解として持つ
// (ｚは任意)
// D_0 ... D_n の1行目の要素を (u_n v_n) と置くと、
// (u_i v_i) = (-u_(i-1)*(b_i/a_i) + v_(i-1), u_(i-1))
// これを繰り返して (u_n v_n) を求めることで、x=(u_n v_n) (z 1)^T が分かる
// y は (g - a*x) / b から求まる
#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (_zero, g, _u, v) = std::iter::successors(Some((a, b, 1, 0)), |&(a, b, u, v)| {
        if a == 0 {
            None
        } else {
            Some((b % a, a, -u * (b / a) + v, u))
        }
    })
    .last()
    .unwrap();

    (v, (g - a * v) / b, g)
}

// 素数とは限らないmに対して逆元を求める
// gcd(a, m) == 1 のときのみ解あり
// (gcdが1でないときは両辺をgcdで割るなど検討しよう)
// extgcd(a, m).0 と実質的に同じ
// a * r = 1 mod m
// modinv
// 再帰版
// #[allow(dead_code)]
// fn invmod(a: usize, m: usize) -> Option<usize> {
//     let a = a % m;

//     if a == 0 || m == 1 {
//         None
//     } else if m % a == 0 {
//         if a == 1 {
//             Some(1)
//         } else {
//             None
//         }
//     } else if let Some(r) = invmod(m % a, a) {
//         Some(((1 + (a - r) * (m % a)) / a + (a - r) * (m / a)) % m)
//     } else {
//         None
//     }
// }
// extgcdのコメント参照
#[allow(dead_code)]
fn invmod(a: usize, m: usize) -> Option<usize> {
    let (_zero, g, _u, v) =
        std::iter::successors(Some((a as i64, m as i64, 1, 0)), |&(a, b, u, v)| {
            if a == 0 {
                None
            } else {
                Some((b % a, a, -u * (b / a) + v, u))
            }
        })
        .last()
        .unwrap();

    if g == 1 {
        // |v| < m が保障される
        if v < 0 {
            Some((v + m as i64) as usize)
        } else {
            Some(v as usize)
        }
    } else {
        None
    }
}

#[test]
fn test_extgcd() {
    let cases = vec![(3, 4, 1), (2, 8, 2), (18, 12, 6)];

    for &(a, b, g) in &cases {
        let r = extgcd(a, b);
        assert_eq!(r.2, g);
        assert_eq!(r.0 * a + r.1 * b, r.2);
        assert!(r.0.abs() <= b);
        assert!(r.1.abs() <= a);
    }
}

#[test]
fn test_invmod() {
    let ng_cases = vec![(2, 8), (18, 12)];

    for &(a, m) in &ng_cases {
        assert_eq!(invmod(a, m), None);
    }

    let ok_cases = vec![(3, 4), (6, 7)];
    for &(a, m) in &ok_cases {
        assert_eq!(invmod(a, m).map(|r| r * a % m), Some(1))
    }
}
