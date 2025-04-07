//! 括弧列の構文解析

use std::iter::Peekable;

/// 括弧列のパースの結果
#[derive(Clone, Debug)]
pub struct ParseResult<T> {
    /// 複数の[`Elem`]からなる列
    pub elems: Vec<Elem<T>>,
}

/// ‍対応のある括弧列、または括弧以外の値を一つもつ列挙型
#[derive(Clone, Debug)]
pub enum Elem<T> {
    /// 括弧以外
    Value(T),
    /// 対応のある括弧列
    Paren {
        /// 開き括弧
        open: T,
        /// 括弧の内部のパースの結果
        inner: ParseResult<T>,
        /// 閉じ括弧
        close: T,
    },
}

fn _parse<T: Copy + Eq + std::fmt::Debug>(
    s: &mut Peekable<impl Iterator<Item = T>>,
    open: T,
    close: T,
) -> Option<ParseResult<T>> {
    let mut elems = vec![];

    loop {
        let c = s.peek();

        match c {
            None => break,
            Some(&c) => {
                if c == open {
                    s.next();
                    let inner = _parse(s, open, close)?;

                    let c = s.peek()?;
                    assert_eq!(c, &close);
                    elems.push(Elem::Paren { open, inner, close });
                    s.next();
                } else if c == close {
                    break;
                } else {
                    elems.push(Elem::Value(c));
                    s.next();
                }
            }
        }
    }

    Some(ParseResult { elems })
}

/// 括弧列をパースする。
pub fn parse_paren<T: Copy + Eq + std::fmt::Debug>(
    s: impl IntoIterator<Item = T>,
    open: T,
    close: T,
) -> Option<ParseResult<T>> {
    assert_ne!(open, close);

    let mut s = s.into_iter().peekable();
    let res = _parse(&mut s, open, close);

    if s.peek().is_none() {
        res
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str) -> bool {
        let a = parse_paren(s.chars(), '(', ')');
        a.is_some()
    }

    #[test]
    fn test() {
        assert!(check("()"));
        assert!(check("()()()"));
        assert!(check("(()(()))"));

        assert!(!check(")("));
        assert!(!check("(()"));
        assert!(!check("())"));
        assert!(!check("(()()"));

        assert!(check("a(bc(d)()e((f)g)h)i"));

        assert!(!check("(()()aaa"));
    }
}
