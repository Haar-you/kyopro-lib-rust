//! 永続スタック
//!
//! # Verification
//! - [ABC273 E - Notebook #37628467](https://atcoder.jp/contests/abc273/submissions/37628467)
use std::{iter::from_fn, rc::Rc};

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

/// 永続スタック
#[derive(Debug, Default, Clone)]
pub struct PersistentStack<T> {
    root: Option<Rc<Node<T>>>,
}

impl<T> PersistentStack<T> {
    /// 空の[`PersistentStack`]を生成する。
    pub fn new() -> Self {
        Self { root: None }
    }

    /// 末尾の要素への参照を返す。
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.value)
    }

    /// 値`value`を末尾に追加した[`PersistentStack`]を返す。
    pub fn push(&self, value: T) -> Self {
        Self {
            root: Some(Rc::new(Node {
                value,
                next: self.root.clone(),
            })),
        }
    }

    /// 末尾の要素を削除した[`PersistentStack`]を返す。
    pub fn pop(&self) -> Option<Self> {
        self.root.as_ref().map(|root| Self {
            root: root.next.clone(),
        })
    }

    /// スタックの末尾から先頭への要素の参照を返すイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut root = &self.root;

        from_fn(move || match root {
            None => None,
            Some(r) => {
                root = &r.next;
                Some(&r.value)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = PersistentStack::<u32>::new();
        let a = a.push(4).push(5).push(9);

        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), &[9, 5, 4]);

        let b = a.pop().unwrap();
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), &[9, 5, 4]);
        assert_eq!(b.iter().cloned().collect::<Vec<_>>(), &[5, 4]);

        let c = b.push(2);
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), &[9, 5, 4]);
        assert_eq!(b.iter().cloned().collect::<Vec<_>>(), &[5, 4]);
        assert_eq!(c.iter().cloned().collect::<Vec<_>>(), &[2, 5, 4]);
    }
}
