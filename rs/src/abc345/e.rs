use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        cv: [(usize, usize); n],
    };

    let mut init = vec![[None; 2]; k + 1];
    init[0][0] = Some((0, 0));
    let dp = cv.iter().copied().fold(init, |prev, (c, v)| {
        let mut next = vec![[None; 2]; k + 1];
        for i in 0..=k {
            let mut t = [None; 3];
            let mut idx = 0;

            // not skip i
            {
                if let Some((vv, cc)) = prev[i].iter().copied().flatten().find(|&(_, cc)| cc != c) {
                    t[idx] = Some((vv + v, c));
                    idx += 1;
                }
            }

            // skip i
            if i > 0 {
                t[idx] = prev[i - 1][0];
                idx += 1;
                t[idx] = prev[i - 1][1];
                idx += 1;
            }

            t.sort();
            t.reverse();
            if matches!((t[0], t[1]), (Some((_, cc0)), Some((_, cc1))) if cc0 == cc1) {
                t.swap(1, 2);
            }
            next[i][0] = t[0];
            next[i][1] = t[1];
        }

        next
    });

    if let Some((v, _)) = dp[k][0] {
        println!("{v}");
    } else {
        println!("-1");
    }
}
