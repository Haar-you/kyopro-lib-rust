use std::collections::BTreeMap;

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

        let mut even_root = Node::default();
        even_root.length = 0;
        even_root.index = EVEN;
        even_root.suffix_link = Some(ODD);

        let mut odd_root = Node::default();
        odd_root.length = -1;
        odd_root.index = ODD;
        odd_root.reverse_suffix_links.push(EVEN);

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

                    if t.children.get(&c).is_none() {
                        let mut a = Node::default();
                        a.length = t.length + 2;
                        a.count = 1;
                        a.index = index;
                        a.parent = Some(t.index);
                        sindex_list.push(a.index);

                        t.children.insert(c, a.index);
                        list.push(a);
                    } else {
                        let t = *t.children.get(&c).unwrap();

                        list[t].count += 1;
                        sindex_list.push(list[t].index);
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
