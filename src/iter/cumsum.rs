pub struct CumSum<I, St, F> {
    iter: I,
    st: St,
    is_first: bool,
    f: F,
}

impl<I, St, F> Iterator for CumSum<I, St, F>
where
    I: Iterator,
    St: Copy,
    F: FnMut(&mut St, I::Item),
{
    type Item = St;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            (self.f)(&mut self.st, self.iter.next()?);
        }
        Some(self.st)
    }
}

pub trait IterCumSum: Iterator {
    fn cumsum<St, F>(self, init: St, f: F) -> CumSum<Self, St, F>
    where
        Self: Sized,
        St: Copy,
        F: FnMut(&mut St, Self::Item),
    {
        CumSum {
            iter: self,
            st: init,
            is_first: true,
            f,
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
        let b = a.iter().cumsum(0, |acc, x| *acc += x).collect::<Vec<_>>();

        assert_eq!(b, vec![0, 1, 3, 6, 10, 15]);
    }
}
