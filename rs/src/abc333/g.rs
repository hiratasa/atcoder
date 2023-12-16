use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        r: String,
        n: u128,
    };

    let l = r.len() - 2;
    let p = r[2..].parse::<u128>().unwrap();
    let q = 10u128.pow(l as u32);
    let g = gcd(p, q);

    let (p, q) = (p / g, q / g);

    if q <= n {
        println!("{p} {q}");
        return;
    }

    let compare = |(p0, q0): (u128, u128), (p1, q1): (u128, u128)| (p0 * q1).cmp(&(p1 * q0));

    // Stern–Brocot木の上での探索
    let ((p1, q1), (p2, q2)) = (0..)
        .try_fold(
            ((0, 1), (1, 0), (p, q), false),
            |((p0, q0), (p1, q1), (p, q), rev), _| {
                let m = p / q;

                if q0 + m * q1 > n {
                    let mm = (n - q0) / q1;
                    if !rev {
                        return Err(((p0 + mm * p1, q0 + mm * q1), (p1, q1)));
                    } else {
                        return Err(((p1, q1), (p0 + mm * p1, q0 + mm * q1)));
                    }
                }

                Ok(((p1, q1), (p0 + m * p1, q0 + m * q1), (q, p % q), !rev))
            },
        )
        .unwrap_err();

    let (pp, qq) = match compare((p * q1 - p1 * q, q1), (p2 * q - p * q2, q2)) {
        Ordering::Less | Ordering::Equal => (p1, q1),
        Ordering::Greater => (p2, q2),
    };

    println!("{pp} {qq}");
}

fn gcd(a: u128, b: u128) -> u128 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
