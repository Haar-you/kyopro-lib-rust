//! 黄金分割探索

/// [`golden_search`]で与えられる関数が上に凸か下に凸かを指定する。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Convex {
    /// 上に凸
    Upwards,
    /// 下に凸
    Downwards,
}

const PHI: f64 = 1.6180339887498949_f64;

/// 黄金分割探索
pub fn golden_search<F: Fn(f64) -> f64>(
    mut lb: f64,
    mut ub: f64,
    convex: Convex,
    mut loop_count: usize,
    f: F,
) -> f64 {
    while loop_count > 0 {
        let t1 = (lb * PHI + ub) / (PHI + 1.0);
        let t2 = (lb + ub * PHI) / (PHI + 1.0);

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
