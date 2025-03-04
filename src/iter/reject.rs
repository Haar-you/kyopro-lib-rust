//! `reject`を提供する。

/// `reject`を提供する。
pub trait Reject: Iterator {
    /// `predicate`に該当する要素を除外するイテレータを生成する。
    fn reject<P>(self, mut predicate: P) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        self.filter(move |a| !(predicate)(a))
    }
}

impl<I> Reject for I where I: Iterator + ?Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a: Vec<_> = (1..=20).reject(|x| x % 3 == 0).collect();

        assert_eq!(a, vec![1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20]);
    }
}
