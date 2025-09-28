//! 三分探索

/// [`ternary_search`]で与えられる関数が上に凸か下に凸かを指定する。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Convex {
    /// 上に凸
    Upwards,
    /// 下に凸
    Downwards,
}

/// 三分探索
pub fn ternary_search<F: Fn(f64) -> f64>(
    mut lb: f64,
    mut ub: f64,
    convex: Convex,
    mut loop_count: usize,
    f: F,
) -> f64 {
    while loop_count > 0 {
        let t1 = lb + (ub - lb) / 3.0;
        let t2 = lb + (ub - lb) / 3.0 * 2.0;

        if (matches!(convex, Convex::Upwards) && f(t1) > f(t2))
            || (matches!(convex, Convex::Downwards) && f(t1) < f(t2))
        {
            ub = t2;
        } else {
            lb = t1;
        }

        loop_count -= 1;
    }

    lb
}
