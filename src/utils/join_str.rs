pub trait JoinStr {
    fn join_str(self, _: &str) -> String;
}

impl<T, I> JoinStr for I
where
    T: ToString,
    I: Iterator<Item = T>
{
    fn join_str(self, s: &str) -> String {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(s)
    }
}
