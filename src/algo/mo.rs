//! Mo's algorithm
pub struct Mo<'a> {
    append_left: Box<dyn 'a + Fn(usize)>,
    append_right: Box<dyn 'a + Fn(usize)>,
    remove_left: Box<dyn 'a + Fn(usize)>,
    remove_right: Box<dyn 'a + Fn(usize)>,
    query: Box<dyn 'a + Fn(usize)>,
    q: usize,
    index: usize,
    width: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    ord: Vec<usize>,
}

impl<'a> Mo<'a> {
    pub fn new(
        n: usize,
        q: usize,
        append_left: Box<impl 'a + Fn(usize)>,
        append_right: Box<impl 'a + Fn(usize)>,
        remove_left: Box<impl 'a + Fn(usize)>,
        remove_right: Box<impl 'a + Fn(usize)>,
        query: Box<impl 'a + Fn(usize)>,
    ) -> Self {
        Self {
            append_left,
            append_right,
            remove_left,
            remove_right,
            query,
            q,
            index: 0,
            width: (n as f64).sqrt() as usize,
            left: vec![0; q],
            right: vec![0; q],
            ord: vec![0; q],
        }
    }

    pub fn add(&mut self, l: usize, r: usize) {
        self.left[self.index] = l;
        self.right[self.index] = r;
        self.ord[self.index] = self.index;
        self.index += 1;
    }

    pub fn run(mut self) {
        let left = self.left;
        let right = self.right;
        let width = self.width;

        self.ord.sort_by(|&i, &j| {
            let a = left[i] / width;
            let b = left[j] / width;

            if a == b {
                if a % 2 == 1 {
                    right[i].cmp(&right[j])
                } else {
                    right[j].cmp(&right[i])
                }
            } else {
                a.cmp(&b)
            }
        });

        let mut l = left[self.ord[0]];
        let mut r = left[self.ord[0]];

        for q in 0..self.q {
            let id = self.ord[q];

            while l != left[id] || r != right[id] {
                if l > left[id] {
                    l -= 1;
                    (self.append_left)(l);
                }
                if l < left[id] {
                    (self.remove_left)(l);
                    l += 1;
                }
                if r < right[id] {
                    (self.append_right)(r);
                    r += 1;
                }
                if r > right[id] {
                    r -= 1;
                    (self.remove_right)(r);
                }
            }

            (self.query)(id);
        }
    }
}
