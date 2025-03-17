//! Trie木
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc353/tasks/abc353_e>
//! - <https://atcoder.jp/contests/abc377/tasks/abc377_g>
use std::collections::HashMap;
use std::hash::Hash;

/// Trie木のノード
#[derive(Clone, Debug)]
pub struct TrieNode<T, K> {
    /// ノードに格納している値
    pub value: T,
    children: HashMap<K, *mut Self>,
}

impl<T, K> TrieNode<T, K> {
    fn new(value: T) -> Self {
        Self {
            value,
            children: HashMap::default(),
        }
    }
}

impl<T, K: Copy + Hash + Eq> TrieNode<T, K> {
    /// 子ノードへのキーと子ノードへの参照をもつイテレータを返す。
    pub fn children_nodes(&self) -> impl Iterator<Item = (K, &Self)> {
        self.children
            .clone()
            .into_iter()
            .map(|(k, v)| (k, unsafe { &*v }))
    }

    /// 子ノードへのキーと子ノードへの可変参照をもつイテレータを返す。
    pub fn children_nodes_mut(&mut self) -> impl Iterator<Item = (K, &mut Self)> {
        self.children
            .clone()
            .into_iter()
            .map(|(k, v)| (k, unsafe { &mut *v }))
    }

    fn add<I, FI, F1, F2>(
        &mut self,
        iter: &mut I,
        mut init: FI,
        mut proc: F1,
        rproc: &mut F2,
        prefix: &mut Vec<K>,
    ) where
        I: Iterator<Item = K>,
        FI: FnMut(&Vec<K>) -> T,
        F1: FnMut(&mut T, &Vec<K>),
        F2: FnMut(&mut T, &Vec<K>),
    {
        proc(&mut self.value, prefix);
        if let Some(c) = iter.next() {
            prefix.push(c);

            let next = if let Some(&next) = self.children.get(&c) {
                next
            } else {
                let value = init(prefix);
                let next = Box::new(Self::new(value));
                let next = Box::into_raw(next);
                self.children.insert(c, next);
                next
            };

            assert!(!next.is_null());
            unsafe { &mut *next }.add(iter, init, proc, rproc, prefix);
            prefix.pop();
        }
        rproc(&mut self.value, prefix);
    }
}

/// Trie木
pub struct Trie<T, K> {
    root: *mut TrieNode<T, K>,
}

impl<T, K: Copy + Hash + Eq> Trie<T, K> {
    /// 値`value`を保持するルートのみをもつ[`Trie`]を構築する。
    pub fn new(value: T) -> Self {
        let root = Box::new(TrieNode::new(value));
        let root = Box::into_raw(root);
        Self { root }
    }

    /// Trie木の根ノードへの参照を返す。
    pub fn root_node(&self) -> &TrieNode<T, K> {
        assert!(!self.root.is_null());
        unsafe { &*self.root }
    }

    /// Trie木の根ノードへの可変参照を返す。
    pub fn root_node_mut(&mut self) -> &mut TrieNode<T, K> {
        assert!(!self.root.is_null());
        unsafe { &mut *self.root }
    }

    /// 列`s`をTrie木に追加する。
    ///
    /// - `init`: ノードが新しく追加されるときの初期値を決定する。
    /// - `proc`: 行きがけ順に、ノードの値に処理をする。
    /// - `rproc`: 帰りがけ順に、ノードの値に処理をする。
    pub fn add<I, FI, F1, F2>(&mut self, s: I, init: FI, proc: F1, mut rproc: F2)
    where
        I: IntoIterator<Item = K>,
        FI: FnMut(&Vec<K>) -> T,
        F1: FnMut(&mut T, &Vec<K>),
        F2: FnMut(&mut T, &Vec<K>),
    {
        let mut s = s.into_iter();
        assert!(!self.root.is_null());
        unsafe { &mut *self.root }.add(&mut s, init, proc, &mut rproc, &mut vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Debug, Display};

    fn dfs<T, K>(node: &mut TrieNode<T, K>, prefix: &mut Vec<K>)
    where
        T: Default + Display,
        K: Copy + Hash + Eq + Debug,
    {
        let depth = prefix.len();
        let key = prefix.last();
        println!("{:->depth$} {:?} {}", "", key, node.value);
        for (key, ch) in node.children_nodes_mut() {
            prefix.push(key);
            dfs(ch, prefix);
            prefix.pop();
        }
    }

    #[test]
    fn test() {
        let mut trie = Trie::<u32, char>::new(0);

        let init = |prefix: &Vec<char>| -> u32 { prefix.len() as u32 };

        let proc = |value: &mut u32, prefix: &Vec<char>| {
            println!("{:?}", prefix);
            *value += 1;
        };

        let rproc = |_: &mut u32, prefix: &Vec<char>| {
            println!("{:?}", prefix);
        };

        trie.add("abc".chars(), init, proc, rproc);
        trie.add("abra".chars(), init, proc, rproc);
        trie.add("baa".chars(), init, proc, rproc);

        let root = trie.root_node_mut();
        dfs(root, &mut vec![]);
    }
}
