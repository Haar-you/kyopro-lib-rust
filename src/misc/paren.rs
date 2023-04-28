/// 括弧列が対応が取れているかを調べる
///
/// # Complexity
/// Time Complexity $O(N)$

pub fn check_paren<T: Copy + Eq>(s: impl IntoIterator<Item = T>, open: T, close: T) -> bool {
    let mut stack = vec![];

    for c in s {
        if stack.last().is_some() {
            if c == close {
                stack.pop();
            } else if c == open {
                stack.push(c);
            } else {
                return false;
            }
        } else if c == open {
            stack.push(c);
        } else {
            return false;
        }
    }

    stack.is_empty()
}
