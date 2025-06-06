//! 融合可能ヒープ

use std::mem::swap;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn meld(&mut self, other: Option<Box<Node<T>>>) {
        if let Some(mut other) = other {
            if self.value < other.value {
                swap(self, other.as_mut());
            }

            match self.right.as_mut() {
                Some(right) => right.meld(Some(other)),
                None => self.right = Some(other),
            }

            swap(&mut self.left, &mut self.right);
        }
    }
}

/// 融合可能ヒープ
#[derive(Debug, Clone, Default)]
pub struct SkewHeap<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> SkewHeap<T> {
    /// 空の[`SkewHeap<T>`]を生成する。
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// 他の`SkewHeap<T>`を融合する。
    pub fn meld(&mut self, other: SkewHeap<T>) {
        self.size += other.size;
        match self.root.as_mut() {
            None => self.root = other.root,
            Some(root) => root.meld(other.root),
        }
    }

    /// 値`value`を挿入する。
    pub fn push(&mut self, value: T) {
        self.size += 1;
        let t = Some(Box::new(Node::new(value)));
        match self.root.as_mut() {
            None => self.root = t,
            Some(root) => root.meld(t),
        }
    }

    /// ヒープの最大値を返す。
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.value)
    }

    /// 最大値をヒープから削除して、その値を返す。
    pub fn pop(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(root) => {
                self.size -= 1;

                let Node {
                    value: x,
                    left,
                    right,
                } = *root;
                match left {
                    None => self.root = right,
                    Some(mut left) => {
                        left.meld(right);
                        self.root = Some(left);
                    }
                }

                Some(x)
            }
        }
    }

    /// ヒープに含まれている値の個数を返す。
    pub fn len(&self) -> usize {
        self.size
    }

    /// ヒープが空ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::BinaryHeap;

    #[test]
    fn test() {
        let mut heap = SkewHeap::<u32>::new();
        let mut bheap = BinaryHeap::<u32>::new();

        let mut heap2 = SkewHeap::<u32>::new();
        let mut bheap2 = BinaryHeap::<u32>::new();

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let x = rng.gen::<u32>();
            heap.push(x);
            bheap.push(x);
        }

        for _ in 0..100 {
            let x = rng.gen::<u32>();
            heap2.push(x);
            bheap2.push(x);
        }

        heap.meld(heap2);

        while let Some(x) = bheap2.pop() {
            bheap.push(x);
        }

        while let (Some(x), Some(y)) = (heap.pop(), bheap.pop()) {
            assert_eq!(x, y);
        }
    }
}
