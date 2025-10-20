//! 循環配列
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc372/tasks/abc372_f>

use std::{
    fmt::{Debug, Error, Formatter},
    ops::{Index, IndexMut},
};

/// 循環配列
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct CircularArray<T> {
    data: Vec<T>,
    shift: usize,
}

impl<T> CircularArray<T> {
    /// 右に`n`要素分だけ回転させる。
    pub fn rotate_right(&mut self, n: usize) {
        self.shift = (self.shift + n) % self.data.len();
    }

    /// 左に`n`要素分だけ回転させる。
    pub fn rotate_left(&mut self, n: usize) {
        let len = self.data.len();
        self.shift = (self.shift + len - n % len) % len;
    }

    /// 各要素への参照のイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let k = self.real_index(0);
        let (b, a) = self.data.split_at(k);
        a.iter().chain(b.iter())
    }

    /// 各要素への可変参照のイテレータを返す。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let k = self.real_index(0);
        let (b, a) = self.data.split_at_mut(k);
        a.iter_mut().chain(b.iter_mut())
    }

    fn real_index(&self, index: usize) -> usize {
        let len = self.data.len();
        assert_ne!(len, 0);
        (len + index - self.shift) % len
    }

    /// 配列の長さを返す。
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 配列の長さが`0`かを返す。
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Index<usize> for CircularArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.real_index(index)]
    }
}

impl<T> IndexMut<usize> for CircularArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = self.real_index(index);
        &mut self.data[index]
    }
}

impl<T> IntoIterator for CircularArray<T> {
    type Item = T;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<T>, std::vec::IntoIter<T>>;
    fn into_iter(mut self) -> Self::IntoIter {
        let k = self.real_index(0);
        let a = self.data.split_off(k);
        a.into_iter().chain(self.data)
    }
}

impl<T> From<Vec<T>> for CircularArray<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value,
            shift: 0,
        }
    }
}

impl<T: Debug> Debug for CircularArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = CircularArray::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        eprintln!("{:?}", &a);

        a.rotate_left(1);
        eprintln!("{:?}", &a);

        a.rotate_left(2);
        eprintln!("{:?}", &a);

        a.rotate_right(5);
        eprintln!("{:?}", &a);

        // for i in 0..100 {
        //     dbg!(a[i]);
        // }
    }
}
