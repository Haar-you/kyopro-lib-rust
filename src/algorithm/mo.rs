pub struct Mo<'a> {
    append_right: Box<dyn 'a + Fn(usize)>,
    append_left: Box<dyn 'a + Fn(usize)>,
    remove_right: Box<dyn 'a + Fn(usize)>,
    remove_left: Box<dyn 'a + Fn(usize)>,
    query: Box<dyn 'a + Fn(usize)>,
    q: usize,
    index: usize,
    width: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    ord: Vec<usize>,
}

impl<'a> Mo<'a> {
    pub fn new<F1, F2, F3, F4, F5>(
        n: usize,
        q: usize,
        append_right: Box<F1>,
        append_left: Box<F2>,
        remove_right: Box<F3>,
        remove_left: Box<F4>,
        query: Box<F5>,
    ) -> Self
    where
        F1: 'a + Fn(usize),
        F2: 'a + Fn(usize),
        F3: 'a + Fn(usize),
        F4: 'a + Fn(usize),
        F5: 'a + Fn(usize),
    {
        Self {
            append_right,
            append_left,
            remove_right,
            remove_left,
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

        let mut q = 0;
        let mut l = left[self.ord[0]];
        let mut r = left[self.ord[0]];

        for _ in 0..self.q {
            let id = self.ord[q];
            q += 1;

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
