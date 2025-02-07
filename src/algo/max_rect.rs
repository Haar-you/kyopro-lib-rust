use crate::chmax;
use std::{
    cmp::Ordering,
    ops::{Mul, Range},
};

/// ヒストグラム中の最大面積長方形の面積を計算する。
///
/// **Time complexity** $O(|h|)$
pub fn max_rect_in_histogram<T>(h: &[T]) -> (T, Range<usize>)
where
    T: From<usize> + Mul<Output = T> + Ord + Copy,
{
    let mut st: Vec<(T, usize)> = vec![];
    let mut ret = T::from(0);
    let mut lr = (0, 0);

    for (i, &y1) in h.iter().enumerate() {
        if let Some(&(y2, _)) = st.last() {
            match y2.cmp(&y1) {
                Ordering::Less => {
                    st.push((y1, i));
                }
                Ordering::Greater => {
                    let mut j = i;
                    while let Some(&(y3, k)) = st.last() {
                        if y3 <= y1 {
                            break;
                        }
                        if chmax!(ret, y3 * T::from(i - k)) {
                            lr = (k, i);
                        }
                        j = k;
                        st.pop();
                    }
                    st.push((y1, j));
                }
                _ => {}
            };
        } else {
            st.push((y1, i));
        }
    }

    while let Some((y, i)) = st.pop() {
        if chmax!(ret, y * T::from(h.len() - i)) {
            lr = (i, h.len());
        }
    }

    (
        ret,
        Range {
            start: lr.0,
            end: lr.1,
        },
    )
}

/// グリッド上の最大面積長方形の面積を計算する。
///
/// **Time complexity** $O(hw)$
pub fn max_rect<T: Copy + PartialEq>(d: &[Vec<T>], value: T) -> usize {
    let h = d.len();
    let w = d[0].len();

    let mut c = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if d[i][j] == value {
                c[i][j] = 1;
            }
        }
    }

    for i in 1..h {
        for j in 0..w {
            if c[i][j] == 1 {
                c[i][j] += c[i - 1][j];
            }
        }
    }

    c.into_iter()
        .map(|s| max_rect_in_histogram(&s).0)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_3_C

        let a = [2, 1, 3, 5, 3, 4, 2, 1];
        let (ans, Range { start: l, end: r }) = max_rect_in_histogram(&a);
        assert_eq!(ans, 12);
        assert_eq!(ans, a[l..r].into_iter().min().unwrap() * (r - l));

        let a = [2, 0, 1];
        let (ans, Range { start: l, end: r }) = max_rect_in_histogram(&a);
        assert_eq!(ans, 2);
        assert_eq!(ans, a[l..r].into_iter().min().unwrap() * (r - l));
    }

    #[test]
    fn test_max_rect() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_3_B
        assert_eq!(
            max_rect(
                &[
                    vec![0, 0, 1, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![0, 0, 0, 1, 0],
                    vec![0, 0, 0, 1, 0]
                ],
                0
            ),
            6
        );
    }
}
