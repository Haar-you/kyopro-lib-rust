//! Aho-Corasick法
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/430>
//! - <https://atcoder.jp/contests/abc362/tasks/abc362_g>
//! - <https://atcoder.jp/contests/abc268/tasks/abc268_h>
use std::collections::{HashMap, VecDeque};

/// [`AhoCorasick`]のノード
pub struct Node {
    index: usize,
    children: HashMap<char, *mut Self>,
    failure_link: Option<*mut Self>,
    rev_failure_links: Vec<*mut Self>,
}

impl Node {
    fn new(index: usize) -> Self {
        Self {
            index,
            children: HashMap::new(),
            failure_link: None,
            rev_failure_links: vec![],
        }
    }

    /// ノード毎に割り当てられた番号を返す。
    pub fn index(&self) -> usize {
        self.index
    }

    /// 文字`c`で遷移する子ノードへの参照を返す。
    pub fn child(&self, c: char) -> Option<&Self> {
        self.children.get(&c).map(|&p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }

    /// 子ノードへ遷移できないときに辿るべきノードへの参照を返す。
    pub fn failure_link(&self) -> Option<&Self> {
        self.failure_link.map(|p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }

    /// failure_linkを逆に辿ったノードへの参照へのイテレータを返す。
    pub fn rev_failure_links(&self) -> impl Iterator<Item = &Self> {
        self.rev_failure_links.iter().map(|&p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }
}

fn index_of(p: *mut Node) -> usize {
    assert!(!p.is_null());
    unsafe { (*p).index }
}

fn child_of(p: *mut Node, c: char) -> Option<*mut Node> {
    assert!(!p.is_null());
    unsafe { (*p).children.get(&c).copied() }
}

fn failure_link_of(p: *mut Node) -> Option<*mut Node> {
    assert!(!p.is_null());
    unsafe { (*p).failure_link }
}

fn set_failure_link(from: *mut Node, to: *mut Node) {
    assert!(!from.is_null());
    unsafe {
        (*from).failure_link = Some(to);
        (*to).rev_failure_links.push(from);
    }
}

/// [`AhoCorasick`]を構築するための構造体。
pub struct AhoCorasickBuilder {
    size: usize,
    root: *mut Node,
    dict: Vec<String>,
    dict_index: Vec<Vec<usize>>,
    nodes: Vec<*mut Node>,
}

#[allow(clippy::new_without_default)]
impl AhoCorasickBuilder {
    /// [`AhoCorasickBuilder`]を生成する。
    pub fn new() -> Self {
        let root = Box::new(Node::new(0));
        let root = Box::into_raw(root);

        Self {
            size: 1,
            root,
            dict: vec![],
            dict_index: vec![],
            nodes: vec![],
        }
    }

    /// パターン`pat`を追加する。
    pub fn add(&mut self, pat: &str) {
        self.dict.push(pat.to_owned());

        let mut cur = self.root;

        for c in pat.chars() {
            assert!(!cur.is_null());
            if let Some(next) = child_of(cur, c) {
                cur = next;
            } else {
                let new = Box::new(Node::new(self.size));
                let new = Box::into_raw(new);

                assert!(!cur.is_null());
                unsafe { (*cur).children.insert(c, new) };

                cur = new;
                self.size += 1;
            }
        }

        self.nodes.push(cur);

        self.dict_index.resize(self.size, vec![]);
        self.dict_index[index_of(cur)].push(self.dict.len() - 1);
    }

    /// [`AhoCorasick`]を構築する。
    pub fn build(self) -> AhoCorasick {
        let mut dq = VecDeque::new();
        dq.push_back(self.root);

        while let Some(cur) = dq.pop_front() {
            assert!(!cur.is_null());
            for (&c, &next) in unsafe { (*cur).children.iter() } {
                if cur == self.root {
                    set_failure_link(next, cur);
                } else {
                    let mut i = failure_link_of(cur).unwrap();
                    let mut j = self.root;

                    loop {
                        if let Some(t) = child_of(i, c) {
                            j = t;
                            break;
                        }
                        let Some(t) = failure_link_of(i) else {
                            break;
                        };
                        i = t;
                    }

                    set_failure_link(next, j);
                }

                dq.push_back(next);
            }
        }

        AhoCorasick {
            size: self.size,
            root: self.root,
            dict: self.dict,
            dict_index: self.dict_index,
            nodes: self.nodes,
        }
    }
}

/// Aho-Corasick法
pub struct AhoCorasick {
    size: usize,
    root: *mut Node,
    dict: Vec<String>,
    dict_index: Vec<Vec<usize>>,
    nodes: Vec<*mut Node>,
}

#[allow(clippy::len_without_is_empty)]
impl AhoCorasick {
    /// ノード数を返す。
    pub fn len(&self) -> usize {
        self.size
    }

    /// Trie木の根ノードへの参照を返す。
    pub fn root_node(&self) -> &Node {
        unsafe { &*self.root }
    }

    /// `index`番目に追加したパターンに対応するノードへの参照を返す。
    pub fn node_of(&self, index: usize) -> &Node {
        assert!(!self.nodes[index].is_null());
        unsafe { &*self.nodes[index] }
    }

    /// 文字列`s`がマッチするすべてのパターンを列挙する。
    pub fn matches<F>(&self, s: &str, mut proc: F)
    where
        F: FnMut(usize, std::ops::Range<usize>),
    {
        let mut cur = self.root;

        for (i, c) in s.chars().enumerate() {
            while cur != self.root && unsafe { !(*cur).children.contains_key(&c) } {
                cur = failure_link_of(cur).unwrap();
            }

            cur = child_of(cur, c).unwrap_or(self.root);

            let mut p = cur;

            loop {
                for &j in &self.dict_index[index_of(p)] {
                    let len = self.dict[j].len();
                    proc(j, i + 1 - len..i + 1);
                }

                let Some(q) = failure_link_of(p) else { break };
                p = q;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    fn test() {
        let mut builder = AhoCorasickBuilder::new();

        builder.add("ur");
        builder.add("et");
        builder.add("ur");

        let ac = builder.build();

        let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        ac.matches(s, |index, range| {
            let Range { start, end } = range;
            println!(
                "{} {}\x1b[m\x1b[1m\x1b[32m{}\x1b[m{}",
                index,
                s.get(start.saturating_sub(3)..start).unwrap(),
                s.get(start..end).unwrap(),
                s.get(end..end.saturating_add(3)).unwrap()
            );
        })
    }
}
