use std::iter::{once, successors};

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let pows9 = successors(Some(1usize), |&x| x.checked_mul(9)).collect::<Vec<_>>();
    let table = once(0)
        .chain(pows9[1..].iter().copied())
        .scan(0usize, |x, y| {
            *x = x.checked_add(y)?;
            Some(*x)
        })
        .collect::<Vec<_>>();

    for _ in 0..t {
        input! { k: usize };

        // 桁数
        let d = table.iter().position(|&x| k <= x).unwrap();

        if d == 1 {
            println!("{}", k);
        } else {
            let l = k - table[d - 1] - 1;
            let x0 = 1 + l / pows9[d - 1];
            let y = l % pows9[d - 1];

            print!("{x0}");
            pows9[..d - 1]
                .iter()
                .copied()
                .rev()
                .scan(y, |z, a| {
                    let b = *z / a;
                    *z %= a;

                    Some(b)
                })
                .scan(x0, |prev, x| {
                    let fixed = if *prev <= x { x + 1 } else { x };

                    *prev = fixed;

                    Some(fixed)
                })
                .for_each(|x| {
                    print!("{x}");
                });

            println!();
        }
    }
}
