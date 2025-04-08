//! `cumsum`を提供する。

use std::iter::once;

/// `cumsum`を提供する。
pub trait CumSum: Iterator {
    /// 累積和を返すイテレータを生成する。
    fn cumsum<St, F>(self, init: St, mut f: F) -> impl Iterator<Item = St>
    where
        Self: Sized,
        St: Copy,
        F: FnMut(&mut St, Self::Item),
    {
        once(init).chain(self.scan(init, move |acc, x| {
            f(acc, x);
            Some(*acc)
        }))
    }
}

impl<I> CumSum for I where I: Iterator + ?Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [1, 2, 3, 4, 5];
        let b = a.iter().cumsum(0, |acc, x| *acc += x).collect::<Vec<_>>();

        assert_eq!(b, vec![0, 1, 3, 6, 10, 15]);
    }
}
