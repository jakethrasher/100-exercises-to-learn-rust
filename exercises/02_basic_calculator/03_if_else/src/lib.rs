/// Return `true` if `n` is even, `false` otherwise.
fn is_even(n: u32) -> bool {
    let bool = if n % 2 == 0 { true } else { false };

    return bool;
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
