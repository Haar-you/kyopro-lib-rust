//! Knuth-Morris-Pratt法

#[derive(Clone, Debug)]
pub struct KMP<T> {
    pat: Vec<Option<T>>,
    table: Vec<isize>,
}

impl<T: PartialEq + Clone> KMP<T> {
    /// **Time complexity O(|pat|)**
    pub fn new(pat: Vec<T>) -> Self {
        let m = pat.len();
        let mut table: Vec<isize> = vec![0; m + 1];
        table[0] = -1;

        let mut pat: Vec<_> = pat.into_iter().map(|a| Some(a)).collect();
        pat.push(None);

        let mut i: usize = 2;
        let mut j: usize = 0;
        while i <= m {
            if pat[i - 1] == pat[j] {
                table[i] = (j + 1) as isize;
                i += 1;
                j += 1;
            } else if j > 0 {
                j = table[j] as usize;
            } else {
                table[i] = 0;
                i += 1;
            }
        }

        Self { pat, table }
    }

    /// **Time complexity O(|s|)**
    pub fn matches(&self, s: &[T]) -> Vec<usize> {
        let mut ret = vec![];
        let n = s.len();

        let mut m = 0;
        let mut i = 0;
        while m + i < n {
            if self.pat[i].as_ref() == Some(&s[m + i]) {
                i += 1;
                if i == self.pat.len() - 1 {
                    ret.push(m);
                    m += (i as isize - self.table[i]) as usize;
                    if i > 0 {
                        i = self.table[i] as usize;
                    }
                }
            } else {
                m += (i as isize - self.table[i]) as usize;
                if i > 0 {
                    i = self.table[i] as usize;
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aaa", "aaaaaaaa")]
    #[test_case("ab", "abracadabra")]
    fn test(pat: &str, s: &str) {
        let kmp = KMP::new(pat.as_bytes().to_owned());
        let indices = kmp.matches(s.as_bytes());

        assert_eq!(indices, bruteforce(pat, s));
    }

    fn bruteforce(pat: &str, s: &str) -> Vec<usize> {
        let mut ret = vec![];
        for i in 0..=s.len() - pat.len() {
            if &s[i..i + pat.len()] == pat {
                ret.push(i);
            }
        }
        ret
    }
}
