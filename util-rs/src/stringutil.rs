use cargo_snippet::snippet;

#[snippet("stringutil")]
trait ToString {
    fn to_string(self: Self) -> String;
}

#[snippet("stringutil")]
impl<I, T> ToString for I
where
    I: IntoIterator<Item = T>,
    T: std::convert::TryInto<u32>,
{
    fn to_string(self: Self) -> String {
        self.into_iter()
            // to u32
            .map(|t| t.try_into().ok().unwrap())
            // to char
            .map(|t| std::convert::TryInto::<char>::try_into(t).ok().unwrap())
            .collect()
    }
}

#[test]
fn test_to_string() {
    assert_eq!(vec!['a', 'b', 'c'].to_string(), String::from("abc"));
}
