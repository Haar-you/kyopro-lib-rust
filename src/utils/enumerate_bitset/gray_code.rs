pub fn gray_code(n: u32) -> Box<dyn Iterator<Item = u32>> {
    Box::new((0..1 << n).map(|i| i ^ (i >> 1)))
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
