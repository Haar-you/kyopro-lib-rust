use std::ops::{Add, Range, Sub};

pub struct Imos1D<T> {
    data: Vec<T>,
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T>> Imos1D<T> {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![T::default(); n],
        }
    }

    pub fn update(&mut self, Range { start: l, end: r }: Range<usize>, value: T) {
        self.data[l] = self.data[l] + value;
        if let Some(x) = self.data.get_mut(r) {
            *x = *x - value;
        }
    }

    pub fn build(mut self) -> Vec<T> {
        for i in 1..self.data.len() {
            self.data[i] = self.data[i] + self.data[i - 1];
        }

        self.data
    }
}
