//! 最大値を`NULL`として扱う`usize`

/// 最大値を`NULL`として扱う`usize`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullableUsize(pub usize);

impl NullableUsize {
    /// NULLとして扱う値(`usize`の最大値)
    pub const NULL: NullableUsize = Self(!0);

    /// `NULL`ならば`true`を返す
    pub fn is_null(self) -> bool {
        self.0 == Self::NULL.0
    }
}

impl Default for NullableUsize {
    fn default() -> Self {
        Self::NULL
    }
}
