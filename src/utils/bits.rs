pub fn highest_one(i: u64) -> u32 {
    assert!(i > 0);
    63 - i.leading_zeros()
}
