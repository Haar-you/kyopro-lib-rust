const NIM_PRODUCT_TABLE_8: [[u8; 16]; 16] = [
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

pub struct NimProduct {
    table_large: Vec<Vec<u8>>,
}

impl NimProduct {
    pub fn new() -> Self {
        let mut table_large = vec![vec![0; 256]; 256];

        let mask: u8 = 0xf;

        for a in 0..=255 {
            for b in 0..=255 {
                let au = (a >> 4) as usize;
                let al = (a & mask) as usize;
                let bu = (b >> 4) as usize;
                let bl = (b & mask) as usize;

                table_large[a as usize][b as usize] = ((NIM_PRODUCT_TABLE_8[au][bu]
                    ^ NIM_PRODUCT_TABLE_8[al][bu]
                    ^ NIM_PRODUCT_TABLE_8[au][bl])
                    << 4)
                    ^ (NIM_PRODUCT_TABLE_8[au][NIM_PRODUCT_TABLE_8[bu][1 << 3] as usize]
                        ^ NIM_PRODUCT_TABLE_8[al][bl]);
            }
        }

        NimProduct { table_large }
    }

    pub fn nim_product_8(&self, a: u8, b: u8) -> u8 {
        self.table_large[a as usize][b as usize]
    }

    pub fn nim_product_16(&self, a: u16, b: u16) -> u16 {
        let mask = 0xff;

        let au = (a >> 8) as u8;
        let al = (a & mask) as u8;
        let bu = (b >> 8) as u8;
        let bl = (b & mask) as u8;

        (((self.nim_product_8(au, bu) ^ self.nim_product_8(al, bu) ^ self.nim_product_8(au, bl))
            as u16)
            << 8)
            ^ (self.nim_product_8(au, self.nim_product_8(bu, 1 << 7)) ^ self.nim_product_8(al, bl))
                as u16
    }

    pub fn nim_product_32(&self, a: u32, b: u32) -> u32 {
        let mask = 0xffff;

        let au = (a >> 16) as u16;
        let al = (a & mask) as u16;
        let bu = (b >> 16) as u16;
        let bl = (b & mask) as u16;

        (((self.nim_product_16(au, bu) ^ self.nim_product_16(al, bu) ^ self.nim_product_16(au, bl))
            as u32)
            << 16)
            ^ (self.nim_product_16(au, self.nim_product_16(bu, 1 << 15))
                ^ self.nim_product_16(al, bl)) as u32
    }

    pub fn nim_product_64(&self, a: u64, b: u64) -> u64 {
        let mask = 0xffffffff;

        let au = (a >> 32) as u32;
        let al = (a & mask) as u32;
        let bu = (b >> 32) as u32;
        let bl = (b & mask) as u32;

        (((self.nim_product_32(au, bu) ^ self.nim_product_32(al, bu) ^ self.nim_product_32(au, bl))
            as u64)
            << 32)
            ^ (self.nim_product_32(au, self.nim_product_32(bu, 1 << 31))
                ^ self.nim_product_32(al, bl)) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let np = NimProduct::new();

        assert_eq!(
            np.nim_product_64(18446744073709551615, 18446744073709551615),
            11290409524105353207
        );
    }
}
