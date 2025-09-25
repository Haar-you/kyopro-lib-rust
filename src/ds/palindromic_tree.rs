//! 回文木
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/eertree>

#![allow(clippy::len_without_is_empty)]

use std::collections::{btree_map::Entry, BTreeMap};

const ODD: usize = 0;
const EVEN: usize = 1;

/// [`PalindromicTree`]のノード
#[derive(Default)]
pub struct Node {
    length: isize,
    count: usize,
    index: usize,
    children: BTreeMap<char, *mut Node>,
    parent: Option<*mut Node>,
    suffix_link: Option<*mut Node>,
    reverse_suffix_links: Vec<*mut Node>,
}

impl Node {
    /// 回文の長さを返す。
    pub fn length(&self) -> isize {
        self.length
    }
    /// ノードが表す回文の出現回数を返す。
    pub fn count(&self) -> usize {
        self.count
    }
    /// ノードに割り当てられた番号を返す。
    pub fn index(&self) -> usize {
        self.index
    }

    /// 親ノードへの参照を返す。
    pub fn parent(&self) -> Option<&Self> {
        self.parent.map(|p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }

    /// 接尾辞リンクの行き先ノードへの参照を返す。
    pub fn suffix_link(&self) -> Option<&Self> {
        self.suffix_link.map(|p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }

    /// 子ノードへの推移文字と子ノードへの参照へのイテレータを返す。
    pub fn children(&self) -> impl Iterator<Item = (char, &Self)> {
        self.children.iter().map(|(&k, &v)| {
            assert!(!v.is_null());
            (k, unsafe { &*v })
        })
    }

    /// 接尾辞リンクを逆に辿ったノードへの参照へのイテレータを返す。
    pub fn rev_suffix_links(&self) -> impl Iterator<Item = &Self> {
        self.reverse_suffix_links.iter().map(|&p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }
}

fn index_of(p: *mut Node) -> usize {
    assert!(!p.is_null());
    unsafe { (*p).index }
}

fn length_of(p: *mut Node) -> isize {
    assert!(!p.is_null());
    unsafe { (*p).length }
}

fn suffix_link_of(p: *mut Node) -> Option<*mut Node> {
    assert!(!p.is_null());
    unsafe { (*p).suffix_link }
}

fn child_of(p: *mut Node, c: char) -> Option<*mut Node> {
    assert!(!p.is_null());
    unsafe { (*p).children.get(&c).copied() }
}

fn set_suffix_link(from: *mut Node, to: *mut Node) {
    assert!(!from.is_null());
    assert!(!to.is_null());

    unsafe {
        (*from).suffix_link = Some(to);
        (*to).reverse_suffix_links.push(from);
    }
}

/// 回文木
pub struct PalindromicTree {
    list: Vec<*mut Node>,
    sindex_list: Vec<usize>,
    cur: *mut Node,
    s: Vec<char>,
}

impl PalindromicTree {
    /// 文字列`s`から回文木を構築する。
    pub fn new(s: &str) -> Self {
        let even_root = Box::new(Node {
            length: 0,
            index: EVEN,
            ..Default::default()
        });
        let even_root = Box::into_raw(even_root);

        let odd_root = Box::new(Node {
            length: -1,
            index: ODD,
            ..Default::default()
        });
        let odd_root = Box::into_raw(odd_root);

        set_suffix_link(even_root, odd_root);

        let s = s.chars().collect::<Vec<_>>();
        let mut this = Self {
            list: vec![odd_root, even_root],
            sindex_list: vec![],
            cur: odd_root,
            s: vec![],
        };

        for c in s.into_iter() {
            this.push(c);
        }

        this
    }

    /// 末尾に文字`c`を追加する。
    pub fn push(&mut self, c: char) {
        let i = self.s.len();
        self.s.push(c);

        let mut t = self.cur;

        loop {
            let t_index = index_of(t);

            let k = i as isize - length_of(self.list[t_index]) - 1;
            if k >= 0 && c == self.s[k as usize] {
                let index = self.list.len();
                let t = self.list[t_index];

                assert!(!t.is_null());
                match unsafe { (*t).children.entry(c) } {
                    Entry::Vacant(e) => {
                        let a = Box::new(Node {
                            length: length_of(t) + 2,
                            count: 1,
                            index,
                            parent: Some(self.list[index_of(t)]),
                            ..Default::default()
                        });

                        self.sindex_list.push(a.index);

                        let a = Box::into_raw(a);

                        e.insert(a);
                        self.list.push(a);
                    }
                    Entry::Occupied(e) => {
                        let t = index_of(*e.get());

                        assert!(!self.list[t].is_null());
                        unsafe { (*self.list[t]).count += 1 };
                        self.sindex_list.push(index_of(self.list[t]));
                    }
                }

                break;
            }

            t = suffix_link_of(self.list[t_index]).unwrap();
        }

        let next = child_of(self.list[index_of(t)], c).unwrap();
        let next_index = index_of(next);

        if suffix_link_of(self.list[next_index]).is_none() {
            if length_of(self.list[next_index]) == 1 {
                set_suffix_link(next, self.list[EVEN]);
            } else {
                let mut p = self.cur;

                loop {
                    let p_index = index_of(p);

                    if p != t {
                        let k = i as isize - length_of(self.list[p_index]) - 1;

                        if k >= 0 && c == self.s[k as usize] {
                            let ch = child_of(self.list[p_index], c).unwrap();
                            set_suffix_link(next, ch);
                            break;
                        }
                    }
                    p = suffix_link_of(self.list[p_index]).unwrap();
                }
            }
        }

        self.cur = next;
    }

    /// 回文木に含まれるノードの個数を返す。(長さ`0`,`-1`のノードも含む。)
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// 奇数長回文の木の根への参照を返す。
    pub fn odd_root(&self) -> &Node {
        unsafe { &*self.list[ODD] }
    }

    /// 偶数長回文の木の根への参照を返す。
    pub fn even_root(&self) -> &Node {
        unsafe { &*self.list[EVEN] }
    }

    /// `index`番目のノードへの参照を返す。
    pub fn node_of(&self, index: usize) -> Option<&Node> {
        self.list.get(index).map(|&p| {
            assert!(!p.is_null());
            unsafe { &*p }
        })
    }

    /// 元の文字列の長さ`pos+1`の接頭辞の最大回文接尾辞に対応するノードへの参照を返す。
    pub fn node_from_strpos(&self, pos: usize) -> Option<&Node> {
        self.node_of(self.sindex_list[pos])
    }
}
