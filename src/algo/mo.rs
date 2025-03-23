//! Mo's algorithm
//!
//! # Problems
//! - <https://atcoder.jp/contests/dwacon2017-honsen/tasks/dwango2017final_b>

/// Mo's algorithm
pub struct Mo<'a> {
    append_left: Box<dyn 'a + FnMut(usize)>,
    append_right: Box<dyn 'a + FnMut(usize)>,
    remove_left: Box<dyn 'a + FnMut(usize)>,
    remove_right: Box<dyn 'a + FnMut(usize)>,
    query: Box<dyn 'a + FnMut(usize)>,
    q: usize,
    width: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    ord: Vec<usize>,
}

impl<'a> Mo<'a> {
    /// クエリ処理用の関数を登録して[`Mo`]を作る。
    pub fn new(
        n: usize,
        append_left: Box<impl 'a + FnMut(usize)>,
        append_right: Box<impl 'a + FnMut(usize)>,
        remove_left: Box<impl 'a + FnMut(usize)>,
        remove_right: Box<impl 'a + FnMut(usize)>,
        query: Box<impl 'a + FnMut(usize)>,
    ) -> Self {
        Self {
            append_left,
            append_right,
            remove_left,
            remove_right,
            query,
            q: 0,
            width: (n as f64).sqrt() as usize,
            left: vec![],
            right: vec![],
            ord: vec![],
        }
    }

    /// 範囲`[l, r)`を登録する。
    pub fn add(&mut self, l: usize, r: usize) {
        self.left.push(l);
        self.right.push(r);
        self.ord.push(self.q);
        self.q += 1;
    }

    /// 登録した範囲をすべて訪れるように実行する。
    pub fn run(mut self) {
        let left = self.left;
        let right = self.right;
        let width = self.width;

        self.ord.sort_by(|&i, &j| {
            let a = left[i] / width;
            let b = left[j] / width;

            if a == b {
                if a % 2 == 1 {
                    right[i].cmp(&right[j])
                } else {
                    right[j].cmp(&right[i])
                }
            } else {
                a.cmp(&b)
            }
        });

        let mut l = left[self.ord[0]];
        let mut r = left[self.ord[0]];

        for id in self.ord {
            let left = left[id];
            let right = right[id];

            while l != left || r != right {
                if l > left {
                    l -= 1;
                    (self.append_left)(l);
                } else if l < left {
                    (self.remove_left)(l);
                    l += 1;
                }
                if r < right {
                    (self.append_right)(r);
                    r += 1;
                } else if r > right {
                    r -= 1;
                    (self.remove_right)(r);
                }
            }

            (self.query)(id);
        }
    }
}
