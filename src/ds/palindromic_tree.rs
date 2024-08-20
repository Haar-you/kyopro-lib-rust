#![allow(clippy::len_without_is_empty)]

use std::collections::{btree_map::Entry, BTreeMap};

const ODD: usize = 0;
const EVEN: usize = 1;
type Index = usize;

#[derive(Default)]
pub struct Node {
    length: isize,
    count: usize,
    index: Index,
    children: BTreeMap<char, Index>,
    suffix_link: Option<Index>,
    reverse_suffix_links: Vec<Index>,
    parent: Option<Index>,
}

impl Node {
    pub fn length(&self) -> isize {
        self.length
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn index(&self) -> Index {
        self.index
    }
}

pub struct PalindromicTree {
    list: Vec<Node>,
    sindex_list: Vec<Index>,
}

impl PalindromicTree {
    pub fn new(s: &str) -> Self {
        let s = s.chars().collect::<Vec<_>>();

        let even_root = Node {
            length: 0,
            index: EVEN,
            suffix_link: Some(ODD),
            ..Default::default()
        };

        let odd_root = Node {
            length: -1,
            index: ODD,
            reverse_suffix_links: vec![EVEN],
            ..Default::default()
        };

        let mut cur = odd_root.index;

        let mut list = vec![odd_root, even_root];
        let mut sindex_list = vec![];

        for (i, &c) in s.iter().enumerate() {
            let mut t = cur;

            loop {
                let k = i as isize - list[t].length - 1;
                if k >= 0 && c == s[k as usize] {
                    let index = list.len();
                    let t = &mut list[t];

                    match t.children.entry(c) {
                        Entry::Vacant(e) => {
                            let a = Node {
                                length: t.length + 2,
                                count: 1,
                                index,
                                parent: Some(t.index),
                                ..Default::default()
                            };

                            sindex_list.push(a.index);

                            e.insert(a.index);
                            list.push(a);
                        }
                        Entry::Occupied(e) => {
                            let t = *e.get();

                            list[t].count += 1;
                            sindex_list.push(list[t].index);
                        }
                    }

                    break;
                } else {
                    t = list[t].suffix_link.unwrap();
                }
            }

            let next = *list[t].children.get(&c).unwrap();

            if list[next].suffix_link.is_none() {
                if list[next].length == 1 {
                    list[next].suffix_link = Some(EVEN);
                    list[EVEN].reverse_suffix_links.push(next);
                } else {
                    let mut p = cur;

                    loop {
                        if p != t {
                            let k = i as isize - list[p].length - 1;
                            if k >= 0 && c == s[k as usize] {
                                list[next].suffix_link = Some(*list[p].children.get(&c).unwrap());

                                let ch = *list[p].children.get(&c).unwrap();
                                list[ch].reverse_suffix_links.push(next);
                                break;
                            } else {
                                p = list[p].suffix_link.unwrap();
                            }
                        } else {
                            p = list[p].suffix_link.unwrap();
                        }
                    }
                }
            }

            cur = next;
        }

        Self { list, sindex_list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.list.get(index)
    }

    pub fn get_from_strpos(&self, pos: usize) -> Option<&Node> {
        self.get_node(self.sindex_list[pos])
    }

    pub fn parent_of(&self, index: usize) -> Option<&Node> {
        let p = self.list.get(index)?.parent?;
        Some(&self.list[p])
    }

    pub fn suffix_link_of(&self, index: usize) -> Option<&Node> {
        let p = self.list.get(index)?.suffix_link?;
        Some(&self.list[p])
    }
}
