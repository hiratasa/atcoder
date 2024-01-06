use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, p: usize,
        a: [usize; n],
    };

    if p == 2 {
        println!("{}", n * n);
        return;
    }

    // p-1 の素因数分解
    let prime_factors = (2..)
        .try_fold((vec![], p - 1), |(mut primes, x), i| {
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

    // 位数
    let b = a
        .iter()
        .copied()
        .map(|x| {
            let mut z = p - 1;

            for &q in &prime_factors {
                while z % q == 0 {
                    let y = pow_mod(x, z / q, p);

                    if y != 1 {
                        break;
                    }

                    z /= q;
                }
            }

            z
        })
        .collect::<Vec<_>>();

    // 位数ごとの出現回数
    let c = b
        .iter()
        .copied()
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(x, it)| (x, it.count()))
        .collect::<Vec<_>>();

    let ans = c.iter().copied().map(|(_, k)| k * k).sum::<usize>()
        + c.iter()
            .copied()
            .tuple_combinations()
            .filter(|&((x, _), (y, _))| y % x == 0)
            .map(|((_, k), (_, l))| k * l)
            .sum::<usize>();

    println!("{ans}");
}

fn pow_mod(x: usize, p: usize, m: usize) -> usize {
    let mut x = x as u128;
    let mut p = p as u128;
    let m = m as u128;
    let mut y = 1;

    x = x % m;
    while p > 0 {
        if p & 1 > 0 {
            y = y * x % m;
        }

        x = x * x % m;
        p >>= 1;
    }

    y as usize
}
