//! ロールバック可能Vec
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc165/tasks/abc165_f>

use std::{
    fmt,
    fmt::{Debug, Formatter},
    ops::Index,
};

#[derive(Clone)]
enum History<T> {
    Update(T, usize),
    Push,
    Pop(T),
}

#[derive(Clone, Default)]
pub struct RollbackableVec<T> {
    data: Vec<T>,
    history: Vec<History<T>>,
}

impl<T> RollbackableVec<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            data: vec![],
            history: vec![],
        }
    }

    pub fn push(&mut self, value: T) {
        self.history.push(History::Push);
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        let x = self.data.pop()?;
        self.history.push(History::Pop(x.clone()));
        Some(x)
    }

    pub fn assign(&mut self, index: usize, value: T) {
        self.history
            .push(History::Update(self.data[index].clone(), index));
        self.data[index] = value;
    }

    pub fn rollback(&mut self) -> bool {
        match self.history.pop() {
            None => false,
            Some(x) => {
                match x {
                    History::Update(value, index) => {
                        self.data[index] = value;
                    }
                    History::Push => {
                        self.data.pop();
                    }
                    History::Pop(value) => {
                        self.data.push(value);
                    }
                }
                true
            }
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// スライスを返す
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// 先頭の要素を返す
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    /// 末尾の要素を返す
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }
}

impl<T> Index<usize> for RollbackableVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T: Debug> Debug for RollbackableVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone> From<&RollbackableVec<T>> for Vec<T> {
    fn from(from: &RollbackableVec<T>) -> Self {
        from.data.clone()
    }
}

impl<T> From<Vec<T>> for RollbackableVec<T> {
    fn from(from: Vec<T>) -> Self {
        Self {
            data: from,
            history: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut a = RollbackableVec::from(vec![1, 2, 3, 4]);

        assert_eq!(a.as_slice(), &[1, 2, 3, 4]);

        a.push(5);
        assert_eq!(a.as_slice(), &[1, 2, 3, 4, 5]);

        a.rollback();
        assert_eq!(a.as_slice(), &[1, 2, 3, 4]);

        a.pop();
        assert_eq!(a.as_slice(), &[1, 2, 3]);

        a.rollback();
        assert_eq!(a.as_slice(), &[1, 2, 3, 4]);

        a.assign(2, 9);
        assert_eq!(a.as_slice(), &[1, 2, 9, 4]);

        a.rollback();
        assert_eq!(a.as_slice(), &[1, 2, 3, 4]);
    }
}
