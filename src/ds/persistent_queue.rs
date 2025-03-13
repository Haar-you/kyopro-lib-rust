//! 永続キュー

use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    ancestors: Vec<Rc<Node<T>>>,
    depth: usize,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            ancestors: vec![],
            depth: 0,
        }
    }
}

/// 永続キュー
#[derive(Default, Debug)]
pub struct PersistentQueue<T> {
    #[allow(clippy::type_complexity)]
    front_back_node: Option<(Rc<Node<T>>, Rc<Node<T>>)>,
}

impl<T> PersistentQueue<T> {
    /// 値`value`をただ一つだけもつ[`PersistentQueue`]を生成する。
    pub fn new(value: T) -> Self {
        let p = Rc::new(Node::new(value));

        Self {
            front_back_node: Some((Rc::clone(&p), p)),
        }
    }

    /// 値`value`を末尾に追加した[`PersistentQueue`]を返す。
    pub fn push(&self, value: T) -> Self {
        match self.front_back_node.as_ref() {
            None => {
                let p = Rc::new(Node::new(value));
                Self {
                    front_back_node: Some((Rc::clone(&p), p)),
                }
            }
            Some((front_node, back_node)) => {
                let mut t = Node::new(value);

                t.depth = back_node.depth + 1;

                t.ancestors.reserve(back_node.ancestors.len() + 1);
                t.ancestors.push(Rc::clone(back_node));
                for i in 1.. {
                    match t.ancestors.get(i - 1) {
                        None => {
                            break;
                        }
                        Some(s) => {
                            if let Some(x) = s.ancestors.get(i - 1) {
                                t.ancestors.push(Rc::clone(x));
                            } else {
                                break;
                            }
                        }
                    }
                }
                t.ancestors.shrink_to_fit();

                Self {
                    front_back_node: Some((Rc::clone(front_node), Rc::new(t))),
                }
            }
        }
    }

    /// 先頭の要素を削除した[`PersistentQueue`]を返す。    
    pub fn pop(&self) -> Option<Self> {
        self.front_back_node
            .as_ref()
            .map(|(front_node, back_node)| {
                if front_node.depth == back_node.depth {
                    Self {
                        front_back_node: None,
                    }
                } else {
                    let mut d = back_node.depth - front_node.depth - 1;
                    let mut t = back_node;

                    for i in (0..t.ancestors.len()).rev() {
                        if d >= (1 << i) {
                            d -= 1 << i;
                            t = &t.ancestors[i];
                        }
                        if d == 0 {
                            break;
                        }
                    }

                    Self {
                        front_back_node: Some((Rc::clone(t), Rc::clone(back_node))),
                    }
                }
            })
    }

    /// 先頭の要素への参照を返す。
    pub fn front(&self) -> Option<&T> {
        self.front_back_node.as_ref().map(|(t, _)| &t.value)
    }

    /// 末尾の要素への参照を返す。
    pub fn back(&self) -> Option<&T> {
        self.front_back_node.as_ref().map(|(_, t)| &t.value)
    }

    /// キューが空ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.front_back_node.is_none()
    }

    /// キューの要素数を返す。
    pub fn len(&self) -> usize {
        self.front_back_node
            .as_ref()
            .map_or(0, |(f, b)| b.depth - f.depth + 1)
    }
}
