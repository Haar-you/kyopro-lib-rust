//! 点を通る円の接線

use crate::geom::*;

/// 点`p`を通る円`c`の接線を求める
pub fn tangent_circle(c: Circle, p: Vector, eps: Eps) -> Vec<Vector> {
    let d = (p - c.center).abs();

    if eps.lt(d, c.radius) {
        return vec![];
    }
    if eps.eq(d, c.radius) {
        return vec![p];
    }

    let a = (c.radius / d).acos();
    let t = (p.1 - c.center.1).atan2(p.0 - c.center.0);

    vec![
        c.center + Vector::polar(c.radius, t + a),
        c.center + Vector::polar(c.radius, t - a),
    ]
}
