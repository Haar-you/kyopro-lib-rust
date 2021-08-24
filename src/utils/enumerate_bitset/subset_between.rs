pub struct SubsetBetween {
    a: u32,
    t: u32,
    x: u32,
    end: bool,
}

impl Iterator for SubsetBetween {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            let ret = self.t | self.a;

            if self.t == 0 {
                self.end = true;
            } else {
                self.t = (self.t - 1) & self.x;
            }

            Some(ret)
        }
    }
}

pub fn subset_between(a: u32, b: u32) -> SubsetBetween {
    SubsetBetween {
        a,
        t: 0,
        x: b ^ (a & b),
        end: a & !b != 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32, y: u32) {
        let a = (0..=x)
            .filter(|i| (x & !i) == 0 && (!y & i) == 0)
            .collect::<Vec<_>>();

        let b = subset_between(x, y).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111, 0b11111111);
        check(0b00000000, 0b11111111);
        check(0b10101010, 0b11111111);
        check(0b00000001, 0b01010101);
        check(0b00000001, 0b00000010);
    }
}
