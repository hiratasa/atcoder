use cargo_snippet::snippet;

#[snippet("lowerbound")]
#[snippet("lowerboundint")]
#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;

        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }

    begin
}

#[snippet("lowerboundint")]
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: Fn(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

#[test]
fn test_lower_bound_integer() {
    assert_eq!(lower_bound(3, 7, 1, |i| { i.cmp(&3) }), 3);
}

#[test]
fn test_lower_bound_float() {
    const EPSILON: f64 = 0.000001;
    assert!(
        (lower_bound(1.0, 10.0, EPSILON, |i| { i.partial_cmp(&2.5).unwrap() }) - 2.5).abs()
            < EPSILON
    );
}
