use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MultiSet<T> {
    map: BTreeMap<T, usize>,
    size: usize,
}

impl<T: Ord + Eq + Clone> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_default() += 1;
        self.size += 1;
    }

    pub fn remove(&mut self, value: T) -> bool {
        if let Some(count) = self.map.get_mut(&value) {
            *count -= 1;
            self.size -= 1;

            if *count == 0 {
                self.map.remove(&value);
            }

            true
        } else {
            false
        }
    }

    pub fn first(&self) -> Option<T> {
        self.map.iter().next().map(|(k, _)| k.clone())
    }

    pub fn last(&self) -> Option<T> {
        self.map.iter().next_back().map(|(k, _)| k.clone())
    }

    pub fn pop_last(&mut self) -> Option<T> {
        if let Some((k, v)) = self.map.iter_mut().next_back() {
            *v -= 1;
            self.size -= 1;

            let k = k.clone();

            if *v == 0 {
                self.map.remove(&k);
            }

            Some(k)
        } else {
            None
        }
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if let Some((k, v)) = self.map.iter_mut().next() {
            *v -= 1;
            self.size -= 1;

            let k = k.clone();

            if *v == 0 {
                self.map.remove(&k);
            }

            Some(k)
        } else {
            None
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn count(&self, value: &T) -> usize {
        self.map.get(&value).cloned().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
