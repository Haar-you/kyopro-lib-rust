use std::ops::Add;

pub struct _CumSum<I, T> {
    iter: I,
    st: T,
    is_first: bool,
}

impl<I: Iterator<Item = T>, T: Add<Output = T> + Clone> Iterator for _CumSum<I, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(self.st.clone())
        } else {
            let t = self.st.clone() + self.iter.next()?;
            self.st = t;
            Some(self.st.clone())
        }
    }
}

pub trait CumSum: Iterator {
    fn cumsum(self, init: Self::Item) -> _CumSum<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Add<Output = Self::Item> + Clone,
    {
        _CumSum {
            iter: self,
            st: init,
            is_first: true,
        }
    }
}

impl<I> CumSum for I where I: Iterator + ?Sized {}
