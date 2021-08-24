pub struct SubsetDesc {
    a: u32,
    t: u32,
    end: bool,
}

impl Iterator for SubsetDesc {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            let ret = self.t;

            if self.t == 0 {
                self.end = true;
            } else {
                self.t = (self.t - 1) & self.a;
            }

            Some(ret)
        }
    }
}

pub fn subset_desc(a: u32) -> SubsetDesc {
    SubsetDesc {
        a,
        t: a,
        end: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32) {
        let a = (0..=x).rev().filter(|i| (!x & i) == 0).collect::<Vec<_>>();

        let b = subset_desc(x).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111);
        check(0b00000000);
        check(0b10101010);
        check(0b00000001);
        check(0b10000000);
        check(0b10000001);
        check(0b11011011);
    }
}
