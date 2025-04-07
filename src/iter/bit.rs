//! `bit_ones`と`bit_zeros`を提供する。

/// `bit_ones`と`bit_zeros`を提供する。
pub trait EnumBit {
    /// ビットが`1`である桁を昇順に列挙する。
    fn bit_ones(self, n: usize) -> impl Iterator<Item = usize>;
    /// ビットが`0`である桁を昇順に列挙する。
    fn bit_zeros(self, n: usize) -> impl Iterator<Item = usize>;
}

macro_rules! implement {
    ($($t:ty),*) => {
        $(
            impl EnumBit for $t {
                fn bit_ones(self, n: usize) -> impl Iterator<Item = usize> {
                    (0..n).filter(move |&i| self & (1 << i) != 0)
                }
                fn bit_zeros(self, n: usize) -> impl Iterator<Item = usize> {
                    (0..n).filter(move |&i| self & (1 << i) == 0)
                }
            }
        )*
    }
}

implement!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use crate::iter::collect::CollectVec;

    use super::*;

    #[test]
    fn test() {
        let a = 0b1001010010101_usize;
        dbg!(a.bit_ones(64).collect_vec());

        let a = 0b1001010010101_u64;
        dbg!(a.bit_ones(64).collect_vec());

        let a = 0b1001010010101_u32;
        dbg!(a.bit_ones(32).collect_vec());
        dbg!(a.bit_zeros(32).collect_vec());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let a = 0b1001010010101_u32;
        dbg!(a.bit_ones(64).collect_vec());
    }
}
