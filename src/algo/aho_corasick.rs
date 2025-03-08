use std::collections::{HashMap, VecDeque};

pub struct AhoCorasickBuilder {
    size: usize,
    trie: Vec<HashMap<char, usize>>,
    failure_edge: Vec<usize>,
    dict: Vec<String>,
    dict_index: Vec<Vec<usize>>,
}

impl AhoCorasickBuilder {
    /// [`AhoCorasickBuilder`]を生成する。
    pub fn new() -> Self {
        Self {
            size: 1,
            trie: vec![HashMap::new()],
            failure_edge: vec![0],
            dict: vec![],
            dict_index: vec![],
        }
    }

    /// パターン`pat`を追加する。
    pub fn add(&mut self, pat: &str) {
        self.dict.push(pat.to_owned());

        let mut cur = 0;

        for c in pat.chars() {
            if let Some(&index) = self.trie[cur].get(&c) {
                cur = index;
            } else {
                self.trie.push(HashMap::new());
                self.trie[cur].insert(c, self.size);
                cur = self.size;

                self.size += 1;
            }
        }

        self.dict_index.resize(self.size, vec![]);
        self.dict_index[cur].push(self.dict.len() - 1);
    }

    /// [`AhoCorasick`]を構築する。
    pub fn build(mut self) -> AhoCorasick {
        self.failure_edge.resize(self.size, 0);

        let mut dq = VecDeque::new();
        dq.push_back(0);

        while let Some(cur) = dq.pop_front() {
            for (&c, &next) in &self.trie[cur] {
                if cur == 0 {
                    self.failure_edge[next] = 0;
                } else {
                    let mut i = self.failure_edge[cur];
                    let mut j = 0;

                    loop {
                        if let Some(&index) = self.trie[i].get(&c) {
                            j = index;
                            break;
                        } else {
                            if i == 0 {
                                break;
                            }
                            i = self.failure_edge[i];
                        }
                    }

                    self.failure_edge[next] = j;

                    let temp = self.dict_index[j].clone();
                    self.dict_index[next].extend(temp);
                }

                dq.push_back(next);
            }
        }

        AhoCorasick {
            trie: self.trie,
            failure_edge: self.failure_edge,
            dict: self.dict,
            dict_index: self.dict_index,
        }
    }
}

pub struct AhoCorasick {
    trie: Vec<HashMap<char, usize>>,
    failure_edge: Vec<usize>,
    dict: Vec<String>,
    dict_index: Vec<Vec<usize>>,
}

impl AhoCorasick {
    /// 文字列`s`がマッチするすべてのパターンを列挙する。
    pub fn matches<F>(&self, s: &str, mut proc: F)
    where
        F: FnMut(usize, usize),
    {
        let mut cur = 0;

        for (i, c) in s.chars().enumerate() {
            while cur != 0 && !self.trie[cur].contains_key(&c) {
                cur = self.failure_edge[cur];
            }

            cur = self.trie[cur].get(&c).copied().unwrap_or(0);

            for &j in &self.dict_index[cur] {
                let len = self.dict[j].len();
                proc(i + 1 - len, len);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut builder = AhoCorasickBuilder::new();

        builder.add("ur");
        builder.add("et");

        let ac = builder.build();

        let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        ac.matches(s, |from, len| {
            let to = from + len;
            println!(
                "{}\x1b[m\x1b[1m\x1b[32m{}\x1b[m{}",
                s.get(from.saturating_sub(3)..from).unwrap(),
                s.get(from..to).unwrap(),
                s.get(to..to.saturating_add(3)).unwrap()
            );
        })
    }
}
