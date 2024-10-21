//! `reject`を提供する。

/// `predicate`に該当する要素を除外するイテレータ。
pub struct _Reject<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Iterator for _Reject<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(a) = self.iter.next() {
            if !(self.predicate)(&a) {
                return Some(a);
            }
        }
        None
    }
}

/// `reject`を提供する。
pub trait Reject: Iterator {
    /// `predicate`に該当する要素を除外するイテレータを生成する。
    fn reject<P>(self, predicate: P) -> _Reject<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        _Reject {
            iter: self,
            predicate,
        }
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
