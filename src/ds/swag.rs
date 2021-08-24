pub use crate::algebra::traits::*;

pub struct SlidingWindowAggregation<T, S> {
    front_stack: Vec<T>,
    back_stack: Vec<T>,
    front_sum: Vec<T>,
    back_sum: Vec<T>,
    semigroup: S,
}

impl<T, S> SlidingWindowAggregation<T, S>
where
    T: Clone,
    S: Semigroup<Output = T>,
{
    pub fn new(semigroup: S) -> Self {
        SlidingWindowAggregation {
            front_stack: vec![],
            back_stack: vec![],
            front_sum: vec![],
            back_sum: vec![],
            semigroup,
        }
    }

    fn f(&self, a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(self.semigroup.op(a, b)),
            (x @ Some(_), _) => x,
            (_, x @ Some(_)) => x,
            (_, _) => None
        }
    }

    pub fn fold(&self) -> Option<T> {
        self.f(
            self.front_sum.last().cloned(),
            self.back_sum.last().cloned(),
        )
    }

    pub fn push(&mut self, value: T) {
        self.back_stack.push(value.clone());
        self.back_sum
            .push(self.f(self.back_sum.last().cloned(), Some(value)).unwrap());
    }

    pub fn pop(&mut self) {
        if self.front_stack.is_empty() {
            self.back_sum.clear();

            while let Some(value) = self.back_stack.pop() {
                self.front_stack.push(value.clone());
                self.front_sum
                    .push(self.f(Some(value), self.front_sum.last().cloned()).unwrap());
            }
        }

        self.front_stack.pop();
        self.front_sum.pop();
    }
}
