#![allow(clippy::new_without_default)]

#[derive(Clone, Debug)]
pub struct XorShift {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl XorShift {
    pub fn new() -> Self {
        Self {
            x: 123_456_789,
            y: 362_436_069,
            z: 521_288_629,
            w: 88_675_123,
        }
    }

    pub fn gen(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rand = XorShift::new();

        for _ in 0..100 {
            rand.gen();
        }
    }
}
