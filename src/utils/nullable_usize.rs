#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullableUsize(pub usize);

impl NullableUsize {
    pub const NULL: NullableUsize = Self(!0);

    pub fn is_null(self) -> bool {
        self.0 == Self::NULL.0
    }
}

impl Default for NullableUsize {
    fn default() -> Self {
        Self::NULL
    }
}
