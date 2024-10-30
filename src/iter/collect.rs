//! `collect_vec`を提供する。

/// `collect_vec`を提供する。
pub trait CollectVec: Iterator {
    /// `.collect::<Vec<_>>()`と同じ。
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }
}

impl<I> CollectVec for I where I: Iterator + ?Sized {}
