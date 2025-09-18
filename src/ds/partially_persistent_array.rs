//! 部分永続配列

/// 部分永続配列
#[derive(Clone, Debug)]
pub struct PartiallyPersistentArray<T> {
    time: usize,
    data: Vec<Vec<(usize, T)>>,
}

impl<T: Clone> PartiallyPersistentArray<T> {
    /// 値`value`を`n`個もつ配列を作る。
    pub fn new(value: T, n: usize) -> Self {
        Self {
            time: 0,
            data: vec![vec![(0, value)]; n],
        }
    }

    /// 最新時刻を返す。
    pub fn latest(&self) -> usize {
        self.time
    }

    /// 最新の配列の`index`番目の要素を`value`で更新する。
    pub fn set(&mut self, index: usize, value: T) {
        self.time += 1;
        self.data[index].push((self.time, value));
    }

    /// 時刻`time`での配列の`index`番目の要素を返す。
    pub fn get(&self, index: usize, time: usize) -> &T {
        match self.data[index].binary_search_by_key(&time, |a| a.0) {
            Ok(i) => &self.data[index][i].1,
            Err(i) => &self.data[index][i - 1].1,
        }
    }

    /// 時刻`time`での配列へのイテレータを返す。
    pub fn iter_at(&self, time: usize) -> impl Iterator<Item = &T> {
        self.data
            .iter()
            .map(move |a| match a.binary_search_by_key(&time, |a| a.0) {
                Ok(i) => &a[i].1,
                Err(i) => &a[i - 1].1,
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::collect::CollectVec;

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 10;
        let t = 100;
        let mut a = PartiallyPersistentArray::new(0, n);
        let mut b = vec![0; n];

        let mut history = vec![b.clone()];

        for _ in 0..t {
            let i = rng.gen_range(0..n);
            let x = rng.gen::<u32>();

            b[i] = x;
            a.set(i, x);

            history.push(b.clone());
        }

        for time in 0..=t {
            assert_eq!(history[time], a.iter_at(time).cloned().collect_vec());
        }
    }
}
