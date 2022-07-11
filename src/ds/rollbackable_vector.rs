//! ロールバック可能Vec

use std::{
    fmt,
    fmt::{Debug, Formatter},
    ops::Index,
};

#[derive(Clone)]
enum HistoryType<T> {
    Update(T, usize),
    Push,
    Pop(T),
}

#[derive(Clone, Default)]
pub struct RollbackableVec<T> {
    data: Vec<T>,
    history: Vec<HistoryType<T>>,
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
        self.history.push(HistoryType::Push);
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let x = self.data.pop();
            self.history
                .push(HistoryType::Pop(x.as_ref().unwrap().clone()));
            x
        }
    }

    pub fn assign(&mut self, index: usize, value: T) {
        self.history.push(HistoryType::Update(
            self.data[index].clone(),
            index,
        ));
        self.data[index] = value;
    }

    pub fn rollback(&mut self) -> bool {
        if self.history.is_empty() {
            false
        } else {
            match self.history.pop().unwrap() {
                HistoryType::Update(value, index) => {
                    self.data[index] = value;
                }
                HistoryType::Push => {
                    self.data.pop();
                }
                HistoryType::Pop(value) => {
                    self.data.push(value);
                }
            }

            true
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
mod tests {
    #[test]
    fn test() {}
}
