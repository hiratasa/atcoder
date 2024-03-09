use itertools::{chain, iproduct};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    };

    let removes = (0..n)
        .filter(|&i| {
            (0..n).filter(|&j| j != i).any(|j| {
                if s[i] == s[j] {
                    i > j
                } else {
                    s[j].contains(&s[i])
                }
            })
        })
        .collect::<Vec<_>>();

    for idx in removes.into_iter().rev() {
        s.remove(idx);
    }

    let n = s.len();
    let s = s
        .into_iter()
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let precalc = (0..n)
        .map(|i| {
            let s0 = &s[i];

            (0..n)
                .map(|j| {
                    let s1 = &s[j];

                    let t = chain(s1.iter().copied(), s0.iter().copied()).collect::<Vec<_>>();
                    let z = z_algorithm(&t);

                    (0..s0.len())
                        .filter(|&i| z[s1.len() + i] >= s0.len() - i)
                        .min()
                        .unwrap_or(s0.len())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dp = iproduct!(0usize..1 << n, 0..n)
        .filter(|&(set, i)| set & (1 << i) > 0)
        .fold(vec![vec![usize::MAX; n]; 1 << n], |mut dp, (set, i)| {
            let t = &s[i];
            let l = t.len();

            dp[set][i] = if set == 1 << i {
                l
            } else {
                (0..n)
                    .filter(|&j| set & (1 << j) > 0 && j != i)
                    .map(|j| dp[set ^ (1 << i)][j] + (s[i].len() + precalc[j][i] - s[j].len()))
                    .min()
                    .unwrap()
            };

            dp
        });

    let ans = (0..n).map(|i| dp[(1 << n) - 1][i]).min().unwrap();

    println!("{ans}");
}

#[allow(dead_code)]
fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
}
