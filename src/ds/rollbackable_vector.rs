#[derive(Clone)]
enum RollbackableVecHistoryType<T> {
    Update(T, usize),
    Push,
    Pop(T),
}

#[derive(Clone)]
pub struct RollbackableVec<T> {
    data: Vec<T>,
    history: Vec<RollbackableVecHistoryType<T>>,
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
        self.history.push(RollbackableVecHistoryType::Push);
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            let x = self.data.pop();
            self.history
                .push(RollbackableVecHistoryType::Pop(x.as_ref().unwrap().clone()));
            x
        }
    }

    pub fn assign(&mut self, index: usize, value: T) {
        self.history.push(RollbackableVecHistoryType::Update(
            self.data[index].clone(),
            index,
        ));
        self.data[index] = value;
    }

    pub fn rollback(&mut self) -> bool {
        if self.history.len() == 0 {
            false
        } else {
            match self.history.pop().unwrap() {
                RollbackableVecHistoryType::Update(value, index) => {
                    self.data[index] = value;
                }
                RollbackableVecHistoryType::Push => {
                    self.data.pop();
                }
                RollbackableVecHistoryType::Pop(value) => {
                    self.data.push(value);
                }
            }

            true
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> std::ops::Index<usize> for RollbackableVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for RollbackableVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
