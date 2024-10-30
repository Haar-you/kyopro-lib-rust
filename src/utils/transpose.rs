/// タプルの`Vec`を`Vec`のタプルに変換する。
pub trait Transpose {
    type Output;
    fn transpose(self) -> Self::Output;
}

macro_rules! impl_transpose {
    ($($t:tt),+; $($index:tt),+) => {
        impl<$($t),+> Transpose for Vec<($($t),+)> {
            type Output = ($(Vec<$t>),+);
            fn transpose(self) -> Self::Output {
                let mut ret = ($(Vec::<$t>::new()),+);

                for x in self {
                    $(
                        ret.$index.push(x.$index);
                    )+
                }

                ret
            }
        }
    };
}

impl_transpose!(T0, T1; 0, 1);
impl_transpose!(T0, T1, T2; 0, 1, 2);
impl_transpose!(T0, T1, T2, T3; 0, 1, 2, 3);
impl_transpose!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![(1, "b", 0.4), (2, "aa", 0.3), (3, "ccc", -0.2)];

        assert_eq!(
            a.transpose(),
            (vec![1, 2, 3], vec!["b", "aa", "ccc"], vec![0.4, 0.3, -0.2])
        );
    }
}
