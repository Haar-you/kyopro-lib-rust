pub struct SupersetDesc {
    a: u32,
    t: u32,
    y: u32,
    end: bool,
}

impl Iterator for SupersetDesc {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            let ret = self.t | self.a;
            if self.t == 0 {
                self.end = true;
            } else {
                self.t = (self.t - 1) & self.y;
            }

            Some(ret)
        }
    }
}

pub fn superset_desc(a: u32, n: u32) -> SupersetDesc {
    let x = (1 << n) - 1;
    let y = x ^ (a & x);
    SupersetDesc {
        a,
        t: y,
        y,
        end: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32, n: u32) {
        let a = (0..1 << n)
            .rev()
            .filter(|i| (x & !i) == 0)
            .collect::<Vec<_>>();

        let b = superset_desc(x, n).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111, 8);
        check(0b00000000, 8);
        check(0b10101010, 8);
        check(0b00000001, 8);
        check(0b10000000, 8);
        check(0b10000001, 8);
        check(0b11011011, 8);
    }
}
