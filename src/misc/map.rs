pub trait Map {
    type Input;
    type C<T>;

    fn map<Output, F: FnMut(Self::Input) -> Output>(self, f: F) -> Self::C<Output>;
}

impl<T> Map for Vec<T> {
    type Input = T;
    type C<A> = Vec<A>;

    fn map<Output, F: FnMut(Self::Input) -> Output>(self, f: F) -> Self::C<Output> {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 2, 3];
        let _b = a.map(|x| x * 2);
    }
}
