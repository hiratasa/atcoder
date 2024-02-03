use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    };

    let t = ab
        .iter()
        .copied()
        .enumerate()
        .fold(vec![0; 2 * n], |mut t, (i, (a, b))| {
            t[a] = i;
            t[b] = i;

            t
        });

    let ans = !t
        .into_iter()
        .fold(vec![], |mut st, i| {
            if matches!(st.last(), Some(&j) if i == j) {
                st.pop();
            } else {
                st.push(i);
            }

            st
        })
        .is_empty();

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
