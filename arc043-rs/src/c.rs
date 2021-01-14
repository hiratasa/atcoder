use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

struct BIT {
    values: Vec<usize>,
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT { values: vec![0; n] }
    }

    fn add(&mut self, idx: usize, value: usize) {
        let mut idx = idx + 1;

        while idx <= self.values.len() {
            self.values[idx - 1] += value;
            idx += 1 << idx.trailing_zeros();
        }
    }

    // [0, idx)
    fn sum(&self, idx: usize) -> usize {
        let mut sum = 0;

        let mut idx = idx;
        while idx > 0 {
            sum += self.values[idx - 1];
            idx -= 1 << idx.trailing_zeros();
        }

        sum
    }

    fn get(&self, idx: usize) -> usize {
        self.sum(idx + 1) - self.sum(idx)
    }
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    }

    let c = a
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut c, (i, aa)| {
            c[aa] = i;
            c
        });

    let t = b
        .iter()
        .copied()
        .enumerate()
        .scan(BIT::new(n), |bit, (i, bb)| {
            bit.add(c[bb], 1);
            Some(i - bit.sum(c[bb]))
        })
        .sum::<usize>();
    if t % 2 > 0 {
        println!("-1");
        return;
    }

    eprintln!("{}", t);

    let ans = b
        .iter()
        .copied()
        .try_fold((t / 2, BIT::new(n), vec![]), |(r, mut bit, mut ans), bb| {
            let idx = c[bb];

            bit.add(idx, 1);
            let x = idx - bit.sum(idx);

            if x < r {
                ans.push(bb);
                Ok((r - x, bit, ans))
            } else {
                ans.extend(
                    a.iter()
                        .copied()
                        .enumerate()
                        .filter(|&(i, _aa)| bit.get(i) == 0)
                        .map(|t| t.1)
                        .take(x - r),
                );
                ans.push(bb);
                ans.extend(
                    a.iter()
                        .copied()
                        .enumerate()
                        .filter(|&(i, _aa)| bit.get(i) == 0)
                        .map(|t| t.1)
                        .skip(x - r),
                );
                Err(ans)
            }
        })
        .err()
        .unwrap();
    println!("{}", ans.iter().map(|x| x + 1).format(" "));
}
