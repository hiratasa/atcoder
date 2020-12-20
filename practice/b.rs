use std::io::*;
use std::str::*;
use std::mem::*;
use std::cmp::Ordering;

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_cols<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn fix_tp_sorted<F>(chars: &mut Vec<char>, less: F, left: char, right: char)
where F: FnMut(char, char) -> Optional<bool>
{
    let fixed = Vec::new();
    let hold = Vec::new();
    let iter = chars.iter().map(|c| *c);

    // before `right`
    while let Some(c) = iter.next() {
        if c == left {
            // left appears before right, so no fix is requied
            return;
        }

        if c == right {
            hold.push(c);
            break;
        }

        fixed.push(c);
    }

    // holding
    while let Some(c) = iter.next() {
        if c == left {
            TODO
        }

        if hold.iter().any(|&h| less(h, c)) {
            hold.push(c);
        } else {
            fixed.push(c);
        }
    }

    // holding
    while let Some(c) = iter.next() {
        if c == left {
            TODO
        }

        if hold.iter().any(|&h| less(h, c)) {
            hold.push(c);
        } else {
            fixed.push(c);
        }
    }




    fixed.append(chars.iter().map(|c| *c).take_while(|c| c != left && c != right));



}

fn choice_pivot<F>(chars: &Vec<char>, less: &mut F) -> char
where
    F: FnMut(char, char) -> bool,
{
    if chars.len() <= 2 {
        return chars[0];
    }

    let mut pivot0 = chars[0];
    let mut pivot1 = chars[chars.len() / 2];
    let mut pivot2 = chars[chars.len() - 1];

    if !less(pivot0, pivot1) {
        swap(&mut pivot0, &mut pivot1);
    }

    if !less(pivot1, pivot2) {
        swap(&mut pivot1, &mut pivot2);
    }

    if !less(pivot0, pivot1) {
        swap(&mut pivot0, &mut pivot1);
    }

    return pivot1;
}

fn quick_sort<F>(chars: Vec<char>, less: &mut F) -> Vec<char>
where
    F: FnMut(char, char) -> bool,
{
    if chars.len() <= 1 {
        return chars;
    }

    let pivot = choice_pivot(&chars, less);

    let mut smaller = Vec::new();
    let mut bigger = Vec::new();

    for c in chars {
        if c == pivot {
            continue;
        } else if less(c, pivot) {
            smaller.push(c);
        } else {
            bigger.push(c);
        }
    }

    if smaller.is_empty() {
        smaller.push(pivot);
    } else {
        bigger.push(pivot);
    }

    let sorted_smaller = quick_sort(smaller, less);
    let mut sorted_bigger = quick_sort(bigger, less);

    let mut sorted = sorted_smaller;
    sorted.append(&mut sorted_bigger);
    sorted
}

fn main() {
    let n_chars;
    let n_queries;

    let v = read_cols::<usize>();
    n_chars = v[0];
    n_queries = v[1];

    let mut cache = Vec::new();
    cache.resize(n_chars, Vec::<Option<bool>>::new());
    for i in 0..n_chars {
        cache[i].resize(n_chars, None);
    }

    let mut less = move |c1, c2| {
        let i1 = ((c1 as u8) - b'A') as usize;
        let i2 = ((c2 as u8) - b'A') as usize;
        if let Some(b) = cache[i1][i2] {
            return b;
        }

        println!("? {} {}", c1, c2);
        let s = read_line();

        if s == "<" {
            cache[i1][i2] = Some(true);
            cache[i2][i1] = Some(false);
            return true;
        } else {
            cache[i1][i2] = Some(false);
            cache[i2][i1] = Some(true);
            return false;
        }
    };

    let chars: Vec<char> = (b'A'..b'Z' + 1).map(|c| c as char).take(n_chars).collect();

    let compare = move |x: &char, y: &char| {
        if less(*x, *y) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    // let sorted = quick_sort(chars, &mut less);
    let mut sorted = chars;
    sorted.sort_by(compare);

    print!("! ");
    for c in sorted {
        print!("{}", c);
    }

    println!("");
}
