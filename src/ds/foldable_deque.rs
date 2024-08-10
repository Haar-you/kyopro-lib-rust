pub use crate::algebra::traits::*;

pub struct FoldableDeque<S: Semigroup> {
    front_stack: Vec<S::Output>,
    back_stack: Vec<S::Output>,
    front_sum: Vec<S::Output>,
    back_sum: Vec<S::Output>,
    semigroup: S,
}

impl<T: Clone, S: Semigroup<Output = T>> FoldableDeque<S> {
    pub fn new(semigroup: S) -> Self {
        FoldableDeque {
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
            (_, _) => None,
        }
    }

    pub fn fold(&self) -> Option<T> {
        self.f(
            self.front_sum.last().cloned(),
            self.back_sum.last().cloned(),
        )
    }

    pub fn push_back(&mut self, value: T) {
        self.back_stack.push(value.clone());
        self.back_sum
            .push(self.f(self.back_sum.last().cloned(), Some(value)).unwrap());
    }

    pub fn push_front(&mut self, value: T) {
        self.front_stack.push(value.clone());
        self.front_sum
            .push(self.f(Some(value), self.front_sum.last().cloned()).unwrap());
    }

    fn build_sum(&mut self) {
        for value in &self.front_stack {
            self.front_sum.push(
                self.f(Some(value.clone()), self.front_sum.last().cloned())
                    .unwrap(),
            );
        }

        for value in &self.back_stack {
            self.back_sum.push(
                self.f(self.back_sum.last().cloned(), Some(value.clone()))
                    .unwrap(),
            );
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.front_stack.is_empty() {
            self.back_sum.clear();

            let n = self.back_stack.len();
            if n == 0 {
                return None;
            }

            self.front_stack = self.back_stack.split_off((n + 1) / 2);
            std::mem::swap(&mut self.front_stack, &mut self.back_stack);
            self.front_stack.reverse();

            self.build_sum();
        }

        self.front_sum.pop();
        self.front_stack.pop()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.back_stack.is_empty() {
            self.front_sum.clear();

            let n = self.front_stack.len();
            if n == 0 {
                return None;
            }

            self.back_stack = self.front_stack.split_off((n + 1) / 2);
            std::mem::swap(&mut self.front_stack, &mut self.back_stack);
            self.back_stack.reverse();

            self.build_sum();
        }

        self.back_sum.pop();
        self.back_stack.pop()
    }

    pub fn front(&self) -> Option<&T> {
        self.front_stack.last().or_else(|| self.back_stack.first())
    }

    pub fn back(&self) -> Option<&T> {
        self.back_stack.last().or_else(|| self.front_stack.first())
    }

    pub fn len(&self) -> usize {
        self.front_stack.len() + self.back_stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.front_stack.is_empty() && self.back_stack.is_empty()
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra::affine::*, math::ff::modint::*, modulo};
    use rand::Rng;
    use std::collections::VecDeque;

    const M: u32 = 998244353;
    modulo!(Mod, M);
    type Mint = ModInt<Mod>;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let monoid = Affine::<Mint>::new();

        let mut deq = VecDeque::<(Mint, Mint)>::new();
        let mut swag = FoldableDeque::new(monoid.clone());

        for _ in 0..1000 {
            assert_eq!(deq.front(), swag.front());
            assert_eq!(deq.back(), swag.back());
            assert_eq!(deq.len(), swag.len());

            let ty = rng.gen_range(0..5);

            match ty {
                0 => {
                    let a = rng.gen_range(0..M).into();
                    let b = rng.gen_range(0..M).into();
                    deq.push_front((a, b));
                    swag.push_front((a, b));
                }
                1 => {
                    let a = rng.gen_range(0..M).into();
                    let b = rng.gen_range(0..M).into();
                    deq.push_back((a, b));
                    swag.push_back((a, b));
                }
                2 => {
                    assert_eq!(deq.pop_front(), swag.pop_front());
                }
                3 => {
                    assert_eq!(deq.pop_back(), swag.pop_back());
                }
                4 => {
                    assert_eq!(
                        deq.iter()
                            .fold((1.into(), 0.into()), |acc, &x| monoid.op(acc, x)),
                        swag.fold().unwrap_or(monoid.id())
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}
*/
