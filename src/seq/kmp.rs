#[derive(Clone, Debug)]
pub struct KMP {
    pat: Vec<u8>,
    table: Vec<isize>
}

impl KMP {
    pub fn new(pat: &str) -> Self {
        let m = pat.len();
        let mut table: Vec<isize> = vec![0; m + 1];
        table[0] = -1;

        let mut pat: Vec<u8> = pat.as_bytes().to_vec();
        pat.push(0);

        let mut i: usize = 2;
        let mut j: usize = 0;
        while i <= m {
            if pat[i - 1] == pat[j] {
                table[i] = (j + 1) as isize;
                i += 1;
                j += 1;
            }
            else if j > 0 {
                j = table[j] as usize;
            }
            else {
                table[i] = 0;
                i += 1;
            }
        }

        Self { pat, table }
    }

    pub fn matches(&self, s: &str) -> Vec<usize> {
        let mut ret = vec![];
        let n = s.len();

        let s = s.as_bytes().to_vec();

        let mut m = 0;
        let mut i = 0;
        while m + i < n {
            if self.pat[i] == s[m + i] {
                i += 1;
                if i == self.pat.len() - 1 {
                    ret.push(m);
                    m += (i as isize - self.table[i]) as usize;
                    if i > 0 {
                        i = self.table[i] as usize;
                    }
                }
            }
            else {
                m += (i as isize - self.table[i]) as usize;
                if i > 0 {
                    i = self.table[i] as usize;
                }
            }

        }

        ret
    }
}
