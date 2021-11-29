//! 座標圧縮
use crate::algo::bsearch::lower_bound;

#[derive(Clone)]
pub struct Compressor<T> {
    data: Vec<T>,
}

#[derive(Clone, Default)]
pub struct CompressorBuilder<T> {
    data: Vec<T>,
}

impl<T> Compressor<T>
where
    T: Clone + Ord + Eq,
{
    /// `value`が何番目の値(0-index)かを返す。
    /// # Complexity
    /// Time complexity O(log(n))
    pub fn index(&self, value: &T) -> usize {
        lower_bound(&self.data, value)
    }

    /// `i`番目の値を返す。
    /// # Complexity
    /// Time complexity O(1)
    pub fn get(&self, i: usize) -> &T {
        &self.data[i]
    }

    pub fn compress(&self, values: impl IntoIterator<Item = T>) -> Vec<usize> {
        values.into_iter().map(|x| self.index(&x)).collect()
    }

    pub fn decompress(&self, indices: impl IntoIterator<Item = usize>) -> Vec<&T> {
        indices.into_iter().map(|i| self.get(i)).collect()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T: Clone + Ord + Eq> CompressorBuilder<T> {
    pub fn new() -> Self {
        CompressorBuilder { data: vec![] }
    }

    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn add_vec(&mut self, values: impl IntoIterator<Item = T>) {
        self.data.extend(values.into_iter());
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
    use crate::hashset;
    use std::collections::HashSet;

    #[test]
    fn test() {
        let data = vec![1, 3, 2, 4, 5, 9, 0, -1, 3];
        let mut builder = CompressorBuilder::<_>::new();
        builder.add_vec(data.clone());
        let compressor = builder.build();

        assert_eq!(
            compressor.compress(data.clone()),
            vec![2, 4, 3, 5, 6, 7, 1, 0, 4]
        );
        assert_eq!(
            compressor
                .decompress(vec![2, 4, 3, 5, 6, 7, 1, 0, 4])
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
            data
        );

        let data = hashset![1, 3, 2, 4, 5, 9, 0, -1, 3];
        let mut builder = CompressorBuilder::<_>::new();
        builder.add_vec(data.clone());
        let compressor = builder.build();

        assert_eq!(
            compressor
                .compress(data.clone())
                .into_iter()
                .collect::<HashSet<_>>(),
            hashset![2, 4, 3, 5, 6, 7, 1, 0, 4]
        );
        assert_eq!(
            compressor
                .decompress(vec![2, 4, 3, 5, 6, 7, 1, 0, 4])
                .into_iter()
                .copied()
                .collect::<HashSet<_>>(),
            data
        );
    }
}
