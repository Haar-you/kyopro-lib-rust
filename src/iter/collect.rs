pub trait CollectVec: Iterator {
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }
}

impl<I> CollectVec for I where I: Iterator + ?Sized {}
