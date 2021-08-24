pub struct SubsetAsc {
    a: u32,
    t: u32,
    end: bool,
}

impl Iterator for SubsetAsc {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            if self.t == self.a {
                self.end = true;
            }

            let ret = self.t;
            self.t = ((self.t as i32 - self.a as i32) & (self.a as i32)) as u32;

            Some(ret)
        }
    }
}

pub fn subset_asc(a: u32) -> SubsetAsc {
    SubsetAsc {
        a,
        t: 0,
        end: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32) {
        let a = (0..=x).filter(|i| (!x & i) == 0).collect::<Vec<_>>();

        let b = subset_asc(x).collect::<Vec<_>>();

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
