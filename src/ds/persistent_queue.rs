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

#[derive(Default, Debug)]
pub struct PersistentQueue<T> {
    #[allow(clippy::type_complexity)]
    front_back_node: Option<(Rc<Node<T>>, Rc<Node<T>>)>,
}

impl<T> PersistentQueue<T> {
    pub fn new(value: T) -> Self {
        let p = Rc::new(Node::new(value));

        Self {
            front_back_node: Some((p.clone(), p)),
        }
    }

    pub fn push(&self, value: T) -> Self {
        match self.front_back_node.clone() {
            None => {
                let p = Rc::new(Node::new(value));
                Self {
                    front_back_node: Some((p.clone(), p)),
                }
            }
            Some((front_node, back_node)) => {
                let mut t = Node::new(value);

                t.depth = back_node.depth + 1;

                t.ancestors.reserve(back_node.ancestors.len() + 1);
                t.ancestors.push(back_node);
                for i in 1.. {
                    match t.ancestors.get(i - 1).cloned() {
                        None => {
                            break;
                        }
                        Some(s) => {
                            if let Some(x) = s.ancestors.get(i - 1).cloned() {
                                t.ancestors.push(x);
                            } else {
                                break;
                            }
                        }
                    }
                }
                t.ancestors.shrink_to_fit();

                Self {
                    front_back_node: Some((front_node, Rc::new(t))),
                }
            }
        }
    }

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
                    let mut t = back_node.clone();

                    for i in (0..t.ancestors.len()).rev() {
                        if d >= (1 << i) {
                            d -= 1 << i;
                            t = t.ancestors[i].clone();
                        }
                        if d == 0 {
                            break;
                        }
                    }

                    Self {
                        front_back_node: Some((t, back_node.clone())),
                    }
                }
            })
    }

    pub fn front(&self) -> Option<&T> {
        self.front_back_node.as_ref().map(|(t, _)| &t.value)
    }

    pub fn back(&self) -> Option<&T> {
        self.front_back_node.as_ref().map(|(_, t)| &t.value)
    }

    pub fn is_empty(&self) -> bool {
        self.front_back_node.is_none()
    }

    pub fn len(&self) -> usize {
        self.front_back_node
            .as_ref()
            .map_or(0, |(f, b)| b.depth - f.depth + 1)
    }
}
