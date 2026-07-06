//! Montgomery演算

const B: u32 = 64;
const R: u128 = 1 << B;
const MASK: u128 = R - 1;

/// Montgomery演算
#[derive(Clone, Copy)]
pub struct Montgomery {
    pub modulo: u64,
    pub r2: u128,
    pub m: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Wrapped(pub u64);

impl Montgomery {
    pub fn new(modulo: u64) -> Montgomery {
        assert!(modulo % 2 != 0);
        assert!(modulo > 0);

        let r = R % modulo as u128;
        let r2 = r * r % modulo as u128;
        let m = {
            let mut ret: u64 = 0;
            let mut r = R;
            let mut i = 1;
            let mut t = 0;
            while r > 1 {
                if t % 2 == 0 {
                    t += modulo;
                    ret += i;
                }
                t >>= 1;
                r >>= 1;
                i <<= 1;
            }
            ret
        };

        Self { modulo, r2, m }
    }

    pub fn reduce(&self, value: u128) -> u64 {
        let &Self { modulo, m, .. } = self;

        let mut ret =
            (((((value & MASK) * m as u128) & MASK) * modulo as u128 + value) >> B) as u64;
        if ret >= modulo {
            ret -= modulo;
        }
        ret
    }

    pub fn wrap(&self, a: u64) -> Wrapped {
        Wrapped(self.reduce(a as u128 * self.r2))
    }

    pub fn unwrap(&self, a: Wrapped) -> u64 {
        self.reduce(a.0 as u128)
    }

    pub fn mul(&self, a: Wrapped, b: Wrapped) -> Wrapped {
        Wrapped(self.reduce(a.0 as u128 * b.0 as u128))
    }

    pub fn add(&self, a: Wrapped, b: Wrapped) -> Wrapped {
        let mut t = a.0 as u128 + b.0 as u128;
        if t > self.modulo as u128 {
            t -= self.modulo as u128;
        }
        Wrapped(t as u64)
    }
}
