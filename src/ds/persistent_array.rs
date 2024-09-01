//! 永続配列

use std::rc::Rc;

#[derive(Clone)]
enum Node<T> {
    Terminal {
        value: Rc<T>,
    },
    Internal {
        size: usize,
        l_ch: Option<Rc<Node<T>>>,
        r_ch: Option<Rc<Node<T>>>,
    },
}

#[derive(Clone)]
pub struct PersistentArray<T> {
    size: usize,
    root: Option<Rc<Node<T>>>,
}

fn get_size<T>(node: &Option<Rc<Node<T>>>) -> usize {
    node.as_ref().map_or(0, |node| match node.as_ref() {
        Node::Terminal { .. } => 1,
        Node::Internal { size, .. } => *size,
    })
}

impl<T> PersistentArray<T>
where
    T: Clone,
{
    /// **Time complexity O(n)**
    pub fn new(size: usize, value: T) -> Self {
        if size == 0 {
            Self {
                size: 0,
                root: None,
            }
        } else {
            let depth = usize::BITS - (size - 1_usize).leading_zeros() + 1;
            let values = vec![value; size];
            let root = Self::_init(0, size, &values, depth);

            Self { size, root }
        }
    }

    fn _init(l: usize, r: usize, values: &[T], depth: u32) -> Option<Rc<Node<T>>> {
        if l == r {
            return None;
        }
        if depth == 1 {
            Some(Rc::new(Node::Terminal {
                value: Rc::new(values[l].clone()),
            }))
        } else {
            let mid = (l + r) / 2;
            let l_ch = Self::_init(l, mid, values, depth - 1);
            let r_ch = Self::_init(mid, r, values, depth - 1);

            let t = Node::Internal {
                size: get_size(&l_ch) + get_size(&r_ch),
                l_ch,
                r_ch,
            };

            Some(Rc::new(t))
        }
    }

    fn _traverse(node: &Option<Rc<Node<T>>>, ret: &mut Vec<T>) {
        if let Some(node) = node {
            match node.as_ref() {
                Node::Terminal { value } => {
                    ret.push(value.as_ref().clone());
                }
                Node::Internal { l_ch, r_ch, .. } => {
                    Self::_traverse(l_ch, ret);
                    Self::_traverse(r_ch, ret);
                }
            }
        }
    }

    fn _set(prev: &Rc<Node<T>>, i: usize, value: T) -> Rc<Node<T>> {
        match prev.as_ref() {
            Node::Terminal { .. } => Rc::new(Node::Terminal {
                value: Rc::new(value),
            }),
            Node::Internal { l_ch, r_ch, .. } => {
                let (l_ch, r_ch) = {
                    let k = get_size(l_ch);
                    if i < k {
                        (
                            Some(Self::_set(l_ch.as_ref().unwrap(), i, value)),
                            r_ch.clone(),
                        )
                    } else {
                        (
                            l_ch.clone(),
                            Some(Self::_set(r_ch.as_ref().unwrap(), i - k, value)),
                        )
                    }
                };

                Rc::new(Node::Internal {
                    size: get_size(&l_ch) + get_size(&r_ch),
                    l_ch,
                    r_ch,
                })
            }
        }
    }

    /// **Time complexity O(log n)**
    pub fn set(&self, i: usize, value: T) -> Self {
        assert!(
            i < self.size,
            "index out of bounds: the len is {} but the index is {}",
            self.size,
            i
        );
        Self {
            size: self.size,
            root: Some(Self::_set(self.root.as_ref().unwrap(), i, value)),
        }
    }

    fn _get(node: &Rc<Node<T>>, i: usize) -> Rc<Node<T>> {
        match node.as_ref() {
            Node::Terminal { .. } => Rc::clone(node),
            Node::Internal { l_ch, r_ch, .. } => {
                let k = get_size(l_ch);
                if i < k {
                    Self::_get(l_ch.as_ref().unwrap(), i)
                } else {
                    Self::_get(r_ch.as_ref().unwrap(), i - k)
                }
            }
        }
    }

    /// **Time complexity O(log n)**
    pub fn get(&self, i: usize) -> Rc<T> {
        assert!(
            i < self.size,
            "index out of bounds: the len is {} but the index is {}",
            self.size,
            i
        );
        match Self::_get(self.root.as_ref().unwrap(), i).as_ref() {
            Node::Terminal { value } => Rc::clone(value),
            _ => unreachable!(),
        }
    }
}

impl<T: Clone> From<&PersistentArray<T>> for Vec<T> {
    fn from(from: &PersistentArray<T>) -> Vec<T> {
        let mut ret = vec![];
        PersistentArray::<T>::_traverse(&from.root, &mut ret);
        ret
    }
}

impl<T: Clone> From<Vec<T>> for PersistentArray<T> {
    fn from(value: Vec<T>) -> Self {
        let size = value.len();
        if size == 0 {
            Self {
                size: 0,
                root: None,
            }
        } else {
            let depth = usize::BITS - (size - 1_usize).leading_zeros() + 1;
            let root = Self::_init(0, size, &value, depth);

            Self { size, root }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = PersistentArray::<i32>::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(Vec::<i32>::from(&a), [1, 2, 3, 4, 5]);

        let b = a.set(0, 4);
        assert_eq!(Vec::<i32>::from(&b), [4, 2, 3, 4, 5]);

        let c = b.set(2, 6);
        assert_eq!(Vec::<i32>::from(&c), [4, 2, 6, 4, 5]);

        let d = b.set(2, 9);
        assert_eq!(Vec::<i32>::from(&d), [4, 2, 9, 4, 5]);

        let e = c.set(4, -3);

        assert_eq!(Vec::<i32>::from(&a), [1, 2, 3, 4, 5]);
        assert_eq!(Vec::<i32>::from(&b), [4, 2, 3, 4, 5]);
        assert_eq!(Vec::<i32>::from(&c), [4, 2, 6, 4, 5]);
        assert_eq!(Vec::<i32>::from(&d), [4, 2, 9, 4, 5]);
        assert_eq!(Vec::<i32>::from(&e), [4, 2, 6, 4, -3]);
    }
}
