#[macro_export]
macro_rules! load_input {
    () => {{
        include_str!(concat!("../inputs/", env!("CARGO_BIN_NAME"), ".txt"))
    }};
}

#[macro_export]
macro_rules! load_test {
    () => {{
        include_str!(concat!("../inputs/", env!("CARGO_BIN_NAME"), "_test.txt"))
    }};
}

/// Get the first and last iterator elements that match a predicate.
pub fn first_last<T: Copy, I: Iterator>(
    iter: I,
    check: impl Fn(I::Item) -> Option<T>,
) -> Option<(T, T)> {
    iter.fold(None, |acc, next| match (acc, check(next)) {
        (None, Some(next)) => Some((next, next)),
        (Some((first, _)), Some(next)) => Some((first, next)),
        _ => acc,
    })
}

/// Parse a list of numbers seperated by 1 or more spaces.
pub fn parse_number_list(s: &str) -> impl Iterator<Item = u64> + '_ {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse().unwrap())
}
