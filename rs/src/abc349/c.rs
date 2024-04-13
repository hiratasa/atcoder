use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        mut t: Chars,
    };

    if t.last() == Some(&'X') {
        t.pop();
    }

    if s.into_iter()
        .fold(t, |mut t, c| {
            if !t.is_empty() && t[0] == c.to_ascii_uppercase() {
                t.remove(0);
            }

            t
        })
        .is_empty()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
