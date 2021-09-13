#![allow(clippy::many_single_char_names)]

use std::rc::Rc;

enum Node<T> {
    Terminal {
        value: T,
    },
    Internal {
        size: usize,
        l_ch: Option<Rc<Node<T>>>,
        r_ch: Option<Rc<Node<T>>>,
    },
}

pub struct PersistentArray<T> {
    size: usize,
    root: Option<Rc<Node<T>>>,
}

fn get_size<T>(node: Option<Rc<Node<T>>>) -> usize {
    node.map_or(0, |node| match *node {
        Node::<T>::Terminal { .. } => 1,
        Node::<T>::Internal { size, .. } => size,
    })
}

impl<T> PersistentArray<T>
where
    T: Clone,
{
    pub fn new(size: usize, value: T) -> Self {
        let depth = {
            let mut t = 1;
            while size > (1 << t) {
                t += 1;
            }
            t + 1
        };

        let root = Self::init(size, value, 1, depth);

        Self { size, root }
    }

    fn init(s: usize, value: T, d: usize, depth: usize) -> Option<Rc<Node<T>>> {
        if s == 0 {
            return None;
        }
        if d == depth {
            Some(Rc::new(Node::<T>::Terminal { value }))
        } else {
            let l = Self::init(s / 2, value.clone(), d + 1, depth);
            let r = Self::init(s - s / 2, value, d + 1, depth);

            let t = Node::<T>::Internal {
                size: get_size(l.clone()) + get_size(r.clone()),
                l_ch: l,
                r_ch: r,
            };

            Some(Rc::new(t))
        }
    }

    fn _traverse(node: Option<Rc<Node<T>>>, ret: &mut Vec<T>) {
        if let Some(node) = node {
            match *node {
                Node::<T>::Terminal { ref value } => {
                    ret.push(value.clone());
                }
                Node::<T>::Internal {
                    ref l_ch, ref r_ch, ..
                } => {
                    Self::_traverse(l_ch.clone(), ret);
                    Self::_traverse(r_ch.clone(), ret);
                }
            }
        }
    }

    fn _set(prev: Rc<Node<T>>, i: usize, value: T) -> Rc<Node<T>> {
        match *prev {
            Node::<T>::Terminal { .. } => Rc::new(Node::<T>::Terminal { value }),
            Node::<T>::Internal {
                ref l_ch, ref r_ch, ..
            } => {
                let k = get_size(l_ch.clone());
                let (l, r) = {
                    if i < k {
                        (
                            Some(Self::_set(Rc::clone(l_ch.as_ref().unwrap()), i, value)),
                            r_ch.clone(),
                        )
                    } else {
                        (
                            l_ch.clone(),
                            Some(Self::_set(Rc::clone(r_ch.as_ref().unwrap()), i - k, value)),
                        )
                    }
                };

                Rc::new(Node::<T>::Internal {
                    size: get_size(l.clone()) + get_size(r.clone()),
                    l_ch: l,
                    r_ch: r,
                })
            }
        }
    }

    pub fn set(&self, i: usize, value: T) -> Self {
        Self {
            size: self.size,
            root: Some(Self::_set(self.root.clone().unwrap(), i, value)),
        }
    }

    pub fn get(&self, i: usize) -> T {
        fn _get<T>(node: Rc<Node<T>>, i: usize) -> Rc<Node<T>> {
            match *node {
                Node::<T>::Terminal { .. } => node,
                Node::<T>::Internal {
                    ref l_ch, ref r_ch, ..
                } => {
                    let k = get_size(l_ch.clone());
                    if i < k {
                        _get(Rc::clone(l_ch.as_ref().unwrap()), i)
                    } else {
                        _get(Rc::clone(r_ch.as_ref().unwrap()), i - k)
                    }
                }
            }
        }

        match *_get(Rc::clone(self.root.as_ref().unwrap()), i) {
            Node::<T>::Terminal { ref value } => value.clone(),
            _ => {
                unreachable!();
            }
        }
    }
}

impl<T: Clone> From<&PersistentArray<T>> for Vec<T> {
    fn from(from: &PersistentArray<T>) -> Vec<T> {
        let mut ret = vec![];
        PersistentArray::<T>::_traverse(from.root.clone(), &mut ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = PersistentArray::<i32>::new(5, 0);
        assert_eq!(Vec::<i32>::from(&a), [0, 0, 0, 0, 0]);

        let b = a.set(0, 4);
        assert_eq!(Vec::<i32>::from(&b), [4, 0, 0, 0, 0]);

        let c = b.set(2, 6);
        assert_eq!(Vec::<i32>::from(&c), [4, 0, 6, 0, 0]);

        let d = b.set(2, 9);
        assert_eq!(Vec::<i32>::from(&d), [4, 0, 9, 0, 0]);

        assert_eq!(Vec::<i32>::from(&a), [0, 0, 0, 0, 0]);
        assert_eq!(Vec::<i32>::from(&b), [4, 0, 0, 0, 0]);
        assert_eq!(Vec::<i32>::from(&c), [4, 0, 6, 0, 0]);
        assert_eq!(Vec::<i32>::from(&d), [4, 0, 9, 0, 0]);
    }
}
