pub trait JoinStr: Iterator {
    fn join_str(self, s: &str) -> String
    where
        Self: Sized,
        Self::Item: ToString,
    {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(s)
    }
}

impl<I> JoinStr for I where I: Iterator + ?Sized {}
