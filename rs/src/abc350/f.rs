use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    let (pairs, _) =
        s.iter()
            .enumerate()
            .fold((vec![None; n], vec![]), |(mut pairs, mut st), (i, &c)| {
                match c {
                    '(' => {
                        st.push(i);
                    }
                    ')' => {
                        let j = st.pop().unwrap();
                        pairs[i] = Some(j);
                        pairs[j] = Some(i);
                    }
                    _ => {}
                }
                (pairs, st)
            });

    let mut i = 0;
    let mut ans = vec![];
    let mut rev = false;
    while i < n {
        match (s[i], rev) {
            ('(', false) => {
                i = pairs[i].unwrap() - 1;
                rev = true;
            }
            ('(', true) => {
                i = pairs[i].unwrap() + 1;
                rev = false;
            }
            (')', false) => {
                i = pairs[i].unwrap() - 1;
                rev = true;
            }
            (')', true) => {
                i = pairs[i].unwrap() + 1;
                rev = false;
            }
            (c, false) => {
                ans.push(c);
                i += 1;
            }
            (c, true) => {
                if c.is_ascii_lowercase() {
                    ans.push(c.to_ascii_uppercase());
                } else {
                    ans.push(c.to_ascii_lowercase());
                }
                i -= 1;
            }
        }
    }

    println!("{}", ans.iter().join(""));
}
