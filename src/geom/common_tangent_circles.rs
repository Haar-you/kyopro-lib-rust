//! 2つの円の共通接線

use crate::geom::*;

/// 2つの円の共通接線を求める
pub fn common_tangent_circles(a: Circle, b: Circle, eps: Eps) -> Vec<Vector> {
    let cc = b.center - a.center;
    let d = cc.abs();
    let n = cc.unit();
    let r_diff = a.radius - b.radius;
    let r_sum = a.radius + b.radius;

    if eps.eq(r_sum, d) {
        let t = cc.normal() * r_diff.mul_add(-r_diff, cc.abs_sq()).sqrt();
        vec![
            a.center + (cc * r_diff + t) * a.radius / cc.abs_sq(),
            a.center + (cc * r_diff - t) * a.radius / cc.abs_sq(),
            a.center + n * a.radius,
        ]
    } else if eps.lt(r_sum, d) {
        let t_diff = cc.normal() * r_diff.mul_add(-r_diff, cc.abs_sq()).sqrt();
        let t_sum = cc.normal() * r_sum.mul_add(-r_sum, cc.abs_sq()).sqrt();

        vec![
            a.center + (cc * r_diff + t_diff) * a.radius / cc.abs_sq(),
            a.center + (cc * r_diff - t_diff) * a.radius / cc.abs_sq(),
            a.center + (cc * r_sum + t_sum) * a.radius / cc.abs_sq(),
            a.center + (cc * r_sum - t_sum) * a.radius / cc.abs_sq(),
        ]
    } else if eps.gt(r_sum, d) && eps.gt(d, (a.radius - b.radius).abs()) {
        let t = cc.normal() * r_diff.mul_add(-r_diff, cc.abs_sq()).sqrt();
        vec![
            a.center + (cc * r_diff + t) * a.radius / cc.abs_sq(),
            a.center + (cc * r_diff - t) * a.radius / cc.abs_sq(),
        ]
    } else if eps.eq((a.radius - b.radius).abs(), d) {
        vec![if eps.gt(a.radius, b.radius) {
            a.center + n * a.radius
        } else {
            b.center - n * b.radius
        }]
    } else {
        vec![]
    }
}
