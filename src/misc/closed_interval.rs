#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ClosedInterval<T> {
    pub start: T,
    pub end: T,
}

impl<T> ClosedInterval<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T: Ord + PartialEq + Copy> ClosedInterval<T> {
    pub fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        if self.end < other.start || other.end < self.start {
            Err((self, other))
        } else {
            Ok(Self {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        type I = ClosedInterval<i64>;

        assert_eq!(
            I::new(0, 3).merge(I::new(4, 7)),
            Err((I::new(0, 3), I::new(4, 7)))
        );
        assert_eq!(I::new(0, 3).merge(I::new(3, 6)), Ok(I::new(0, 6)));
        assert_eq!(I::new(0, 3).merge(I::new(-2, 5)), Ok(I::new(-2, 5)));
        assert_eq!(I::new(0, 3).merge(I::new(-2, 2)), Ok(I::new(-2, 3)));
        assert_eq!(I::new(0, 3).merge(I::new(1, 2)), Ok(I::new(0, 3)));
    }
}
