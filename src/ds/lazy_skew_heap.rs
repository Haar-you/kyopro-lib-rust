//! 遅延加算付き融合可能ヒープ

use crate::trait_alias;
use std::{mem::swap, ops::AddAssign};

trait_alias!(Elem, Ord + Default + Copy + AddAssign);

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    lazy: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Elem> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            lazy: T::default(),
            left: None,
            right: None,
        }
    }

    fn propagate(&mut self) {
        self.value += self.lazy;
        if let Some(left) = &mut self.left {
            left.as_mut().lazy += self.lazy;
        }
        if let Some(right) = &mut self.right {
            right.as_mut().lazy += self.lazy;
        }
        self.lazy = T::default();
    }

    pub fn meld(&mut self, other: Option<Box<Node<T>>>) {
        self.propagate();
        if let Some(mut other) = other {
            other.as_mut().propagate();

            if self.value < other.value {
                swap(self, &mut other.as_mut());
            }

            match self.right.as_mut() {
                Some(right) => right.meld(Some(other)),
                None => self.right = Some(other),
            }

            swap(&mut self.left, &mut self.right);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LazySkewHeap<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Elem> LazySkewHeap<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn meld(&mut self, other: LazySkewHeap<T>) {
        self.size += other.size;
        match self.root.as_mut() {
            None => self.root = other.root,
            Some(root) => root.meld(other.root),
        }
    }

    pub fn push(&mut self, value: T) {
        self.size += 1;
        let t = Some(Box::new(Node::new(value)));
        match self.root.as_mut() {
            None => self.root = t,
            Some(root) => root.meld(t),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(root) => {
                self.size -= 1;

                let Node {
                    value: x,
                    left,
                    right,
                    ..
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

    pub fn add(&mut self, value: T) {
        if let Some(root) = self.root.as_mut() {
            root.lazy += value;
            root.propagate();
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

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
        let mut heap = LazySkewHeap::<u32>::new();
        let mut bheap = BinaryHeap::<u32>::new();

        let mut heap2 = LazySkewHeap::<u32>::new();
        let mut bheap2 = BinaryHeap::<u32>::new();

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let x = rng.gen::<u32>() % 10000;
            heap.push(x);
            bheap.push(x);
        }

        let x = rng.gen::<u32>() % 10000;
        heap.add(x);
        bheap = bheap.into_iter().map(|a| a + x).collect::<BinaryHeap<_>>();

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
