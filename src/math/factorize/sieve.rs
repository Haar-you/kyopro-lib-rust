pub struct FactorizeSieve {
    p: Vec<usize>,
}

impl FactorizeSieve {
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];

        for i in 2..=n {
            if p[i] == 0 {
                for j in (i..=n).step_by(i) {
                    if p[j] == 0 {
                        p[j] = i;
                    }
                }
            }
        }

        Self { p }
    }

    pub fn factorize(&self, mut n: usize) -> Vec<usize> {
        let mut ret = vec![];

        while n > 1 {
            ret.push(self.p[n]);
            n /= self.p[n];
        }

        ret
    }
}
