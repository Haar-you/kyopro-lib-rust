pub trait VecMap<T> {
    fn map<U, F>(self, f: F) -> Vec<U>
    where
        F: FnMut(T) -> U;
}

impl<T> VecMap<T> for Vec<T> {
    fn map<U, F>(self, f: F) -> Vec<U>
    where
        F: FnMut(T) -> U,
    {
        self.into_iter().map(f).collect()
    }
}
