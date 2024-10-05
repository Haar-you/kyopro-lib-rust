//! 連結リスト
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc225/tasks/abc225_d>

#[derive(Clone)]
/// 連結リストの内部ノード
pub struct Node<T> {
    /// `Node`がもつ値
    pub value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

#[derive(Clone, Default)]
/// 複数の連結リストをまとめたもの
pub struct LinkedListPool<T> {
    data: Vec<Node<T>>,
}

impl<T> LinkedListPool<T> {
    /// `LinkedListPool`を生成する。
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// `prev`の後ろに`next`を接続する。
    pub fn concat(&mut self, prev: usize, next: usize) -> bool {
        if self.data[prev].next.is_none() && self.data[next].prev.is_none() {
            self.data[prev].next = Some(next);
            self.data[next].prev = Some(prev);
            true
        } else {
            false
        }
    }

    /// `cur`の前でリストを切断する。
    pub fn split_before(&mut self, cur: usize) {
        if let Some(prev) = self.data[cur].prev.take() {
            self.data[prev].next = None;
        }
    }

    /// `cur`の後ろでリストを切断する。
    pub fn split_after(&mut self, cur: usize) {
        if let Some(next) = self.data[cur].next.take() {
            self.data[next].prev = None;
        }
    }

    /// `cur`の次の要素
    pub fn next_of(&self, cur: usize) -> Option<usize> {
        self.data[cur].next
    }

    /// `cur`の前の要素
    pub fn prev_of(&self, cur: usize) -> Option<usize> {
        self.data[cur].prev
    }

    /// `value`を値としてもつ単一要素の連結リストを追加する。
    pub fn push(&mut self, value: T) {
        self.data.push(Node::new(value));
    }

    /// `cur`が属する連結リストの先頭を返す。
    ///
    /// **Time complexity O(n)**
    pub fn first_of(&self, mut cur: usize) -> usize {
        while let Some(prev) = self.data[cur].prev {
            cur = prev;
        }
        cur
    }

    /// `cur`が属する連結リストの末尾を返す。
    ///
    /// **Time complexity O(n)**
    pub fn last_of(&self, mut cur: usize) -> usize {
        while let Some(next) = self.data[cur].next {
            cur = next;
        }
        cur
    }

    /// `cur`から`cur`の属する連結リストの終端まで走査するイテレータを返す。
    pub fn iter(&self, cur: usize) -> impl Iterator<Item = &T> {
        let mut cur = Some(cur);
        std::iter::from_fn(move || match cur {
            Some(c) => {
                cur = self.data[c].next;
                Some(&self.data[c].value)
            }
            _ => None,
        })
    }

    /// `cur`から`cur`の属する連結リストの先頭まで走査するイテレータを返す。
    pub fn riter(&self, cur: usize) -> impl Iterator<Item = &T> {
        let mut cur = Some(cur);
        std::iter::from_fn(move || match cur {
            Some(c) => {
                cur = self.data[c].prev;
                Some(&self.data[c].value)
            }
            _ => None,
        })
    }
}
