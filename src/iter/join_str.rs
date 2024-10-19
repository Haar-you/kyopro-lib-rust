//! `join_str`を提供する。

/// `join_str`を提供する。
pub trait JoinStr: Iterator {
    /// 要素を[`String`]に変換して、要素間に`s`を挿入して、結合する。
    fn join_str(self, s: &str) -> String
    where
        Self: Sized,
        Self::Item: ToString,
    {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(s)
    }
}

impl<I> JoinStr for I where I: Iterator + ?Sized {}
