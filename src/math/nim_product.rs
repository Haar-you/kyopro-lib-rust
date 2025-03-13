const NIM_PRODUCT_TABLE_16: [[u8; 16]; 16] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 2, 3, 1, 8, 10, 11, 9, 12, 14, 15, 13, 4, 6, 7, 5],
    [0, 3, 1, 2, 12, 15, 13, 14, 4, 7, 5, 6, 8, 11, 9, 10],
    [0, 4, 8, 12, 6, 2, 14, 10, 11, 15, 3, 7, 13, 9, 5, 1],
    [0, 5, 10, 15, 2, 7, 8, 13, 3, 6, 9, 12, 1, 4, 11, 14],
    [0, 6, 11, 13, 14, 8, 5, 3, 7, 1, 12, 10, 9, 15, 2, 4],
    [0, 7, 9, 14, 10, 13, 3, 4, 15, 8, 6, 1, 5, 2, 12, 11],
    [0, 8, 12, 4, 11, 3, 7, 15, 13, 5, 1, 9, 6, 14, 10, 2],
    [0, 9, 14, 7, 15, 6, 1, 8, 5, 12, 11, 2, 10, 3, 4, 13],
    [0, 10, 15, 5, 3, 9, 12, 6, 1, 11, 14, 4, 2, 8, 13, 7],
    [0, 11, 13, 6, 7, 12, 10, 1, 9, 2, 4, 15, 14, 5, 3, 8],
    [0, 12, 4, 8, 13, 1, 9, 5, 6, 10, 2, 14, 11, 7, 15, 3],
    [0, 13, 6, 11, 9, 4, 15, 2, 14, 3, 8, 5, 7, 10, 1, 12],
    [0, 14, 7, 9, 5, 11, 2, 12, 10, 4, 13, 3, 15, 1, 8, 6],
    [0, 15, 5, 10, 1, 14, 4, 11, 2, 13, 7, 8, 3, 12, 6, 9],
];

thread_local! {
    static NIM_PRODUCT_TABLE_256: [[u8; 256]; 256] = {
        let mut ret = [[0; 256]; 256];

        let mask: u8 = 0xf;

        for a in 0..=255 {
            for b in 0..=255 {
                let au = (a >> 4) as usize;
                let al = (a & mask) as usize;
                let bu = (b >> 4) as usize;
                let bl = (b & mask) as usize;

                let au_bu = NIM_PRODUCT_TABLE_16[au][bu];
                let al_bu = NIM_PRODUCT_TABLE_16[al][bu];
                let au_bl = NIM_PRODUCT_TABLE_16[au][bl];
                let al_bl = NIM_PRODUCT_TABLE_16[al][bl];

                ret[a as usize][b as usize] = ((au_bu ^ al_bu ^ au_bl) << 4)
                    ^ NIM_PRODUCT_TABLE_16[au][NIM_PRODUCT_TABLE_16[bu][1 << 3] as usize]
                    ^ al_bl;
            }
        }

        ret
    };
}

/// [`u8`]同士のNimber productを求める。
pub fn nim_product_8(a: u8, b: u8) -> u8 {
    NIM_PRODUCT_TABLE_256.with(|t| t[a as usize][b as usize])
}

macro_rules! impl_nim_product {
    ( $(#[$meta:meta])* $uint:ty, $np:ident, $uint_h:ty, $np_h:ident, $bits:expr ) => {
        $(#[$meta])*
        pub fn $np(a: $uint, b: $uint) -> $uint {
            let bits = $bits;

            let mask = (1 << bits) - 1;

            let au = (a >> bits) as $uint_h;
            let al = (a & mask) as $uint_h;
            let bu = (b >> bits) as $uint_h;
            let bl = (b & mask) as $uint_h;

            let au_bu = $np_h(au, bu) as $uint;
            let al_bu = $np_h(al, bu) as $uint;
            let au_bl = $np_h(au, bl) as $uint;
            let al_bl = $np_h(al, bl) as $uint;

            ((au_bu ^ al_bu ^ au_bl) << bits)
                ^ $np_h(au, $np_h(bu, 1 << (bits - 1))) as $uint
                ^ al_bl
        }
    };
}

impl_nim_product!(
    /// [`u64`]同士のNimber productを求める。
    u64, nim_product_64, u32, nim_product_32, 32
);
impl_nim_product!(
    /// [`u32`]同士のNimber productを求める。
    u32, nim_product_32, u16, nim_product_16, 16
);
impl_nim_product!(
    /// [`u16`]同士のNimber productを求める。
    u16, nim_product_16, u8, nim_product_8, 8
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            nim_product_64(18446744073709551615, 18446744073709551615),
            11290409524105353207
        );
    }
}
