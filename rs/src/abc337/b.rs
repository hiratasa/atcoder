use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = s
        .into_iter()
        .try_fold(0, |stage, c| {
            let c = c as usize - 'A' as usize;

            if c >= stage {
                Some(c)
            } else {
                None
            }
        })
        .is_some();

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
