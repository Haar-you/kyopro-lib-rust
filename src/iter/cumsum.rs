use std::ops::AddAssign;

pub struct CumSum<I, T> {
    iter: I,
    st: T,
    is_first: bool,
}

impl<I> Iterator for CumSum<I, I::Item>
where
    I: Iterator,
    I::Item: AddAssign + Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            self.st += self.iter.next()?;
        }
        Some(self.st)
    }
}

pub trait IterCumSum: Iterator {
    fn cumsum(self, init: Self::Item) -> CumSum<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: AddAssign + Copy,
    {
        CumSum {
            iter: self,
            st: init,
            is_first: true,
        }
    }
}

impl<I> IterCumSum for I where I: Iterator + ?Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.into_iter().cumsum(0).collect::<Vec<_>>();

        assert_eq!(b, vec![0, 1, 3, 6, 10, 15]);
    }
}
