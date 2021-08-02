use crate::utils::lower_bound::*;

#[derive(Clone)]
pub struct Compressor<T> {
    data: Vec<T>
}

#[derive(Clone)]
pub struct CompressorBuilder<T> {
    data: Vec<T>
}

impl<T> Compressor<T>
where
    T: Clone + Ord + Eq
{
    pub fn index(&self, value: T) -> usize {
        lower_bound(&self.data, value)
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

impl<T> CompressorBuilder<T>
where
    T: Clone + Ord + Eq
{
    pub fn new() -> Self {
        CompressorBuilder {
            data: vec![]
        }
    }

    pub fn add(&mut self, value: T) -> &mut Self{
        self.data.push(value);
        self
    }

    pub fn add_vec(&mut self, values: Vec<T>) -> &mut Self {
        self.data.extend(values);
        self
    }

    pub fn build(&mut self) -> Compressor<T> {
        self.data.sort();
        self.data.dedup();
        Compressor {
            data: self.data.clone()
        }
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
