//! 座標圧縮
use crate::algo::bsearch_slice::BinarySearch;

/// 座標圧縮のための構造体
#[derive(Clone)]
pub struct Compressor<T> {
    data: Vec<T>,
}

/// [`Compressor<T>`]を生成する
#[derive(Clone, Default)]
pub struct CompressorBuilder<T> {
    data: Vec<T>,
}

impl<T: Ord + Eq> Compressor<T> {
    /// `value`が何番目の値(0-index)かを返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn index(&self, value: &T) -> usize {
        self.data.lower_bound(value)
    }

    /// `i`番目の値を返す。
    ///
    /// **Time complexity** $O(1)$
    pub fn get(&self, i: usize) -> &T {
        &self.data[i]
    }

    /// 最小値を返す。
    pub fn min(&self) -> Option<&T> {
        self.data.first()
    }

    /// 最大値を返す。
    pub fn max(&self) -> Option<&T> {
        self.data.last()
    }

    /// `values`の要素をすべて座標圧縮する。
    pub fn compress<'a>(
        &'a self,
        values: impl IntoIterator<Item = T> + 'a,
    ) -> impl Iterator<Item = usize> + 'a {
        values.into_iter().map(move |x| self.index(&x))
    }

    /// `values`の要素をすべて復元する。
    pub fn decompress<'a>(
        &'a self,
        indices: impl IntoIterator<Item = usize> + 'a,
    ) -> impl Iterator<Item = &'a T> + 'a {
        indices.into_iter().map(move |i| self.get(i))
    }

    /// 座標圧縮後の要素の種類数
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T: Ord + Eq> CompressorBuilder<T> {
    /// `CompressorBuilder<T>`を生成する。
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// 座標圧縮対象に`value`を追加する。
    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    /// **Time complexity** $O(n \log n)$
    pub fn build(mut self) -> Compressor<T> {
        self.data.sort();
        self.data.dedup();
        Compressor { data: self.data }
    }
}

impl<U> Extend<U> for CompressorBuilder<U> {
    fn extend<T: IntoIterator<Item = U>>(&mut self, iter: T) {
        self.data.extend(iter);
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
        builder.extend(data.clone());
        let compressor = builder.build();

        assert_eq!(
            compressor.compress(data.clone()).collect::<Vec<_>>(),
            vec![2, 4, 3, 5, 6, 7, 1, 0, 4]
        );
        assert_eq!(
            compressor
                .decompress(vec![2, 4, 3, 5, 6, 7, 1, 0, 4])
                .copied()
                .collect::<Vec<_>>(),
            data
        );

        let data = hashset![1, 3, 2, 4, 5, 9, 0, -1, 3];
        let mut builder = CompressorBuilder::<_>::new();
        builder.extend(data.clone());
        let compressor = builder.build();

        assert_eq!(
            compressor.compress(data.clone()).collect::<HashSet<_>>(),
            hashset![2, 4, 3, 5, 6, 7, 1, 0, 4]
        );
        assert_eq!(
            compressor
                .decompress(vec![2, 4, 3, 5, 6, 7, 1, 0, 4])
                .copied()
                .collect::<HashSet<_>>(),
            data
        );
    }
}
