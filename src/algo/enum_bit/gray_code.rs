//! Gray code
//!
//! <https://ja.wikipedia.org/wiki/%E3%82%B0%E3%83%AC%E3%82%A4%E3%82%B3%E3%83%BC%E3%83%89>

/// `n`桁のGray codeを昇順に列挙する。
pub fn gray_code(n: u32) -> impl Iterator<Item = u32> {
    (0..1 << n).map(|i| i ^ (i >> 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            gray_code(3).collect::<Vec<_>>(),
            [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100]
        );
    }
}
