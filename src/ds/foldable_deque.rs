//! 半群で畳み込み可能なdeque
pub use crate::algebra::traits::*;

#[derive(Clone, Default, Debug)]
/// 半群で畳み込み可能なdeque
pub struct FoldableDeque<S: Semigroup> {
    front_stack: Vec<S>,
    back_stack: Vec<S>,
    front_sum: Vec<S>,
    back_sum: Vec<S>,
}

impl<S: Semigroup + Clone> FoldableDeque<S> {
    /// 空の`FoldableDeque<S>`を生成する。
    pub fn new() -> Self {
        FoldableDeque {
            front_stack: vec![],
            back_stack: vec![],
            front_sum: vec![],
            back_sum: vec![],
        }
    }

    fn f(&self, a: Option<S>, b: Option<S>) -> Option<S> {
        match (a, b) {
            (Some(a), Some(b)) => Some(S::op(a, b)),
            (x @ Some(_), _) => x,
            (_, x @ Some(_)) => x,
            (_, _) => None,
        }
    }

    /// すべての要素を`S`の演算で畳み込んだ結果を返す。
    pub fn fold(&self) -> Option<S> {
        self.f(
            self.front_sum.last().cloned(),
            self.back_sum.last().cloned(),
        )
    }

    /// 末尾に`value`を追加する。
    pub fn push_back(&mut self, value: S) {
        self.back_stack.push(value.clone());
        self.back_sum
            .push(self.f(self.back_sum.last().cloned(), Some(value)).unwrap());
    }

    /// 先頭に`value`を追加する。
    pub fn push_front(&mut self, value: S) {
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

    /// 先頭の要素を削除して返す。
    pub fn pop_front(&mut self) -> Option<S> {
        if self.front_stack.is_empty() {
            self.back_sum.clear();

            let n = self.back_stack.len();
            if n == 0 {
                return None;
            }

            self.front_stack = self.back_stack.split_off(n.div_ceil(2));
            std::mem::swap(&mut self.front_stack, &mut self.back_stack);
            self.front_stack.reverse();

            self.build_sum();
        }

        self.front_sum.pop();
        self.front_stack.pop()
    }

    /// 末尾の要素を削除して返す。
    pub fn pop_back(&mut self) -> Option<S> {
        if self.back_stack.is_empty() {
            self.front_sum.clear();

            let n = self.front_stack.len();
            if n == 0 {
                return None;
            }

            self.back_stack = self.front_stack.split_off(n.div_ceil(2));
            std::mem::swap(&mut self.front_stack, &mut self.back_stack);
            self.back_stack.reverse();

            self.build_sum();
        }

        self.back_sum.pop();
        self.back_stack.pop()
    }

    /// 先頭の要素への参照を返す。
    pub fn front(&self) -> Option<&S> {
        self.front_stack.last().or_else(|| self.back_stack.first())
    }

    /// 末尾の要素への参照を返す。
    pub fn back(&self) -> Option<&S> {
        self.back_stack.last().or_else(|| self.front_stack.first())
    }

    /// 要素数を返す。
    pub fn len(&self) -> usize {
        self.front_stack.len() + self.back_stack.len()
    }

    /// 要素数が`0`なら`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.front_stack.is_empty() && self.back_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra::affine::*, num::const_modint::ConstModInt};
    use rand::Rng;
    use std::collections::VecDeque;

    const M: u32 = 998244353;
    type Mint = ConstModInt<M>;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let mut deq = VecDeque::<Affine<Mint>>::new();
        let mut swag = FoldableDeque::<Affine<Mint>>::new();

        for _ in 0..1000 {
            assert_eq!(deq.front(), swag.front());
            assert_eq!(deq.back(), swag.back());
            assert_eq!(deq.len(), swag.len());

            let ty = rng.gen_range(0..5);

            match ty {
                0 => {
                    let a = Mint::new(rng.gen_range(0..M));
                    let b = Mint::new(rng.gen_range(0..M));
                    deq.push_front(Affine(a, b));
                    swag.push_front(Affine(a, b));
                }
                1 => {
                    let a = Mint::new(rng.gen_range(0..M));
                    let b = Mint::new(rng.gen_range(0..M));
                    deq.push_back(Affine(a, b));
                    swag.push_back(Affine(a, b));
                }
                2 => {
                    assert_eq!(deq.pop_front(), swag.pop_front());
                }
                3 => {
                    assert_eq!(deq.pop_back(), swag.pop_back());
                }
                4 => {
                    assert_eq!(
                        deq.iter().cloned().fold_m(),
                        swag.fold().unwrap_or(Affine::id())
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}
