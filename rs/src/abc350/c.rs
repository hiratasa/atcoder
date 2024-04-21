use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    };

    let mut b = a
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; n], |mut b, (i, j)| {
            b[j] = i;
            b
        });

    let mut ans = vec![];
    for i in 0..n {
        if a[i] == i {
            continue;
        }

        let j = b[i];

        ans.push((i, j));
        a.swap(i, j);
        b[i] = i;
        b[a[j]] = j;
    }

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
