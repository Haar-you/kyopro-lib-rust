#[derive(Clone, Debug, Default)]
pub struct IntervalHeap<T> {
    data: Vec<T>,
}

#[inline]
fn left(i: usize) -> usize {
    (i + 1) * 2 - (i & 1)
}

#[inline]
fn right(i: usize) -> usize {
    left(i) + 2
}

#[inline]
fn min(i: usize) -> usize {
    i | 1
}

#[inline]
fn max(i: usize) -> usize {
    i & !1
}

#[inline]
fn parent(i: usize) -> usize {
    (((i - 2) >> 2) << 1) | (i & 1)
}

impl<T: Ord> IntervalHeap<T> {
    #[inline]
    fn min_index(&self) -> usize {
        self.data.len().saturating_sub(1).min(1)
    }

    #[inline]
    fn max_index(&self) -> usize {
        0
    }

    fn top_down(&mut self, mut k: usize) -> usize {
        let n = self.data.len();

        if (k & 1) == 1 {
            while left(k) < n {
                let c = if n <= right(k) || self.data[left(k)] < self.data[right(k)] {
                    left(k)
                } else {
                    right(k)
                };

                if c < n && self.data[c] < self.data[k] {
                    self.data.swap(c, k);
                    k = c;
                } else {
                    break;
                }
            }
        } else {
            while left(k) < n {
                let c = if n <= right(k) || self.data[left(k)] > self.data[right(k)] {
                    left(k)
                } else {
                    right(k)
                };

                if c < n && self.data[c] > self.data[k] {
                    self.data.swap(c, k);
                    k = c;
                } else {
                    break;
                }
            }
        }

        k
    }

    fn bottom_up(&mut self, mut k: usize) {
        if min(k) < self.data.len() && self.data[max(k)] < self.data[min(k)] {
            self.data.swap(max(k), min(k));
            k ^= 1;
        }

        let root = 1;
        while root < k {
            let p = max(parent(k));
            if self.data[k] <= self.data[p] {
                break;
            }
            self.data.swap(p, k);
            k = p;
        }

        while root < k {
            let p = min(parent(k));
            if self.data[k] >= self.data[p] {
                break;
            }
            self.data.swap(p, k);
            k = p;
        }
    }

    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn min(&self) -> Option<&T> {
        self.data.get(self.min_index())
    }

    pub fn max(&self) -> Option<&T> {
        self.data.get(self.max_index())
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.bottom_up(self.data.len() - 1);
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let x = self.data.swap_remove(self.min_index());
            let k = self.top_down(1);
            self.bottom_up(k);
            Some(x)
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let x = self.data.swap_remove(self.max_index());
            let k = self.top_down(0);
            self.bottom_up(k);
            Some(x)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
