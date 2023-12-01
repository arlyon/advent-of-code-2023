#[macro_export]
macro_rules! load_input {
    () => {{
        include_str!(concat!("../inputs/", env!("CARGO_BIN_NAME"), ".txt"))
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
