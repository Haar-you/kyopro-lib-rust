//! 原始根

use crate::for_loop;
use crate::math::mod_ops::pow::*;

/// 原始根
pub const fn primitive_root(p: u32) -> u32 {
    let mut pf = [0; 32];
    let mut n = p - 1;
    let mut j = 0;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            pf[j] = i;
            j += 1;
        }
        i += 1
    }

    if n != 1 {
        pf[j] = n;
    }

    for_loop!(let mut g = 2; g <= p; g += 1; {
        let mut ok = true;

        for_loop!(let mut i = 0; i < pf.len(); i += 1; {
            let f = pf[i];
            if f == 0 {
                break;
            }
            if mod_pow(g as u64, (p as u64 - 1) / f as u64, p as u64) == 1 {
                ok = false;
                break;
            }
        });

        if ok {
            return g;
        }
    });

    panic!("No primitive roots.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(primitive_root(469762049), 3);
        assert_eq!(primitive_root(167772161), 3);
        assert_eq!(primitive_root(754974721), 11);
        assert_eq!(primitive_root(1012924417), 5);

        struct A<const P: u32, const PR: u32>;
        impl<const P: u32, const PR: u32> A<P, PR> {
            fn print(&self) {
                dbg!(P, PR);
            }
        }

        let a = A::<998244353, { primitive_root(998244353) }>;
        a.print();
    }
}
