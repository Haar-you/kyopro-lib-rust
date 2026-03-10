//! 永続配列

use std::cmp::Ordering;
use std::ops::Index;
use std::ptr;

enum Value<T> {
    Value(T),
    Ptr(*const Node<T>),
}

struct Node<T> {
    value: Value<T>,
    index: usize,
    left: *const Self,
    right: *const Self,
}

impl<T> Node<T> {
    fn new(value: Value<T>, index: usize, left: *const Self, right: *const Self) -> Self {
        Self {
            value,
            index,
            left,
            right,
        }
    }

    fn ref_value<'a>(node: *const Self) -> &'a T {
        assert!(!node.is_null());
        let node = unsafe { &*node };

        match &node.value {
            Value::Value(x) => x,
            &Value::Ptr(p) => {
                assert!(!p.is_null());
                match unsafe { &(*p).value } {
                    Value::Value(x) => x,
                    _ => unreachable!(),
                }
            }
        }
    }
}

/// 永続配列
#[derive(Clone)]
pub struct PersistentArray<T> {
    size: usize,
    root: *const Node<T>,
}

impl<T> PersistentArray<T>
where
    T: Clone,
{
    /// `n`個の`value`からなる永続配列を作る。
    ///
    /// **Time complexity** $O(n)$
    pub fn new(n: usize, value: T) -> Self {
        Self::from_vec(vec![value; n])
    }

    fn _traverse(node: *const Node<T>, ret: &mut Vec<T>) {
        if !node.is_null() {
            let node = unsafe { &*node };
            Self::_traverse(node.left, ret);
            ret.push(Node::ref_value(node).clone());
            Self::_traverse(node.right, ret);
        }
    }

    /// `Vec`へ変換する。
    ///
    /// **Time complexity** $O(n)$
    pub fn into_vec(&self) -> Vec<T> {
        let mut ret = vec![];
        Self::_traverse(self.root, &mut ret);
        ret
    }
}

impl<T> PersistentArray<T> {
    /// `Vec`から永続配列を作る。
    ///
    /// **Time complexity** $O(n)$
    pub fn from_vec(v: Vec<T>) -> Self {
        if v.is_empty() {
            Self {
                size: 0,
                root: ptr::null_mut(),
            }
        } else {
            let size = v.len();

            let mut a = v
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    Box::into_raw(Box::new(Node::new(
                        Value::Value(x),
                        i,
                        ptr::null(),
                        ptr::null(),
                    )))
                })
                .collect::<Vec<_>>();

            let max = (size + 1).next_power_of_two();

            let get_par = |i: usize| {
                let lowest = 1 << i.trailing_zeros();
                i ^ lowest | (lowest << 1)
            };

            for i in 0..size {
                let i = i + 1;

                let mut par = get_par(i);
                while par <= max {
                    if par <= size {
                        let p = unsafe { &mut *a[par - 1] };
                        if par < i {
                            p.right = a[i - 1];
                        } else {
                            p.left = a[i - 1];
                        }

                        break;
                    }
                    par = get_par(par);
                }
            }

            let root = a[(max >> 1) - 1];
            Self { size, root }
        }
    }

    fn _set(prev: *const Node<T>, i: usize, val: T) -> *const Node<T> {
        assert!(!prev.is_null());
        let prev = unsafe { &*prev };

        let (left, right, value);
        match i.cmp(&prev.index) {
            Ordering::Less => {
                left = Self::_set(prev.left, i, val);
                right = prev.right;
                value = match prev.value {
                    Value::Value(_) => Value::Ptr(prev as *const _),
                    Value::Ptr(p) => Value::Ptr(p),
                };
            }
            Ordering::Greater => {
                left = prev.left;
                right = Self::_set(prev.right, i, val);
                value = match prev.value {
                    Value::Value(_) => Value::Ptr(prev as *const _),
                    Value::Ptr(p) => Value::Ptr(p),
                };
            }
            Ordering::Equal => {
                left = prev.left;
                right = prev.right;
                value = Value::Value(val);
            }
        }

        let node = Box::new(Node::new(value, prev.index, left, right));
        Box::into_raw(node)
    }

    /// `i`番目の要素を`value`に変更した永続配列を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn set(&self, i: usize, value: T) -> Self {
        assert!(
            i < self.size,
            "index out of bounds: the len is {} but the index is {i}",
            self.size,
        );
        Self {
            size: self.size,
            root: Self::_set(self.root, i, value),
        }
    }

    fn _get<'a>(node: *const Node<T>, i: usize) -> &'a T {
        assert!(!node.is_null());
        let node = unsafe { &*node };

        match i.cmp(&node.index) {
            Ordering::Less => Self::_get(node.left, i),
            Ordering::Greater => Self::_get(node.right, i),
            Ordering::Equal => Node::ref_value(node),
        }
    }

    /// `i`番目の要素を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn get(&self, i: usize) -> &T {
        assert!(
            i < self.size,
            "index out of bounds: the len is {} but the index is {i}",
            self.size
        );
        Self::_get(self.root, i)
    }
}

impl<T: Clone> Index<usize> for PersistentArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = PersistentArray::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(a.into_vec(), [1, 2, 3, 4, 5]);

        let b = a.set(0, 4);
        assert_eq!(b.into_vec(), [4, 2, 3, 4, 5]);

        let c = b.set(2, 6);
        assert_eq!(c.into_vec(), [4, 2, 6, 4, 5]);

        let d = b.set(2, 9);
        assert_eq!(d.into_vec(), [4, 2, 9, 4, 5]);

        let e = c.set(4, -3);

        assert_eq!(a.into_vec(), [1, 2, 3, 4, 5]);
        assert_eq!(b.into_vec(), [4, 2, 3, 4, 5]);
        assert_eq!(c.into_vec(), [4, 2, 6, 4, 5]);
        assert_eq!(d.into_vec(), [4, 2, 9, 4, 5]);
        assert_eq!(e.into_vec(), [4, 2, 6, 4, -3]);
    }
}
