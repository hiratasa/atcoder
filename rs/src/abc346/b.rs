use proconio::input;

fn main() {
    input! {
        w: usize, b: usize,
    };

    let s = "wbwbwwbwbwbw".chars().cycle().take(300).collect::<Vec<_>>();

    if s.windows(w + b).any(|t| {
        let ww = t.iter().filter(|&&c| c == 'w').count();
        let bb = t.len() - ww;

        ww == w && bb == b
    }) {
        println!("Yes");
    } else {
        println!("No");
    }
}
