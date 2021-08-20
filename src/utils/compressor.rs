use crate::algorithm::bsearch::lower_bound;

#[derive(Clone)]
pub struct Compressor<T> {
    data: Vec<T>,
}

#[derive(Clone)]
pub struct CompressorBuilder<T> {
    data: Vec<T>,
}

impl<T> Compressor<T>
where
    T: Clone + Ord + Eq,
{
    pub fn index(&self, value: T) -> usize {
        lower_bound(&self.data, &value)
    }

    pub fn get(&self, i: usize) -> T {
        self.data[i].clone()
    }

    pub fn compress(&self, values: &[T]) -> Vec<usize> {
        values.iter().map(|x| self.index(x.clone())).collect()
    }

    pub fn decompress(&self, indices: &[usize]) -> Vec<T> {
        indices.into_iter().map(|&i| self.get(i)).collect()
    }
}

impl<T: Clone + Ord + Eq> CompressorBuilder<T> {
    pub fn new() -> Self {
        CompressorBuilder { data: vec![] }
    }

    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn add_vec(&mut self, values: Vec<T>) {
        self.data.extend(values);
    }

    pub fn build(mut self) -> Compressor<T> {
        self.data.sort();
        self.data.dedup();
        Compressor { data: self.data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = vec![1, 3, 2, 4, 5, 9, 0, -1, 3];
        let mut builder = CompressorBuilder::<_>::new();
        builder.add_vec(data.clone());
        let compressor = builder.build();

        assert_eq!(compressor.compress(&data), vec![2, 4, 3, 5, 6, 7, 1, 0, 4]);
        assert_eq!(compressor.decompress(&[2, 4, 3, 5, 6, 7, 1, 0, 4]), data);
    }
}
