use crate::chmax;

/// ヒストグラム中の最大面積長方形の面積を計算する。
///
/// Time complexity O(|h|)
pub fn max_rect_in_histogram<T>(h: &[T]) -> T
where
    T: From<usize> + std::ops::Mul<Output = T> + Ord + Copy
{
    let mut st: Vec<(T, usize)> = Vec::new();
    let mut ret = T::from(0);

    for (i, &y1) in h.iter().enumerate() {
        if let Some(&(y2, _)) = st.last() {
            if y2 < y1 {
                st.push((y1, i));
            }
            else if y2 > y1 {
                let mut j = i;
                while let Some(&(y3, k)) = st.last() {
                    if y3 <= y1 {
                        break;
                    }
                    chmax!(ret, y3 * T::from(i - k));
                    j = k;
                    st.pop();
                }
                st.push((y1, j));
            }
        }
        else {
            st.push((y1, i));
        }
    }

    while let Some((y, i)) = st.pop() {
        chmax!(ret, y * T::from(h.len() - i));
    }

    ret
}


/// グリッド上の最大面積長方形の面積を計算する。
///
/// Time complexity O(hw)
pub fn max_rect<T: Copy + PartialEq>(d: &[Vec<T>], value: T) -> usize {
    let h = d.len();
    let w = d[0].len();

    let mut c = vec![vec![0; w]; h];
    for i in 0 .. h {
        for j in 0 .. w {
            if d[i][j] == value {
                c[i][j] = 1;
            }
        }
    }

    for i in 1 .. h {
        for j in 0 .. w {
            if c[i][j] == 1 {
                c[i][j] += c[i - 1][j];
            }
        }
    }

    c.into_iter().map(|s| max_rect_in_histogram(&s)).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_3_C
        assert_eq!(max_rect_in_histogram(&[2, 1, 3, 5, 3, 4, 2, 1]), 12);
        assert_eq!(max_rect_in_histogram(&[2, 0, 1]), 2);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_3_B
        assert_eq!(max_rect(&[
            vec![0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 0]
        ], 0), 6);
    }
}
