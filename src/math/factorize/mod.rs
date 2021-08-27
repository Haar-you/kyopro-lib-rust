pub mod sieve;
pub mod trial;

#[cfg(test)]
mod tests {
    use super::trial::*;

    #[test]
    fn test() {
        assert_eq!(factorize(100), [(2, 2), (5, 2)]);
        assert_eq!(factorize(49), [(7, 2)]);
        assert_eq!(factorize(97), [(97, 1)]);
    }
}
