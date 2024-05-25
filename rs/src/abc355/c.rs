use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, t: usize,
        a: [Usize1; t],
    };

    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut diag0 = 0;
    let mut diag1 = 0;

    for (i, x) in a.into_iter().enumerate() {
        let row = x / n;
        let col = x % n;

        rows[row] += 1;
        cols[col] += 1;

        if row == col {
            diag0 += 1;
        }
        if row + col == n - 1 {
            diag1 += 1;
        }

        if rows[row] == n || cols[col] == n || diag0 == n || diag1 == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
